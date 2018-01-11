import Promise from 'bluebird'

// Fetch from the Roll API
function callApi (command = '1d20') {
  return new Promise(async (resolve, reject) => {
    try {
      const request = await fetch(`https://roll.poweredvtt.com/v1/${command}`)
      if (request.ok) {
        const json = await request.json()
        resolve(json)
      } else {
        reject(new Error('Unexpected response body'))
      }
    } catch (error) {
      reject(error)
    }
  })
}

const executor = {}

const Executor = (options) => {
  // The selected target
  executor._target = undefined
  // Inline macros should be run as the parent token
  executor._runAs = undefined
  // The api to use for roll commands
  executor.api = options.api || callApi
  // A method to hook into 'input'
  executor.input = options.input || undefined
  // A dictionary of local macros
  executor.macros = {}
  // A dictionary of tokens
  executor.tokens = {}
  // Parse method for inline macros
  executor.parse = options.parse || undefined
  // A method to hook into 'prompt'
  executor.prompt = options.prompt || undefined
  // A dictionary of Variable
  executor.results = {}
  // A method to hook into 'target'
  executor.target = options.target || undefined
  // A dictionary of VariableReserved
  executor.variables = {}
  
  return executor
}

// Get an argument value from a number, token, variable, etc
export function getArgValue (value = {}) {
  if (typeof value.Number === 'number') { 
    return value.Number
  } else if (typeof value.Float === 'number') {
    return value.Float
  } else if (typeof value.Text === 'string') {
    return value.Text
  } else if (typeof value.Token === 'object') {
    // We can only read data from an attribute right now
    const attribute = value.Token.attribute
    let name = value.Token.name
    // Find the actual token id if it's @target
    if (name === 'target') {
      if (executor._target) {
        name = executor._target
      } else {
        throw new Error('No target selected')
      }
    }
    const token = executor.tokens[name]
    if (token) {
      const attr = token.attributes[attribute]
      if (attr) {
        return getArgValue(attr)
      }
      throw new Error(`Cannot find token attribute ${name}.${attribute}`)
    } else {
      throw new Error(`Cannot find token ${name}`)
    }
  } else if (typeof value.Variable === 'string') {
    if (executor.results[value.Variable]) {
      return executor.results[value.Variable]
    } else if (executor.variables[`${value.Variable}`]) {
      // bug! we sometimes parse Variable instead of VariableReserved?
      return executor.variables[`${value.Variable}`]
    }
  } else if (typeof value.VariableReserved === 'number') {
    if (executor.variables[`${value.VariableReserved}`]) {
      return executor.variables[`${value.VariableReserved}`]
    }
  }

  // If we can't get the value, return back the arg
  return value
}

// Execute all the steps
export function execute (steps = [], cleanup = true) {
  return new Promise(async (resolve, reject) => {
    // Create the output, we pass this around each function until we return at the end
    const output = Object.assign({}, {
      messages: [],
      rolls: [],

      _raw: [],
    })

    try {
      // Execute each step serially
      let exit = false
      await Promise.each(steps, (step) => {
        // If we encounter an exit, stop processing
        if (step.op === 'Exit') {
          exit = true
        }
        return !exit ? executeStep(step, output) : Promise.resolve()
      })

      // Return the final output
      resolve(output)

      // make sure we cleanup
      if (cleanup) {
        executor.variables = {}
        executor.results = {}
        executor._target = undefined
        executor._runAs = undefined
      }
    } catch (error) {
      reject(error)
    }
  })
}

// Execute a single step, pass along the output to modify
export function executeStep (step = {}, output) {
  switch (step.op) {
    case 'Input':
      return executeStepInput(step, output)
    case 'Lambda':
      return executeStepLambda(step, output)
    case 'Prompt':
      return executeStepPrompt(step, output)
    case 'Roll':
      return executeStepRoll(step, output)
    case 'Say':
      return executeStepSay(step, output)
    case 'Target':
      return executeStepTarget(step, output)
    case 'Whisper':
      return Promise.resolve() // todo
    default:
      return Promise.resolve()
  }
}

// Execute a lambda step
export async function executeStepLambda (step = {}, output) {
  return Promise.each(step.args, (arg) => {
    if (typeof arg.Conditional !== 'undefined') {
      const conditional = arg.Conditional 
      const comparison = conditional.comparison
      const left = getArgValue(conditional.left)
      const right = getArgValue(conditional.right)
      const success = conditional.success
      const failure = conditional.failure

      switch (comparison) {
        case 'EqualTo':
          return executeStep(left === right ? success : failure, output)
        case 'GreaterThanOrEqual':
          return executeStep(left >= right ? success : failure, output)
        case 'LessThanOrEqual':
          return executeStep(left <= right ? success : failure, output)
        case 'GreaterThan':
          return executeStep(left > right ? success : failure, output)
        case 'LessThan':
          return executeStep(left < right ? success : failure, output)
      }
    } else if (typeof arg.Assign !== 'undefined') {
      const assign = arg.Assign 
      const left = assign.left
      const right = assign.right.map(val => getArgValue(val))
      const isToken = typeof left.Token === 'object'
      const isVariable = typeof left.Variable === 'string'

      if (isVariable || isToken) {
        let nextOp // next operation to perform
        const result = right.reduce((a, v) => {
          if (typeof v === 'string') {
            nextOp = undefined
            return a === 0 ? v : `${a} ${v}`
          } else if (typeof v === 'number') {
            switch (nextOp) {
              case 'Add':
                nextOp = undefined
                return a + v
              case 'Divide':
                nextOp = undefined
                return a / v
              case 'Multiply':
                nextOp = undefined
                return a * v
              case 'Subtract':
                nextOp = undefined
                return a - v
              default:
                nextOp = undefined
                return v
            }
          } else if (v && typeof v.Primitive === 'string') {
            nextOp = v.Primitive
            return a
          }
        }, 0)

        if (isVariable) {
          executor.results[left.Variable] = result
        } else if (isToken) {
          const name = left.Token.name
          const attribute = left.Token.attribute
          let token = executor.tokens[name === 'target' ? executor._target : name]
          if (!token) {
            token = {
              attributes: {},
              macros: {},
            }
          }

          if (typeof result === 'number' && Number.isInteger(result)) {
            token.attributes[attribute] = { Number: result }
          } else if (typeof result === 'number') {
            token.attributes[attribute] = { Float: result }
          } else if (typeof result === 'string') {
            token.attributes[attribute] = { Text: result }
          } else if (typeof result === 'boolean') {
            token.attributes[attribute] = { Boolean: result }
          }

          // set the real token attribute
          executor.tokens[name === 'target' ? executor._target : name] = token
        }
      }
    } else if (typeof arg.Token === 'object') {
      // run an inline macro
      const name = arg.Token.name
      const macroName = arg.Token.macro_name
      if (executor.tokens[name === 'target' ? executor._target : name]) {
        const token = executor.tokens[name === 'target' ? executor._target : name]
        if (token.macros[macroName]) {
          const macro = getArgValue(token.macros[macroName])
          const inlineProgram = executor.parse(`#${macroName} ${macro}`)

          // Set run as
          executor._runAs = name

          let exit = false
          return Promise.each(inlineProgram.steps, (step) => {
            // If we encounter an exit, stop processing
            if (step.op === 'Exit') {
              exit = true
            }
            return !exit ? executeStep(step, output) : Promise.resolve()
          })
        }
      }
      return Promise.reject(new Error(`Token macro @${name}->${macroName} not found`))
    }

    return Promise.resolve()
  })
}

export function executeStepRoll (step = {}, output) {
  return new Promise(async (resolve, reject) => {
    // equation for the roll - this is a bit backwards right now because we parse the step out and
    // really should just be passing the raw input instead of reassembling the pieces
    // to feed into the Roll API
    let command = '' 
    let comment = null 
    step.args.forEach((arg) => {
      if (typeof arg.Roll !== 'undefined') {
        const rollArg = arg.Roll
        if (typeof rollArg.Advantage !== 'undefined') {
          command += 'adv'
        } else if (typeof rollArg.Disadvantage !== 'undefined') {
          command += 'dis'
        } else if (typeof rollArg.Comment !== 'undefined') {
          comment = getArgValue(rollArg.Comment)
        } else if (typeof rollArg.D !== 'undefined') {
          command += `d${getArgValue(rollArg.D)}`
        } else if (typeof rollArg.E !== 'undefined') {
          command += `e${getArgValue(rollArg.E)}`
        } else if (typeof rollArg.GT !== 'undefined') {
          command += `gt${getArgValue(rollArg.GT)}`
        } else if (typeof rollArg.GTE !== 'undefined') {
          command += `gte${getArgValue(rollArg.GTE)}`
        } else if (typeof rollArg.H !== 'undefined') {
          command += `kh${getArgValue(rollArg.H)}`
        } else if (typeof rollArg.L !== 'undefined') {
          command += `kl${getArgValue(rollArg.L)}`
        } else if (typeof rollArg.LT !== 'undefined') {
          command += `lt${getArgValue(rollArg.LT)}`
        } else if (typeof rollArg.LTE !== 'undefined') {
          command += `lte${getArgValue(rollArg.LTE)}`
        } else if (typeof rollArg.Max !== 'undefined') {
          command += `max${getArgValue(rollArg.Max)}`
        } else if (typeof rollArg.Min !== 'undefined') {
          command += `min${getArgValue(rollArg.Min)}`
        } else if (typeof rollArg.ModifierNeg !== 'undefined') {
          command += `-${getArgValue(rollArg.ModifierNeg)}`
        } else if (typeof rollArg.ModifierPos !== 'undefined') {
          // make sure the value is actually positive
          const value = getArgValue(rollArg.ModifierPos)
          if (value >= 0) {
            command += `+${value}`
          } else {
            command += `${value}`
          }
        } else if (typeof rollArg.N !== 'undefined') {
          command += `${getArgValue(rollArg.N)}`
        } else if (typeof rollArg.RO !== 'undefined') {
          command += `ro${getArgValue(rollArg.RO)}`
        } else if (typeof rollArg.RR !== 'undefined') {
          command += `rr${getArgValue(rollArg.RR)}`
        } else if (typeof rollArg.Sides !== 'undefined') {
          command += `d[${rollArg.Sides.map(side => getArgValue(side)).join(',')}]`
        }
      }
    })

    try {
      const roll = await executor.api(command)
      const runAs = executor._runAs ? { token: executor._runAs } : {}
      const formattedRoll = Object.assign({}, roll, runAs, { comment })
      output.rolls.push(formattedRoll)
      output._raw.push(formattedRoll)

      if (step.result === 'Save') {
        const length = Object.keys(executor.variables).length
        executor.variables[`${length + 1}`] = roll.value
      }
      resolve(roll)
    } catch (error) {
      reject(error)
    }
  })
}

export function executeStepSay (step = {}, output) {
  return new Promise(async (resolve, reject) => {
    const message = {
      from: null,
      to: null,
      message: '',
    }

    step.args.forEach((arg) => {
      if (typeof arg.Say === 'object') {
         if (typeof arg.Say.Message === 'string') {
           // if we have a message, add a space
          message.message += arg.Say.Message
        } else if (arg.Say.From) {
          message.from = arg.Say.From.name
        }
      } else if (typeof arg.Variable === 'string' || typeof arg.VariableReserved === 'number') {
        message.message += `${getArgValue(arg)}`
      }
    })

    if (message.from === null && executor._runAs) {
      message.from = executor._runAs
    }

    output.messages.push(message)
    output._raw.push(message)
    resolve()
  })
}

export function executeStepInput (step = {}, output) {
  return new Promise(async (resolve, reject) => {
    if (executor.input) {
      let message = ''
      step.args.forEach((arg) => {
        if (typeof arg.Input === 'string') {
          message = arg.Input
        }
      })

      try {
        const result = await executor.input(message)
        const length = Object.keys(executor.variables).length
        executor.variables[`${length + 1}`] = result

        resolve()
      } catch (error) {
        reject(new Error('Input encountered an error or was cancelled'))
      }
    } else {
      return reject(new Error('No input callback provided'))
    }
  })
}

export function executeStepPrompt (step = {}, output) {
  return new Promise(async (resolve, reject) => {
    if (executor.prompt) {
      let message = ''
      let options = {}
      step.args.forEach((arg) => {
        if (typeof arg.Prompt.message === 'string') {
          message = arg.Prompt.message
        } else if (typeof arg.Prompt.Variable === 'object') {
          message += getArgValue(arg.Prompt)
        }
        
        if (typeof arg.Prompt.options === 'object') {
          options = arg.Prompt.options
        }
      })

      try {
        const displayOptions = options.map((option, i) => ({
          key: option.key || i,
          value: toString(option.value),
        }))
        const result = await executor.prompt(message, displayOptions)
        
        const length = Object.keys(executor.variables).length
        executor.variables[`${length + 1}`] = options[result].key || getArgValue(options[result].value)

        resolve()
      } catch (error) {
        return reject(error)
      }
    } else {
      return reject(new Error('No prompt callback provided'))
    }
  })
}

export function executeStepTarget (step = {}, output) {
  return new Promise(async (resolve, reject) => {
    if (executor.target) {
      let message = 'Choose a target'
      step.args.forEach((arg) => {
        if (arg.Target.Message === 'string') {
          message = arg.Target.Message
        }
      })

      try {
        const id = await executor.target(message)
        executor._target = id
        resolve()
      } catch (error) {
        return reject(new Error('Target encountered an error or was cancelled'))
      }
    } else {
      reject(new Error('No target callback provided'))
    }
  })
}

function toString(arg) {
  if (typeof arg.Token === 'object') {
    const token = arg.Token
    const attribute = token.attribute ? `.${token.attribute}` : ''
    const macro = token.macro_name ? `->${token.macro_name}` : ''
    return `@${token.name}${attribute}${macro}`
  } else if (typeof arg.Text === 'string') {
    return arg.Text
  } else if (typeof arg.Number === 'number' || typeof arg.Float === 'number') {
    return arg.Number || arg.Float
  }

  return arg
}

export default Executor
