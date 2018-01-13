import test from 'ava'
import { TTML } from './index'
import sinon from 'sinon'
import Executor, {
  execute,
  Output,
} from './executor'

// test data
let conditionalStep
let executor
let mockPrompt
let mockRoll
let mockTarget
let program
let stepRoll
let stepPrompt
let stepTarget
let stepRolld8
let stepAssign
let token
let tokenMacros
let tokenMe
let tokenRaw

test.beforeEach((t) => {
  // Reset the mock functions
  mockPrompt = undefined
  mockRoll = undefined
  mockTarget = undefined

  const api = sinon.stub().returns(
    {'id':'5e5f9750-6e78-4108-8fcf-b74731e2d0a0','comment':null,'dice':[{'id':'60b2dfd5-2f05-4bb6-b2ee-0ef7435c0d33','child':null,'die':'D20','is_dropped':false,'is_rerolled':false,'is_successful':true,'max':20,'min':1,'sides':20,'timestamp':'2018-01-09T14:20:20.374915727Z','value':12}],'equation':'1d20','execution_time':0,'modifiers':[],'raw_value':12,'timestamp':'2018-01-09T14:20:20.374917631Z','value':12}
  )

  executor = Executor({
    api,
  })

  // Reset the steps
  stepRoll = {
    args: [
      { Roll: { N: { Number: 1 } } },
      { Roll: { D: { Number: 20 } } },
      {
        Roll: {
          Comment: {
            TextInterpolated: {
              parts: [
                { Text: 'I am a comment' },
              ],
            },
          },
        },
      },
    ],
    op: 'Roll',
    result: 'Save',
  }

  stepPrompt = {
    args: [
      {
        Prompt: {
          message: {
            parts: [
              { Text: 'Select your attack' },
            ],
          },
          options: [
            { key: null, value: { Token: { attribute: 'dexterity_mod', macro_name: null, name: 'me' } } },
            { key: null, value: { Token: { attribute: 'strength_mod', macro_name: null, name: 'me' } } },
            {
              key: 'asd',
              value: {
                TextInterpolated: {
                  parts: [
                    { Text: 'asd' },
                  ],
                },
              },
            },
          ],
        },
      },
    ],
    op: 'Prompt',
    result: 'Save',
  }

  stepTarget = {
    args: [{
      Target: {
        Message: {
          parts: [
            { Text: 'Choose your victim' },
          ],
        },
      },
    }],
    op: 'Target',
    result: 'Save',
  }

  stepRolld8 = {
    args: [
      { Roll: { N: { Number: 1 } } },
      { Roll: { D: { Number: 8 } } },
      { Roll: { ModifierPos: { VariableReserved: 1 } } },
    ],
    op: 'Roll',
    result: 'Save',
  }

  stepAssign = {
    args: [
      {
        Assign: {
          left: { Token: { attribute: 'hp', macro_name: null, name: 'target' } },
          right: [
            { Token: { attribute: 'hp', macro_name: null, name: 'target' } },
            { Primitive: 'Subtract' },
            { VariableReserved: 2 },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }

  conditionalStep = {
    args: [{
      Conditional: {
        comparison: 'EqualTo',
        failure: {
          args: [],
          op: 'Exit',
          result: 'Ignore',
        },
        left: { Number: 10 },
        right: { Number: 10 },
        success: stepRoll,
      },
    }],
    op: 'Lambda',
    result: 'Ignore',
  }

  // Reset the testing program
  program = {
    name: 'test-macro',
    steps: [
      stepRoll,
      stepPrompt,
      stepTarget,
      stepRolld8,
      stepAssign,
    ],
  }

  // Reset token
  token = {
    name: 'Tester',
    foo: 'bar',
    hp: 42,
    strength_mod: 11,
    dexterity_mod: 2,
    x: 1240.92,
    y: -1901.11,
    is_test: true,
  }

  tokenMe = {
    hp: 15,
  }

  tokenMacros = {
    'uncanny-dodge': '!r 1d20 "Uncanny dodge!"',
  }

  tokenRaw = {
    attributes: {
      name: { Text: 'Tester' },
      foo: { Text: 'bar' },
      hp: { Number: 42 },
      strength_mod: { Number: 11 },
      dexterity_mod: { Number: 2 },
      x: { Float: 1240.92 },
      y: { Float: -1901.11 },
      is_test: { Boolean: true },
    },
    macros: {
      'uncanny-dodge': { Text: '!r 1d20 "Uncanny dodge!"' },
    },
  }
})

test('.addInputAction adds a callback for input', (t) => {
  const addInputAction = TTML.addInputAction((i, o) => {})
  t.is(typeof addInputAction, 'function')
})

test('.addPromptAction adds a callback for prompt', (t) => {
  const addPromptAction = TTML.addPromptAction((i, o) => {})
  t.is(typeof addPromptAction, 'function')
})

test('.addTargetAction adds a callback for target', (t) => {
  const addTargetAction = TTML.addTargetAction((i) => {})
  t.is(typeof addTargetAction, 'function')
})

test('.getRawTokens gets all internal tokens', (t) => {
  const tokenA = Object.assign({}, tokenRaw, {
    attributes: Object.assign({}, tokenRaw.attributes, { id: 'a' }),
  })
  const tokenB = Object.assign({}, tokenRaw, {
    attributes: Object.assign({}, tokenRaw.attributes, { id: 'b' }),
  })
  const tokenC = Object.assign({}, tokenRaw, {
    attributes: Object.assign({}, tokenRaw.attributes, { id: 'b' }),
  })

  TTML.setRawToken('raw_token_name_a', tokenA)
  TTML.setRawToken('raw_token_name_b', tokenB)
  TTML.setRawToken('raw_token_name_c', tokenC)

  const getRawTokens = TTML.getRawTokens()
  t.deepEqual(getRawTokens, {
    raw_token_name_a: tokenA,
    raw_token_name_b: tokenB,
    raw_token_name_c: tokenC,
  }, 'unexpected raw tokens structure')

})

test('.setRawToken sets a token as-is', (t) => {
  const setRawToken = TTML.setRawToken('raw_token_name', tokenRaw)
  t.deepEqual(setRawToken, tokenRaw, 'unexpected raw token structure')
})

test('.setToken sets a regular JSON-object as a token', (t) => {
  const setToken = TTML.setToken('token_name', token, tokenMacros)
  t.deepEqual(setToken, tokenRaw, 'unexpected raw token structure')
})

test('it executes a == Conditional step', async (t) => {
  let output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 1, 'does not compare EqualTo')

  conditionalStep.args[0].Conditional.left = { Number: 9 }
  conditionalStep.args[0].Conditional.right = { Number: 10 }
  output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 0, 'does not compare EqualTo')
})

test('it executes a >= Conditional step', async (t) => {
  conditionalStep.args[0].Conditional.comparison = 'GreaterThanOrEqual' 
  conditionalStep.args[0].Conditional.left = { Float: 15.5 }
  conditionalStep.args[0].Conditional.right = { Number: 10 }
  let output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 1, 'does not compare GreaterThanOrEqual')

  output.rolls = []
  conditionalStep.args[0].Conditional.comparison = 'GreaterThanOrEqual' 
  conditionalStep.args[0].Conditional.left = { Number: 5 }
  conditionalStep.args[0].Conditional.right = { Number: 15 }
  output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 0, 'does not compare GreaterThanOrEqual')

  output.rolls = []
  conditionalStep.args[0].Conditional.comparison = 'GreaterThanOrEqual' 
  conditionalStep.args[0].Conditional.left = { Number: 15 }
  conditionalStep.args[0].Conditional.right = { Number: 15 }
  output = await execute([ conditionalStep ],false)
  t.is(output.rolls.length, 1, 'does not compare GreaterThanOrEqual')
})

test('it executes a <= Conditional step', async (t) => {
  conditionalStep.args[0].Conditional.comparison = 'LessThanOrEqual' 
  conditionalStep.args[0].Conditional.left = { Float: 5.25 }
  conditionalStep.args[0].Conditional.right = { Number: 6 }
  let output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 1, 'does not compare LessThanOrEqual')

  conditionalStep.args[0].Conditional.comparison = 'LessThanOrEqual' 
  conditionalStep.args[0].Conditional.left = { Number: 25 }
  conditionalStep.args[0].Conditional.right = { Number: 5 }
  output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 0, 'does not compare LessThanOrEqual')

  conditionalStep.args[0].Conditional.comparison = 'LessThanOrEqual' 
  conditionalStep.args[0].Conditional.left = { Number: 2 }
  conditionalStep.args[0].Conditional.right = { Number: 2 }
  output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 1, 'does not compare LessThanOrEqual')
})

test('it executes a > Conditional step', async (t) => {
  conditionalStep.args[0].Conditional.comparison = 'GreaterThan' 
  conditionalStep.args[0].Conditional.left = { Float: 15.5 }
  conditionalStep.args[0].Conditional.right = { Number: 10 }
  let output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 1, 'does not compare GreaterThan')

  conditionalStep.args[0].Conditional.comparison = 'GreaterThan' 
  conditionalStep.args[0].Conditional.left = { Number: 5 }
  conditionalStep.args[0].Conditional.right = { Number: 15 }
  output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 0, 'does not compare GreaterThan')

  conditionalStep.args[0].Conditional.comparison = 'GreaterThan' 
  conditionalStep.args[0].Conditional.left = { Number: 15 }
  conditionalStep.args[0].Conditional.right = { Number: 15 }
  output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 0, 'does not compare GreaterThan')
})

test('it executes a < Conditional step', async (t) => {
  conditionalStep.args[0].Conditional.comparison = 'LessThan' 
  conditionalStep.args[0].Conditional.left = { Float: 5.25 }
  conditionalStep.args[0].Conditional.right = { Number: 6 }
  let output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 1, 'does not compare LessThan')

  output.rolls = []
  conditionalStep.args[0].Conditional.comparison = 'LessThan' 
  conditionalStep.args[0].Conditional.left = { Number: 25 }
  conditionalStep.args[0].Conditional.right = { Number: 5 }
  output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 0, 'does not compare LessThan')

  output.rolls = []
  conditionalStep.args[0].Conditional.comparison = 'LessThan' 
  conditionalStep.args[0].Conditional.left = { Number: 2 }
  conditionalStep.args[0].Conditional.right = { Number: 2 }
  output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 0, 'does not compare LessThan')
})

test('it executes a Exit step', async (t) => {
  const exitStep = {
    args: [],
    op: 'Exit',
    result: 'Ignore',
  }

  const output = await execute([ exitStep, stepRoll ], false)
  t.is(output.rolls.length, 0)
})

test('it executes a Input step', async (t) => {
  executor.input = sinon.stub().resolves('42')
  const step = {
    args: [{
      Input: 'Type something',
    }],
    op: 'Input',
    result: 'Save',
  }
  const output = await execute([ step ], false)
  t.true(executor.input.calledOnce)
  t.true(executor.input.calledWith('Type something'))
  t.is(executor.variables['1'], '42')
})

test('it executes a Prompt step', async (t) => {
  TTML.setToken('me', token, {})
  executor.prompt = sinon.stub().resolves(1)
  const output = await execute([ stepPrompt ], false)
  t.is(executor.variables['1'], 11)
  t.true(executor.prompt.calledWith('Select your attack', [
    { key: 0, value: '@me.dexterity_mod' },
    { key: 1, value: '@me.strength_mod' },
    { key: 'asd', value: 'asd' },
  ]))
})

test('it executes a Roll step using the api', async (t) => {
  let output = await execute([ stepRoll ], false)
  t.true(executor.api.calledOnce)

  const roll = {
    args: [
      { Roll: { N: { Number: 1 } } },
      { Roll: { D: { Number: 8 } } },
      { Roll: { GT: { Number: 6 } } },
    ],
    op: 'Roll',
    result: 'Ignore',
  }
  output = await execute([ roll ], false)
  t.true(executor.api.calledWith('1d8gt6'))

  const roll2 = {
    args: [
      { Roll: { N: { Number: 5 } } },
      {
        Roll: {
          Sides: [
            { Number: 1 },
            { Number: 0 },
            { Number: 5 },
            { Number: 7 },
          ],
        },
      },
    ],
    op: 'Roll',
    result: 'Ignore',
  }
  output = await execute([ roll2 ], false)
  t.true(executor.api.calledWith('5d[1,0,5,7]'))
})

test('it executes a Say step', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Variable: 'foo' },
          right: [
            { Text: 'foovar' },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  const stepSay = {
    args: [
      {
        Say: {
          Message: {
            parts: [
              { Text: 'hello world ' },
              { Variable: 'foo' },
              { Text: ' more text' },
            ],
          },
        }
      },
      { Say: { From: { attribute: null, macro_name: null, name: 'me' } } },
    ],
    op: 'Say',
    result: 'Ignore',
  }

  const output = await execute([ assignStep, stepSay ], false)
  t.is(output.messages.length, 1)
  t.is(output.messages[0].from, 'me')
  t.is(output.messages[0].message, 'hello world foovar more text')
})

test('it executes a Target step', async (t) => {
  executor.target = sinon.stub().resolves('id')
  const output = await execute([ stepTarget ], false)
  t.is(executor._target, 'id')
})

test.skip('it executes a Whisper step', (t) => {})

test('it executes inline macros', async (t) => {
  TTML.setToken('me', token, tokenMacros)

  const step = {
    args: [
      { Say: { Message: 'hello' } },
      { Say: { From: { attribute: null, macro_name: null, name: 'me' } } },
    ],
    op: 'Say',
    result: 'Ignore',
  }

  const inlineStep = {
    args: [
      { Token: { name: 'me', attribute: null, macro_name: 'uncanny-dodge' } },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }

  // mock the parse function used to inline macros
  const parse = sinon.stub().returns({
    name: 'uncanny-dodge',
    steps: [{
      args: [
        { Roll: { N: { Number: 1 } } },
        { Roll: { D: { Number: 20 } } },
      ],
      op: 'Roll',
      result: 'Save',
    }],
  })
  executor.parse = parse

  const output = await execute([ step, inlineStep ], false)
  t.true(executor.parse.calledOnce)
  t.true(executor.parse.calledWith('#uncanny-dodge !r 1d20 "Uncanny dodge!"'))
})

test('it gets saved results', async (t) => {
  const output = await execute([ stepRoll, stepRolld8 ], false)
  t.true(executor.api.calledWith(`1d8+12`))
})

test('it gets token attributes', async (t) => {
  const step = {
    args: [
      { Roll: { N: { Number: 1 } } },
      { Roll: { D: { Number: 8 } } },
      { Roll: { ModifierNeg: { Token: { attribute: 'dexterity_mod', macro_name: null, name: 'me' } } } },
    ],
    op: 'Roll',
    result: 'Save',
  }

  TTML.setToken('me', token, {})

  const output = await execute([ step ], false)
  t.true(executor.api.calledWith(`1d8-2`))
})

test('it gets token attributes from @target', async (t) => {
  const step = {
    args: [
      { Roll: { N: { Number: 1 } } },
      { Roll: { D: { Number: 8 } } },
      { Roll: { ModifierNeg: { Token: { attribute: 'dexterity_mod', macro_name: null, name: 'target' } } } },
    ],
    op: 'Roll',
    result: 'Save',
  }

  TTML.setToken('me', token, {})

  executor.target = sinon.stub().resolves('me')
  const output = await execute([ stepTarget, step ], false)
  t.true(executor.api.calledWith(`1d8-2`))
})

test('it gets variables', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Variable: 'foo' },
          right: [
            { Number: 42 },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  const step = {
    args: [
      { Roll: { N: { Number: 2 } } },
      { Roll: { D: { Number: 20 } } },
      { Roll: { ModifierPos: { Variable: 'foo' } } },
    ],
    op: 'Roll',
    result: 'Save',
  }

  await execute([ assignStep, step ], false)
  t.true(executor.api.calledWith('2d20+42'))
})

test('it sets saved results', async (t) => {
  const output = await execute([ stepRoll ], false)
  t.not(typeof executor.variables['1'], 'undefined')
  t.is(executor.variables['1'], 12)
})

test('it sets token attribues using numbers', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Token: { attribute: 'boo', macro_name: null, name: 'me' } },
          right: [
            { Number: 10 },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  await execute([ assignStep ], false)
  t.not(typeof executor.tokens.me.attributes.boo, 'undefined')
  t.is(executor.tokens.me.attributes.boo.Number, 10)

  const assignStep2 = {
    args: [
      {
        Assign: {
          left: { Token: { attribute: 'baa', macro_name: null, name: 'me' } },
          right: [
            { Float: 15.5 },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  await execute([ assignStep2 ], false)
  t.not(typeof executor.tokens.me.attributes.baa, 'undefined')
  t.is(executor.tokens.me.attributes.baa.Float, 15.5)
})

test('it sets token attributes using strings', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Token: { attribute: 'rar', macro_name: null, name: 'me' } },
          right: [
            { Text: 'Test String' },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  await execute([ assignStep ], false)
  t.not(typeof executor.tokens.me.attributes.rar, 'undefined')
  t.is(executor.tokens.me.attributes.rar.Text, 'Test String')
})

test('it sets variables using interpolated strings', async (t) => {
  const assignStep = {
    args: [{
      Assign: {
        left: { Token: { attribute: 'raz', macro_name: null, name: 'me' } },
        right: [{
          TextInterpolated: {
            parts: [
              { Text: 'Test String ' },
              { Token: { name: 'me', attribute: 'name', macro_name: null } },
            ],
          }
        }],
      },
    }],
    op: 'Lambda',
    result: 'Ignore',
  }
  TTML.setToken('me', token, {})
  await execute([ assignStep ], false)
  t.not(typeof executor.tokens.me.attributes.raz, 'undefined')
  t.is(executor.tokens.me.attributes.raz.Text, 'Test String Tester')
})

test('it sets variables using concat strings', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Token: { attribute: 'raz', macro_name: null, name: 'me' } },
          right: [
            { Text: 'Test String' },
            { Text: 'Combined' },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  await execute([ assignStep ], false)
  t.not(typeof executor.tokens.me.attributes.raz, 'undefined')
  t.is(executor.tokens.me.attributes.raz.Text, 'Test String Combined')
})

test('it sets variables using primitives', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Token: { attribute: 'roo', macro_name: null, name: 'me' } },
          right: [
            { Token: { attribute: 'hp', macro_name: null, name: 'me' } },
            { Primitive: 'Subtract' },
            { Float: 5.5 },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  // Set a token
  TTML.setToken('me', token, {})

  await execute([ assignStep ], false)
  t.not(typeof executor.tokens.me.attributes.roo, 'undefined')
  t.is(executor.tokens.me.attributes.roo.Float, 36.5)
})

test('it sets variables using numbers', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Variable: 'num' },
          right: [
            { Number: 1 },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  const output = await execute([ assignStep ], false)
  t.not(typeof executor.results.num, 'undefined')
  t.is(executor.results.num, 1)

  const assignStep2 = {
    args: [
      {
        Assign: {
          left: { Variable: 'num2' },
          right: [
            { Float: 5.5 },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  await execute([ assignStep2 ], false)
  t.not(typeof executor.results.num2, 'undefined')
  t.is(executor.results.num2, 5.5)
})

test('it sets variables using strings', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Variable: 'bar' },
          right: [
            { Text: 'Test String' },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  const output = await execute([ assignStep ], false)
  t.not(typeof executor.results.bar, 'undefined')
  t.is(executor.results.bar, 'Test String')
})

test('it sets variables using concat strings', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Variable: 'baz' },
          right: [
            { Text: 'Test String' },
            { Text: 'Combined' },
            { Text: 'And More' },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  const output = await execute([ assignStep ], false)
  t.not(typeof executor.results.baz, 'undefined')
  t.is(executor.results.baz, 'Test String Combined And More')
})

test('it sets variables using primitives', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Variable: 'foo' },
          right: [
            { Token: { attribute: 'hp', macro_name: null, name: 'me' } },
            { Primitive: 'Add' },
            { Number: 5 },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  // Set a token
  TTML.setToken('me', token, {})

  const output = await execute([ assignStep ], false)
  t.not(typeof executor.results.foo, 'undefined')
  t.is(executor.results.foo, 47)
})

test.skip('it initializes with a TTML object', (t) => {})
