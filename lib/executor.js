import Promise from 'bluebird'
import { get, set } from 'lodash'

// Fetch from the Roll API
function callApi (command = '1d20') {
  return new Promise(async (resolve, reject) => {
    try {
      const request = await fetch(`https://roll.poweredvtt.com/v1/${encodeURIComponent(command)}`)
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

const TEMPLATE_OUTPUT = {
  type: 'unknown', // message|template|roll
  data: undefined,
}

const executor = {}

const Executor = (options) => {
  // The selected target
  executor._target = undefined
  // Inline macros should be run as the parent token
  executor._runAs = undefined
  // Test Mode (hide output from everyone)
  executor._test = false
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
  if (typeof value.Array === 'object') {
    return value.Array.map((e) => (getArgValue(e)))
  } else if (typeof value.Object === 'object') {
    const values = {}
    Object.keys(value.Object).forEach((key) => {
      values[key] = getArgValue(value.Object[key])
    })
    return values
  } else if (typeof value.Boolean === 'boolean') {
    return value.Boolean
  } else if (typeof value.Number === 'number') {
    return value.Number
  } else if (typeof value.Float === 'number') {
    return value.Float
  } else if (typeof value.Text === 'string') {
    return value.Text
  } else if (typeof value.TextInterpolated === 'object') {
    const values = value.TextInterpolated.parts.map((part) => (
      getArgValue(part)
    ))
    return values.join('')
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
      // Check if we're a deep nested attribute
      const isDeep = /\./.test(attribute)
      if (isDeep) {
        // traverse down the object to find the appropriate value
        const attrs = attribute.split('.')

        let ref = Array.isArray(token) ? token : token.attributes
        attrs.forEach((attr) => {
          // if we have a number, we're referencing an array
          const isNumber = !isNaN(attr)
          if (!isNumber) {
            ref = ref[attr]
          } else if (ref.Array) {
            ref = ref.Array[attr]
          } else {
            // if we're here, we've referenced a deeply nested token
            ref = ref[~~attr].attributes
          }
        })

        return getArgValue(ref)
      } else {
        const attr = token.attributes[attribute]
        if (attr) {
          return getArgValue(attr)
        }
      }
      throw new Error(`Cannot find token attribute @${name}.${attribute}`)
    } else {
      throw new Error(`Cannot find token @${name}`)
    }
  } else if (typeof value.Variable === 'string') {
    // This shouldn't happen but if we have a string '0' then pop the result
    if (value.Variable === '0') {
      // pop the last result (but not our current result)
      const variables = Object.values(executor.variables)
      return variables[variables.length - 1]
    }

    if (executor.results[value.Variable]) {
      return executor.results[value.Variable]
    } else if (executor.variables[`${value.Variable}`]) {
      // bug! we sometimes parse Variable instead of VariableReserved?
      return executor.variables[`${value.Variable}`]
    } else {
      throw new Error(`Variable $\{${value.Variable}} is not set and can not be used`)
    }
  } else if (typeof value.VariableReserved === 'number') {
    if (value.VariableReserved === 0) {
      // pop the last result (but not our current result)
      const variables = Object.values(executor.variables)
      return variables[variables.length - 1]
    } else if (executor.variables[`${value.VariableReserved}`]) {
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
      templates: [],
      _raw: [],
    })

    try {
      // Execute each step serially
      let exit = false
      await Promise.each(steps, (step) => {
        // If we encounter an exit, stop processing
        if (step.op === 'Exit') {
          exit = true
        } else if (step.op === 'TestMode') {
          executor._test = step.args[0] ? step.args[0].TestMode : !executor._test
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
    case 'Case':
    case 'Prompt':
      return executeStepPrompt(step, output)
    case 'Input':
      return executeStepInput(step, output)
    case 'Lambda':
      return executeStepLambda(step, output)
    case 'Roll':
    case 'RollHidden':
    case 'RollWhisper':
      return executeStepRoll(step, output)
    case 'Say':
    case 'Whisper':
      return executeStepSay(step, output)
    case 'Target':
      return executeStepTarget(step, output)
    case 'Template':
      return executeStepTemplate(step, output)
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
      const left = Number(getArgValue(conditional.left))
      const right = Number(getArgValue(conditional.right))
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
    } else if (arg.Assign || arg.Concat || arg.Deduct) {
      const assign = arg.Assign || arg.Concat || arg.Deduct
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
          } else if (typeof v === 'boolean') {
            nextOp = undefined
            return v
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
          } else if (v && typeof v.Boolean !== 'undefined') {
            return v.Boolean
          } else if (v && typeof v.Primitive === 'string') {
            nextOp = v.Primitive
            return a
          } else if (v && typeof v.Array !== 'undefined') {
            nextOp = undefined
            return v.Array.map(e => getArgValue(e))
          } else if (v && typeof v.Object !== 'undefined') {
            nextOp = undefined
            return v.Object
          } else {
            return v
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

          // if we're setting a deep nested attribute, set the appropriate value
          const ref = token.attributes
          const isDeep = /\./.test(attribute)
          const path = []
          if (isDeep) {
            // traverse down the object to find the appropriate value to set
            const attrs = attribute.split('.')

            attrs.forEach((attr) => {
              // if we have a number, we're referencing an array
              const isNumber = !isNaN(attr)
              if (!isNumber) {
                path.push(attr)
              } else {
                path.push(`Array[${~~attr}]`)
              }
            })
          } else {
            path.push(attribute)
          }

          // if we're concat'ing - get the value and append
          if (arg.Concat) {
            const currentVal = get(ref, path.join('.'))
            let newVal = currentVal
            if (currentVal.Array) {
              newVal.Array.push(buildToken(result))
            } else if (currentVal.Number) {
              newVal.Number += result
            } else if (currentVal.Float) {
              newVal.Float += result
            } else if (currentVal.Text) {
              newVal.Text += result
            }

            set(ref, path.join('.'), newVal)
          } else if (arg.Deduct) {
            const currentVal = get(ref, path.join('.'))
            let newVal = currentVal
            if (currentVal.Array) {
              newVal.Array.push(buildToken(result))
            } else if (currentVal.Number) {
              newVal.Number -= result
            } else if (currentVal.Float) {
              newVal.Float -= result
            } else if (currentVal.Text) {
              newVal.Text = newVal.Text.replace(new RegExp(`${result}$`), '')
            }

            set(ref, path.join('.'), newVal)
          } else {
            set(ref, path.join('.'), buildToken(result))
          }

          // set the real token attribute
          executor.tokens[name === 'target' ? executor._target : name] = token
        }
      }
    } else if (typeof arg.Token === 'object') {
      // run an inline macro
      const name = arg.Token.name === 'target' ? executor._target : arg.Token.name
      const macroName = arg.Token.macro_name
      if (executor.tokens[name]) {
        const token = executor.tokens[name]
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
    let to = null 
    let as = null
    step.args.forEach((arg) => {
      if (typeof arg.Roll !== 'undefined') {
        const rollArg = arg.Roll
        if (typeof rollArg.Advantage !== 'undefined') {
          command += 'adv'
        } else if (typeof rollArg.Disadvantage !== 'undefined') {
          command += 'dis'
        } else if (typeof rollArg.Comment !== 'undefined') {
          command += `['${getArgValue(rollArg.Comment)}']`
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
          const comparitive = rollArg.RO.op
          const value = getArgValue(rollArg.RO.value)

          let op = '<'
          switch (comparitive) {
            case 'EqualTo':
              op = '=='
              break
            case 'GreaterThanOrEqual':
              op = '>='
              break
            case 'LessThanOrEqual':
              op = '<='
              break
            case 'GreaterThan':
              op = '>'
              break
            case 'LessThan':
              op = '<'
              break
          }
          command += `ro${op}${value}`
        } else if (typeof rollArg.RR !== 'undefined') {
          const comparitive = rollArg.RR.op
          const value = getArgValue(rollArg.RR.value)

          let op = '<'
          switch (comparitive) {
            case 'EqualTo':
              op = '=='
              break
            case 'GreaterThanOrEqual':
              op = '>='
              break
            case 'LessThanOrEqual':
              op = '<='
              break
            case 'GreaterThan':
              op = '>'
              break
            case 'LessThan':
              op = '<'
              break
          }
          command += `rr${op}${value}`
        } else if (typeof rollArg.Sides !== 'undefined') {
          command += `d[${rollArg.Sides.map(side => getArgValue(side)).join(',')}]`
        } else if (typeof rollArg.Primitive !== 'undefined') {
          switch (rollArg.Primitive) {
            case 'Add': {
              command += ' + '
              break
            }
            case 'Subtract': {
              command += ' - '
              break
            }
          }
        }
      } else if (typeof arg.Token === 'object') {
        if (step.op === 'Roll') {
          as = arg
        } else {
          to = arg
        }
      }
    })

    // if we set the token equal to a token attribute, but that was it - it was probably a roll command
    if (command === '' && (as || to)) {
      command = getArgValue(as || to)
    } else if (to) {
      to = to.Token.name
    } else if (as) {
      as = as.Token.name
    }

    try {
      const apiResponse = await executor.api(command)
      const runAs = executor._runAs ? { token: executor._runAs } : {}
      const formattedRoll = Object.assign(
        {}, apiResponse.roll, runAs, { to, as, is_test: executor._test }
      )
      const result = Object.assign({}, TEMPLATE_OUTPUT, { type: 'roll', data: formattedRoll })
      output.rolls.push(formattedRoll)
      if (step.op !== 'RollHidden') {
        output._raw.push(result)
      }

      if (step.result === 'Save') {
        const length = Object.keys(executor.variables).length
        executor.variables[`${length + 1}`] = formattedRoll.value
      }
      resolve(result)
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
      is_test: executor._test,
    }

    try {
      step.args.forEach((arg) => {
        if (typeof arg.Say.Message === 'object') {
          message.message = getArgValue({ TextInterpolated: arg.Say.Message })
        } else if (typeof arg.Say.From === 'object') {
          message.from = arg.Say.From.name
        } else if (typeof arg.Say.To === 'object') {
          message.to = arg.Say.To.name
        }
      })

      if (message.from === null && executor._runAs) {
        message.from = executor._runAs
      }

      const result = Object.assign({}, TEMPLATE_OUTPUT, { type: 'message', data: message })
      output.messages.push(message)
      output._raw.push(result)
      resolve()
    } catch (error) {
      reject(error)
    }
  })
}

export function executeStepInput (step = {}, output) {
  return new Promise(async (resolve, reject) => {
    if (executor.input) {
      let message = ''
      step.args.forEach((arg) => {
        if (typeof arg.Input === 'object') {
          message = getArgValue({ TextInterpolated: arg.Input })
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
      let options = []
      let defaultValue
      const op = step.op
      step.args.forEach((arg) => {
        if (typeof arg[op].message === 'object') {
          message = getArgValue({ TextInterpolated: arg[op].message })
        }
        if (Array.isArray(arg[op].options)) {
          options = arg[op].options
        }

        if (typeof arg[op].input !== 'undefined') {
          defaultValue = getArgValue(arg.Case.input)
        }
      })

      try {
        let defaultResult
        const optionsIndex = []
        const displayOptions = options.map((option, i) => {
          const key = option.key || i
          const value = toString(option.value)
          optionsIndex.push(Object.assign({}, option, { key }))

          if (defaultValue !== undefined && defaultValue === key) {
            defaultResult = i
          }

          return {
            key,
            value,
          }
        })

        const result = displayOptions.length > 0 && defaultResult === undefined
          ? await executor.prompt(message, displayOptions)
          : defaultResult
        
        const length = Object.keys(executor.variables).length

        if (result !== undefined) {
          if (op === 'Prompt') {
            // if Prompt, return the key or value
            executor.variables[`${length + 1}`] = optionsIndex[result].key || getArgValue(optionsIndex[result].value)
          } else if (op === 'Case') {
            // otherwise just return the value
            executor.variables[`${length + 1}`] = getArgValue(options[result].value)
          }
        }

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

export function executeStepTemplate (step = {}, output) {
  return new Promise(async (resolve, reject) => {
    const template = {
      name: 'default',
      attributes: {},
    }

    try {
      step.args.forEach((arg) => {
        if (typeof arg.Template.Name === 'string') {
          template.name = arg.Template.Name
        } else if (arg.Template.Attributes && typeof arg.Template.Attributes.Object === 'object') {
          Object.keys(arg.Template.Attributes.Object).forEach((attr) => {
            template.attributes[attr] = getArgValue(arg.Template.Attributes.Object[attr])
          })
        }
      })

      const result = Object.assign({}, TEMPLATE_OUTPUT, { type: 'template', data: template })
      output.templates.push(template)
      output._raw.push(result)
      resolve()
    } catch (error) {
      reject(error)
    }
  })
}

function toString(arg) {
  if (typeof arg.Token === 'object') {
    const token = arg.Token
    const attribute = token.attribute ? `.${token.attribute}` : ''
    const macro = token.macro_name ? `->${token.macro_name}` : ''
    return `@${token.name}${attribute}${macro}`
  } else {
    return getArgValue(arg)
  }

  return arg
}


/**
 * Build raw attributes for a token
 * @param {Any} value; The attributes to build
 * @return {Object}
 */
export function buildToken (value) {
  let finalValue = value
  let type = 'Text' // find the right data type

  // We use this data-type naming convention partially for legacy purposes but in
  // case we also want to shove this data back into Rust, it's practically ready
  if (Array.isArray(value)) {
    type = 'Array'
    finalValue = value.map(v => buildToken(v))
    // @todo build the value
  } else if (typeof value === 'object') {
    type = 'Object'
    finalValue = {}
    Object.keys(value).forEach((attribute) => {
      const val = buildToken(value[attribute])
      finalValue[attribute] = val
    })
  } else if (!isNaN(parseFloat(value)) && isFinite(value)) {
    type = value % 1 === 0 ? 'Number' : 'Float'
    finalValue = Number(value)
  } else if (typeof value === 'boolean') {
    type = 'Boolean'
    finalValue = Boolean(value)
  }

  if (type === 'Object') {
    return finalValue
  } else {
    return { [type]: finalValue }
  }
}

/**
 * The opposite of buildToken, this deconstructs the data types
 * @param {Object} token; The attributes to deconstruct
 * @return {Object}
 */
export function unbuildToken (value) {
  let finalValue

  if (value) {
    if (value.Array !== undefined) {
      finalValue = value.Array.map(e => unbuildToken(e))
    } else if (value.Number !== undefined) {
      finalValue = value.Number
    } else if (value.Float !== undefined) {
      finalValue = value.Float
    } else if (value.Text !== undefined) {
      finalValue = value.Text
    } else if (value.Boolean !== undefined) {
      finalValue = value.Boolean
    } else {
      // regular object
      finalValue = {}
      Object.keys(value).forEach((attribute) => {
        finalValue[attribute] = unbuildToken(value[attribute])
      })
    }
  }

  return finalValue
}


export default Executor
