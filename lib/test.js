import test from 'ava'
import { TTML } from './index'
import sinon from 'sinon'
import Executor, {
  buildToken,
  unbuildToken,
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

  const api = sinon.stub().returns({
    roll: {'id':'5e5f9750-6e78-4108-8fcf-b74731e2d0a0','comment':null,'dice':[{'id':'60b2dfd5-2f05-4bb6-b2ee-0ef7435c0d33','child':null,'die':'D20','is_dropped':false,'is_rerolled':false,'is_successful':true,'max':20,'min':1,'sides':20,'timestamp':'2018-01-09T14:20:20.374915727Z','value':12}],'equation':'1d20','execution_time':0,'modifiers':[],'raw_value':12,'timestamp':'2018-01-09T14:20:20.374917631Z','value':12},
  })

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
            { key: '0', value: { Token: { attribute: 'dexterity_mod', macro_name: null, name: 'me' } } },
            { key: '1', value: { Token: { attribute: 'strength_mod', macro_name: null, name: 'me' } } },
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
    hp_formula: '20d20+14',
    hp: 42,
    strength_mod: 11,
    dexterity_mod: 2,
    x: 1240.92,
    y: -1901.11,
    is_test: true,
    attacks: {
      '0': {
        name: 'Melee',
      },
    },
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
      hp_formula: { Text: '20d20+14' },
      strength_mod: { Number: 11 },
      dexterity_mod: { Number: 2 },
      x: { Float: 1240.92 },
      y: { Float: -1901.11 },
      is_test: { Boolean: true },
      attacks: {
        '0': {
          name: { Text: 'Melee' },
        },
      },
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

test('.addFunction adds a callback for a custom function', (t) => {
  const addFunctionAction = TTML.addFunction('get', (i) => {})
  t.is(typeof addFunctionAction, 'function')
})

test('.buildToken sets correct data structure', (t) => {
  const token = {
    name: 'test',
    a: 'b',
    c: {
      d: 'e',
      f: [ 'g' ],
    },
    h: 1,
    j: true,
    k: 3.14,
  }

  const expectedToken = {
    name: { Text: 'test' },
    a: { Text: 'b' },
    c: {
      d: { Text: 'e' },
      f: { Array: [ { Text: 'g' } ] },
    },
    h: { Number: 1 },
    j: { Boolean: true },
    k: { Float: 3.14 },
  }

  const outputToken = buildToken(token)
  t.deepEqual(outputToken, expectedToken)
})

test('.unbuildToken returns deconstructed data structure', (t) => {
  const token = {
    name: { Text: 'test' },
    a: { Text: 'b' },
    c: {
      d: { Text: 'e' },
      f: { Array: [ { Text: 'g' } ] },
    },
    h: { Number: 1 },
    j: { Boolean: true },
    k: { Float: 3.14 },
  }

  const expectedToken = {
    name: 'test',
    a: 'b',
    c: {
      d: 'e',
      f: [ 'g' ],
    },
    h: 1,
    j: true,
    k: 3.14,
  }

  const outputToken = unbuildToken(token)
  t.deepEqual(outputToken, expectedToken)
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

test('.setStaticToken sets a reference to a token', (t) => {
  TTML.setToken('ref_name', token, tokenMacros)
  TTML.setStaticToken('token_name', 'ref_name')
  const staticTokens = TTML.getRawTokens()
  t.deepEqual(staticTokens.token_name, tokenRaw, 'unexpected raw token structure')
})

test('.setStaticToken sets multiple references to tokens', (t) => {
  TTML.setToken('ref_name', token, tokenMacros)
  TTML.setToken('ref_name_2', { hp: 100 })
  TTML.setStaticToken('selected', [ 'ref_name', 'ref_name_2' ])
  const staticTokens = TTML.getRawTokens()
  t.deepEqual(staticTokens.selected, [
    tokenRaw,
    {
      attributes: {
        hp: { Number: 100 },
      },
      macros: {},
    }
  ])
})

test('.setStaticToken sets a reference to a token', (t) => {
  TTML.setToken('ref_name', token, tokenMacros)
  TTML.setStaticToken('token_name', 'ref_name')
  const staticTokens = TTML.getRawTokens()
  t.deepEqual(staticTokens.token_name, tokenRaw, 'unexpected raw token structure')
})

test('it executes a Case step', async (t) => {
  executor.prompt = sinon.stub().resolves('1')
  let stepCase = {
    args: [
      {
        Case: {
          input: {
            Variable: 'foo',
          },
          options: [
            { key: 0, value: { Number: 42 } },
            { key: 'bar', value: { Token: { attribute: 'strength_mod', macro_name: null, name: 'me' } } },
          ],
        },
      },
    ],
    op: 'Case',
    result: 'Save',
  }

  TTML.setToken('me', token, {})
  executor.results['foo'] = 'bar'

  await execute([ stepCase ], false)
  t.is(executor.variables['1'], 11)
})

test('it executes a Case step with a reserved variable', async (t) => {
  executor.prompt = sinon.stub().resolves('0')
  let stepCase = {
    args: [
      {
        Case: {
          input: {
            VariableReserved: 0,
          },
          options: [
            { key: 12, value: { Number: 42 } },
            { key: 'bar', value: { Token: { attribute: 'strength_mod', macro_name: null, name: 'me' } } },
          ],
        },
      },
    ],
    op: 'Case',
    result: 'Save',
  }

  TTML.setToken('me', token, {})
  executor.results['foo'] = 'bar'

  await execute([ stepRoll, stepCase ], false)
  t.is(executor.variables['2'], 42)
})

test('it executes a == Conditional step', async (t) => {
  let output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 1, 'does not compare EqualTo')

  conditionalStep.args[0].Conditional.left = { Number: 9 }
  conditionalStep.args[0].Conditional.right = { Number: 10 }
  output = await execute([ conditionalStep ], false)
  t.is(output.rolls.length, 0, 'does not compare EqualTo')
})

test('it executes a == Conditional step with variable', async (t) => {
  TTML.setToken('me', token, {})

  const step = {
    args: [{
      Conditional: {
        comparison: 'EqualTo',
        failure: {
          args: [],
          op: 'Exit',
          result: 'Ignore',
        },
        left: { VariableReserved: 0 },
        right: { Number: 0 },
        success: stepRoll,
      },
    }],
    op: 'Lambda',
    result: 'Ignore',
  }

  executor.prompt = sinon.stub().resolves('0')
  let output = await execute([ stepPrompt, step ], false)
  t.is(output.rolls.length, 1, 'does not compare EqualTo')
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

test('it executes a custom function step', async (t) => {
  TTML.setToken('me', token, {})
  executor.functions.get = sinon.stub().resolves({ id: 'foo', content: 'bar' })
  const step = {
    args: [
      {
        Concat: {
          left: { Token: { attribute: 'attacks', macro_name: null, name: 'me' } },
          right: [{
            Step: {
              args: [
                { Function: { Text: 'foo' } },
                {
                  Function: { 
                    TextInterpolated: {
                      parts: [
                        { Text: 'Sword of Awesome' },
                      ],
                    },
                  }
                },
              ],
              op: { Function: 'get' },
              result: 'Ignore',
            },
          }],
        },
      },
    ],
    op: 'Lambda',
    result: 'Save',
  }
  const output = await execute([ step ], false)
  t.true(executor.functions.get.calledOnce)
  t.true(executor.functions.get.calledWith('foo', 'Sword of Awesome'))
})

test('it executes a Input step', async (t) => {
  executor.input = sinon.stub().resolves('42')
  const step = {
    args: [{
      Input: {
        parts: [
          { Text: 'Type something' },
        ],
      },
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
  executor.prompt = sinon.stub().resolves('0')
  const output = await execute([ stepPrompt ], false)
  t.is(executor.variables['1'], '0')
  t.true(executor.prompt.calledWith('Select your attack', [
    { key: '0', value: '@me.dexterity_mod' },
    { key: '1', value: '@me.strength_mod' },
    { key: 'asd', value: 'asd' },
  ]))
})

test('it executes a Prompt step with offset keys', async (t) => {
  TTML.setToken('me', token, {})
  const step = {
    args: [
      {
        Prompt: {
          message: {
            parts: [
              { Text: 'Select your attack' },
            ],
          },
          options: [
            { key: '1', value: { Token: { attribute: 'dexterity_mod', macro_name: null, name: 'me' } } },
            { key: '2', value: { Token: { attribute: 'strength_mod', macro_name: null, name: 'me' } } },
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
  executor.prompt = sinon.stub().resolves('0') // select the first option
  const output = await execute([ step ], false)
  t.true(executor.prompt.calledWith('Select your attack', [
    { key: '1', value: '@me.dexterity_mod' },
    { key: '2', value: '@me.strength_mod' },
    { key: 'asd', value: 'asd' },
  ]))
  t.is(executor.variables['1'], '1')
})

test('skips executing a Prompt step with no options given', async (t) => {
  executor.prompt = sinon.stub().resolves()
  const stepPromptNoOptions = {
    args: [
      {
        Prompt: {
          message: {
            parts: [
              { Text: 'This is silly' },
            ],
          },
          options: [],
        },
      },
    ],
    op: 'Prompt',
    result: 'Ignore',
  }
  const output = await execute([ stepPromptNoOptions ], false)
  t.true(executor.prompt.notCalled)
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

  const roll3 = {
    args: [
      { Token: { attribute: 'hp_formula', macro_name: null, name: 'me' } },
    ],
    op: 'Roll',
    result: 'Ignore',
  }
  TTML.setToken('me', token, {})
  output = await execute([ roll3 ], false)
  t.true(executor.api.calledWith('20d20+14'))

  const roll4 = {
    args: [
      { Roll: { N: { Number: 1 } } },
      { Roll: { D: { Number: 8 } } },
      { Roll: { RR: { op: 'LessThan', value: { Number: 6 } } } },
    ],
    op: 'Roll',
    result: 'Ignore',
  }
  output = await execute([ roll4 ], false)
  t.true(executor.api.calledWith('1d8rr<6'))

  const roll5 = {
    args: [
      { Roll: { N: { Number: 1 } } },
      { Roll: { D: { Number: 20 } } },
      { Roll: { Comment: { Text: 'Comment Strike (+2 Proficiency)' } } },
    ],
    op: 'Roll',
    result: 'Ignore',
  }
  output = await execute([ roll5 ], false)
  t.true(executor.api.calledWith(`1d20['Comment Strike (+2 Proficiency)']`))
})

test('it executes a Roll command "as" another token', async (t) => {
  TTML.setToken('ash', { initiative: 0 })

  const roll = {
    args: [
      { Token: { name: 'ash', attribute: null, macro_name: null } },
      { Roll: { N: { Number: 1 } } },
      { Roll: { D: { Number: 8 } } },
      { Roll: { RO: { op: 'GreaterThan', value: { Number: 7 } } } },
    ],
    op: 'Roll',
    result: 'Save',
  }

  const assignStep = {
    args: [
      {
        Assign: {
          left: { Token: { attribute: 'initiative', macro_name: null, name: 'ash' } },
          right: [
            { VariableReserved: 0 },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  const output = await execute([ roll, assignStep ], false)
  t.true(executor.api.calledWith('1d8ro>7'))
  t.is(output.rolls[0].to, null)
  t.is(output.rolls[0].as, 'ash')
})

test('it executes a Hidden Roll step', async (t) => {
  const roll = {
    args: [
      { Roll: { N: { Number: 1 } } },
      { Roll: { D: { Number: 20 } } },
    ],
    op: 'RollHidden',
    result: 'Save',
  }

  const output = await execute([ roll ], false)

  t.is(output._raw.length, 0)
  t.is(output.rolls.length, 1)
  t.not(typeof executor.variables['1'], 'undefined')
  t.is(executor.variables['1'], 12)
})

test('it executes a Whisper Roll step', async (t) => {
  const roll = {
    args: [
      { Token: { name: 'gm', attribute: null, macro_name: null } },
      { Roll: { N: { Number: 1 } } },
      { Roll: { D: { Number: 20 } } },
    ],
    op: 'RollWhisper',
    result: 'Save',
  }

  const output = await execute([ roll ], false)

  t.is(output._raw.length, 1)
  t.is(output.rolls.length, 1)

  const outputRoll = output.rolls[0]
  t.is(outputRoll.to, 'gm')
  t.is(outputRoll.as, null)
})

// test('it executes a Roll step with multiple rolls', async (t) => {
  // const assignStep = {
    // args: [
      // {
        // Assign: {
          // left: { Variable: 'foo' },
          // right: [
            // { Text: '2nd' },
          // ],
        // },
      // },
    // ],
    // op: 'Lambda',
    // result: 'Ignore',
  // }
  // const roll = {
    // args: [
      // { Roll: { N: { Number: 1 } } },
      // { Roll: { D: { Number: 8 } } },
      // { Roll: { GT: { Number: 6 } } },
      // { Roll: { Comment: { Text: 'This is my comment for roll 1' } } },
      // { Roll: { Primitive: 'Add' } },
      // { Roll: { N: { Number: 2 } } },
      // { Roll: { D: { Number: 20 } } },
      // { Roll: { ModifierNeg: { Number: 2 } } },
      // {
        // Roll: {
          // Comment: {
            // TextInterpolated: {
              // parts: [
                // { Text: 'This is my ' },
                // { Variable: 'foo' },
                // { Text: ' roll' },
              // ],
            // },
          // },
        // },
      // },
    // ],
    // op: 'Roll',
    // result: 'Ignore',
  // }

  // await execute([ assignStep, roll ], false)
  // t.true(executor.api.calledWith('1d8gt6[This is my comment for roll 1] + 2d20-2[This is my 2nd roll]'))
// })
//
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

test('it executes a Template step', async (t) => {
  const templateStep = {
    args: [
      {
        Template: {
          Name: 'default',
        },
      },
      {
        Template: {
          Attributes: {
            Object: {
              foo: { Text: 'bar' },
              hp: { Variable: 'hp' },
              attack: {
                Object: {
                  base: {
                    TextInterpolated: {
                      parts: [
                        { Text: '1d20' },
                      ],
                    },
                  },
                },
              },
              name: {
                TextInterpolated: {
                  parts: [
                    { Token: { name: 'me', attribute: 'name', macro_name: null } },
                    { Text: ' & co.' },
                  ],
                },
              },
            },
          },
        },
      },
    ],
    op: 'Template',
    result: 'Ignore',
  }

  TTML.setToken('me', token, tokenMacros)
  executor.results.hp = 42

  const output = await execute([ templateStep ], false)
  t.is(output._raw.length, 1)
  t.is(output.templates.length, 1)
  t.deepEqual(output.templates[0], {
    name: 'default',
    attributes: {
      foo: 'bar',
      hp: 42,
      attack: {
        base: '1d20',
      },
      name: 'Tester & co.',
    },
  })
})

test('it executes a Target step', async (t) => {
  executor.target = sinon.stub().resolves('id')
  const output = await execute([ stepTarget ], false)
  t.is(executor._target, 'id')
})

test('it executes a Test step set to true', async (t) => {
  const stepTest = {
    args: [
      { TestMode: true }
    ],
    op: 'TestMode',
    result: 'Ignore',
  }
  const output = await execute([ stepRoll, stepTest, stepRolld8 ])
  t.is(output.rolls.filter(r => r.is_test).length, 1)
})

test('it executes a Test step set to false', async (t) => {
  const stepTest = {
    args: [
      { TestMode: false }
    ],
    op: 'TestMode',
    result: 'Ignore',
  }
  const output = await execute([ stepRoll, stepTest, stepRolld8 ])
  t.is(output.rolls.filter(r => r.is_test).length, 0)
})

test('it executes a Whisper step', async (t) => {
  const whisperSay = {
    args: [
      {
        Say: {
          To: {
            attribute: null,
            macro_name: null,
            name: 'gm',
          },
        },
      },
      {
        Say: {
          Message: {
            parts: [
              { Text: 'hello my fine game master' },
            ],
          },
        }
      },
      { Say: { From: { attribute: null, macro_name: null, name: 'me' } } },
    ],
    op: 'Whisper',
    result: 'Ignore',
  }

  const output = await execute([ whisperSay ], false)
  t.is(output.messages.length, 1)
  t.is(output._raw[0].type, 'message')
  t.is(output._raw[0].data.from, 'me')
  t.is(output._raw[0].data.to, 'gm')
  t.is(output._raw[0].data.message, 'hello my fine game master')
  t.is(output.messages[0].from, 'me')
  t.is(output.messages[0].to, 'gm')
  t.is(output.messages[0].message, 'hello my fine game master')
})

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

test('it gets deeply-nested variables', async (t) => {
  const step = {
    args: [
      {
        Say: {
          Message: {
            parts: [
              { Text: 'Attacking with ' },
              {
                Token: {
                  attribute: 'attacks.0.name',
                  macro_name: null,
                  name: 'me'
                },
              },
            ],
          },
        }
      },
      { Say: { From: { attribute: null, macro_name: null, name: 'me' } } },
    ],
    op: 'Say',
    result: 'Ignore',
  }

  TTML.setToken('me', token, tokenMacros)

  const output = await execute([ step ], false)
  t.is(output.messages[0].message, 'Attacking with Melee')
})

test('it gets deeply-nested tokens', async (t) => {
  const step = {
    args: [
      {
        Say: {
          Message: {
            parts: [
              { Text: 'This is the real ' },
              {
                Token: {
                  attribute: '1.name',
                  macro_name: null,
                  name: 'selected'
                },
              },
            ],
          },
        }
      },
      { Say: { From: { attribute: null, macro_name: null, name: 'me' } } },
    ],
    op: 'Say',
    result: 'Ignore',
  }

  TTML.setToken('me', token, tokenMacros)
  TTML.setToken('ref_1', { name: 'Brim Brady' })
  TTML.setToken('ref_2', { name: 'Slim Shady' })
  TTML.setStaticToken('selected', [ 'ref_1', 'ref_2' ])

  const output = await execute([ step ], false)
  t.is(output.messages[0].message, 'This is the real Slim Shady')
})

test('it gets deeply-nested tokens and deeply-nested variables', async (t) => {
  const step = {
    args: [
      {
        Say: {
          Message: {
            parts: [
              { Text: 'This is the real ' },
              {
                Token: {
                  attribute: '1.name',
                  macro_name: null,
                  name: 'selected'
                },
              },
              { Text: ' coming at you with a ' },
              {
                Token: {
                  attribute: '1.attacks.0.name',
                  macro_name: null,
                  name: 'selected'
                },
              },
              { Text: ' attack' },
            ],
          },
        }
      },
      { Say: { From: { attribute: null, macro_name: null, name: 'me' } } },
    ],
    op: 'Say',
    result: 'Ignore',
  }

  TTML.setToken('me', token, tokenMacros)
  TTML.setToken('ref_1', { name: 'Brim Brady' })
  TTML.setToken('ref_2', Object.assign({}, token, { name: 'Slim Shady' }))
  TTML.setStaticToken('selected', [ 'ref_1', 'ref_2' ])

  const output = await execute([ step ], false)
  t.is(output.messages[0].message, 'This is the real Slim Shady coming at you with a Melee attack')
})

test('it sets saved results', async (t) => {
  const output = await execute([ stepRoll ], false)
  t.not(typeof executor.variables['1'], 'undefined')
  t.is(executor.variables['1'], 12)
})

test('it pops the last result when not saving a result', async (t) => {
  const popStep = {
    args: [
      { Roll: { N: { Number: 1 } } },
      { Roll: { D: { Number: 8 } } },
      { Roll: { ModifierNeg: { VariableReserved: 0 } } },
    ],
    op: 'Roll',
    result: 'Ignore',
  }

  await execute([ stepRoll, popStep ], false)

  t.true(executor.api.calledWith('1d8-12'))
})

test('it pops the last result when saving a result', async (t) => {
  const popStep = {
    args: [
      {
        Say: {
          Message: {
            parts: [
              { Text: 'I have ' },
              { VariableReserved: 0 },
              { Text: ' schmeckles!' },
            ],
          },
        }
      },
      { Say: { From: { attribute: null, macro_name: null, name: 'me' } } },
    ],
    op: 'Say',
    result: 'Save',
  }

  executor.variables['1'] = 40
  executor.variables['2'] = 10
  const output = await execute([ popStep ], false)

  t.is(output.messages[0].message, 'I have 10 schmeckles!')
})

test('it sets token attributes using booleans (true)', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Token: { attribute: 'rar', macro_name: null, name: 'me' } },
          right: [
            { Boolean: true },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  await execute([ assignStep ], false)
  t.not(typeof executor.tokens.me.attributes.rar, 'undefined')
  t.is(executor.tokens.me.attributes.rar.Boolean, true)
})

test('it sets deep token attributes using booleans', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Token: { attribute: 'rar.0.bar', macro_name: null, name: 'me' } },
          right: [
            { Boolean: true },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  await execute([ assignStep ], false)
  t.not(typeof executor.tokens.me.attributes.rar.Array[0].bar, 'undefined')
  t.is(executor.tokens.me.attributes.rar.Array[0].bar.Boolean, true)
})

test('it sets deep token attributes using JSON structures (Array)', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Token: { attribute: 'rar.0.bar', macro_name: null, name: 'me' } },
          right: [{
            Array: [{
              Object: {
                name: { Text: 'test', },
                weight: { Float: 1.1 },
              },
            }],
          }],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  await execute([ assignStep ], false)
  t.not(typeof executor.tokens.me.attributes.rar.Array[0].bar, 'undefined')
  t.deepEqual(executor.tokens.me.attributes.rar.Array[0].bar.Array, [{
    name: { Text: 'test' },
    weight: { Float: 1.1 },
  }])
})

test('it sets deep token attributes using JSON structures (Object)', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Token: { attribute: 'rar.raz', macro_name: null, name: 'me' } },
          right: [{
            Object: {
              name: { Text: 'test', },
              weight: { Float: 1.1 },
              something: {
                Array: [{
                  Boolean: true,
                }],
              },
            },
          }],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  TTML.setToken('me', token, {})
  await execute([ assignStep ], false)
  t.not(typeof executor.tokens.me.attributes.rar.raz, 'undefined')
  t.deepEqual(executor.tokens.me.attributes.rar.raz, {
    name: { Text: 'test' },
    weight: { Float: 1.1 },
    something: {
      Array: [
        { Boolean: true },
      ],
    },
  })
})

test('it pushes elements to a token attribute array', async (t) => {
  const assignStep = {
    args: [
      {
        Concat: {
          left: { Token: { attribute: 'attacks', macro_name: null, name: 'me' } },
          right: [{
            Object: {
              name: { Text: 'test', },
            },
          }],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  TTML.setToken('me', token, {})
  await execute([ assignStep ], false)
  t.is(Object.keys(executor.tokens.me.attributes.attacks).length, 2)
  t.deepEqual(executor.tokens.me.attributes.attacks['1'], {
    name: { Text: 'test' },
  })
})

test('it pushes multiple consecutive elements to a token attribute array', async (t) => {
  const assignStep = {
    args: [
      {
        Concat: {
          left: { Token: { attribute: 'attacks', macro_name: null, name: 'me' } },
          right: [{
            Object: {
              name: { Text: 'test', },
            },
          }],
        },
      },
      {
        Concat: {
          left: { Token: { attribute: 'attacks', macro_name: null, name: 'me' } },
          right: [{
            Object: {
              name: { Text: 'test 2', },
            },
          }],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  TTML.setToken('me', token, {})
  await execute([ assignStep ], false)
  t.is(Object.keys(executor.tokens.me.attributes.attacks).length, 3)
  t.deepEqual(executor.tokens.me.attributes.attacks['1'], {
    name: { Text: 'test' },
  })
  t.deepEqual(executor.tokens.me.attributes.attacks['2'], {
    name: { Text: 'test 2' },
  })
})

test('it concats string token attributes', async (t) => {
  const assignStep = {
    args: [
      {
        Concat: {
          left: { Token: { attribute: 'name', macro_name: null, name: 'me' } },
          right: [{
            Text: " Test"
          }],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  TTML.setToken('me', token, {})
  t.is(executor.tokens.me.attributes.name.Text, 'Tester')
  await execute([ assignStep ], false)
  t.is(executor.tokens.me.attributes.name.Text, 'Tester Test')
})

test('it concats number token attributes', async (t) => {
  const assignStep = {
    args: [
      {
        Concat: {
          left: { Token: { attribute: 'hp', macro_name: null, name: 'me' } },
          right: [{
            Number: 5
          }],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  TTML.setToken('me', token, {})
  t.is(executor.tokens.me.attributes.hp.Number, 42)
  await execute([ assignStep ], false)
  t.is(executor.tokens.me.attributes.hp.Number, 47)
})

test('it deducts string token attributes', async (t) => {
  const assignStep = {
    args: [
      {
        Deduct: {
          left: { Token: { attribute: 'name', macro_name: null, name: 'me' } },
          right: [{
            Text: 'er'
          }],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  TTML.setToken('me', token, {})
  t.is(executor.tokens.me.attributes.name.Text, 'Tester')
  await execute([ assignStep ], false)
  t.is(executor.tokens.me.attributes.name.Text, 'Test')
})

test('it deducts number token attributes', async (t) => {
  const assignStep = {
    args: [
      {
        Deduct: {
          left: { Token: { attribute: 'hp', macro_name: null, name: 'me' } },
          right: [{
            Number: 2
          }],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  TTML.setToken('me', token, {})
  t.is(executor.tokens.me.attributes.hp.Number, 42)
  await execute([ assignStep ], false)
  t.is(executor.tokens.me.attributes.hp.Number, 40)
})

test('it sets token attributes using booleans (false)', async (t) => {
  const assignStep = {
    args: [
      {
        Assign: {
          left: { Token: { attribute: 'rar', macro_name: null, name: 'me' } },
          right: [
            { Boolean: false },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  await execute([ assignStep ], false)
  t.not(typeof executor.tokens.me.attributes.rar, 'undefined')
  t.is(executor.tokens.me.attributes.rar.Boolean, false)
})

test('it sets token attributes using numbers', async (t) => {
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

test('it sets variables using commands', async (t) => {
  TTML.setToken('me', token, {})

  const step1 = {
    args: [
      {
        Assign: {
          left: { Variable: 'roll' },
          right: [{
            Step: {
              args: [
                { Roll: { N: { Number: 1 } } },
                { Roll: { D: { Number: 20 } } },
              ],
              op: 'Roll',
              result: 'Ignore',
            },
          }],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }

  const step2 = {
    args: [
      {
        Say: {
          Message: {
            parts: [
              { Text: 'We rolled a ' },
              { Variable: 'roll' },
            ],
          },
        }
      },
      { Say: { From: { attribute: null, macro_name: null, name: 'me' } } },
    ],
    op: 'Say',
    result: 'Save',
  }

  const output = await execute([ step1, step2 ], false)
  t.is(output.messages[0].message, 'We rolled a 12')
  t.not(typeof executor.results.roll, 'undefined')
  t.is(executor.results.roll, 12)
})


test('sets variables inside comparitive operations', async (t) => {
  TTML.setToken('me', token, {})

  const localAssignFail = {
    args: [
      {
        Assign: {
          left: { Variable: 'foo' },
          right: [
            { Number: 2 },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }

  const localAssignSuccess = {
    args: [
      {
        Assign: {
          left: { Variable: 'foo' },
          right: [
            { Text: 'bar' },
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  
  const step1 = {
    args: [{
      Conditional: {
        comparison: 'EqualTo',
        failure: localAssignFail,
        left: { Number: 12 },
        right: { Number: 12 },
        success: localAssignSuccess,
      },
    }],
    op: 'Lambda',
    result: 'Ignore',
  }

  const step2 = {
    args: [
      {
        Say: {
          Message: {
            parts: [
              { Text: 'We assigned foo = ' },
              { Variable: 'foo' },
            ],
          },
        }
      },
      { Say: { From: { attribute: null, macro_name: null, name: 'me' } } },
    ],
    op: 'Say',
    result: 'Save',
  }

  const output = await execute([ step1, step2 ], false)
  t.is(output.messages[0].message, 'We assigned foo = bar')
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

test('throws an error if a variable cannot be found', async (t) => {
  const stepSay = {
    args: [
      {
        Say: {
          Message: {
            parts: [
              { Text: 'I have ' },
              { Variable: 'schmeckles' },
              { Text: ' schmeckles!' },
            ],
          },
        }
      },
      { Say: { From: { attribute: null, macro_name: null, name: 'me' } } },
    ],
    op: 'Say',
    result: 'Ignore',
  }

  const output = await t.throws(execute([ stepSay ], false))
  t.is(output.message, 'Variable ${schmeckles} is not set and can not be used')
})

test('throws an error if a Token attribute cannot be found', async (t) => {
  const stepSay = {
    args: [
      {
        Say: {
          Message: {
            parts: [
              { Text: 'I have ' },
              { Token: { attribute: 'grapples', macro_name: null, name: 'me' } },
              { Text: 'grapples' },
            ],
          },
        }
      },
      { Say: { From: { attribute: null, macro_name: null, name: 'me' } } },
    ],
    op: 'Say',
    result: 'Ignore',
  }

  TTML.setToken('me', token, tokenMacros)

  const output = await t.throws(execute([ stepSay ], false))
  t.is(output.message, 'Cannot find token attribute @me.grapples')
})
