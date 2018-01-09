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
let step1
let step2
let step3
let step4
let step5
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
    {"id":"5e5f9750-6e78-4108-8fcf-b74731e2d0a0","comment":null,"dice":[{"id":"60b2dfd5-2f05-4bb6-b2ee-0ef7435c0d33","child":null,"die":"D20","is_dropped":false,"is_rerolled":false,"is_successful":true,"max":20,"min":1,"sides":20,"timestamp":"2018-01-09T14:20:20.374915727Z","value":12}],"equation":"1d20","execution_time":0,"modifiers":[],"raw_value":12,"timestamp":"2018-01-09T14:20:20.374917631Z","value":12}
  )

  executor = Executor({
    api,
  })

  // Reset the steps
  step1 = {
    args: [
      { Roll: { N: { Number: 1 } } },
      { Roll: { D: { Number: 20 } } },
    ],
    op: 'Roll',
    result: 'Save',
  }

  step2 = {
    args: [
      {
        Prompt: {
          Message: 'Select your attack',
          options: {
            '0': { Token: { attribute: 'dexterity_mod', macro_name: null, name: 'me' } },
            '1': { Token: { attribute: 'strength_mod', macro_name: null, name: 'me' } },
          },
        },
      },
    ],
    op: 'Prompt',
    result: 'Save',
  }

  step3 = {
    args: [
      { Target: { Message: 'Choose your victim' } },
    ],
    op: 'Target',
    result: 'Save',
  }

  step4 = {
    args: [
      { Roll: { N: { Number: 1 } } },
      { Roll: { D: { Number: 8 } } },
      { Roll: { ModifierPos: { VariableReserved: 1 } } },
    ],
    op: 'Roll',
    result: 'Save',
  }

  step5 = {
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
        success: step1,
      },
    }],
    op: 'Lambda',
    result: 'Ignore',
  }

  // Reset the testing program
  program = {
    name: 'test-macro',
    steps: [
      step1,
      step2,
      step3,
      step4,
      step5,
    ],
  }

  // Reset token
  token = {
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
  let output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 1, 'does not compare EqualTo')

  conditionalStep.args[0].Conditional.left = { Number: 9 }
  conditionalStep.args[0].Conditional.right = { Number: 10 }
  output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 0, 'does not compare EqualTo')
})

test('it executes a >= Conditional step', async (t) => {
  conditionalStep.args[0].Conditional.comparison = 'GreaterThanOrEqual' 
  conditionalStep.args[0].Conditional.left = { Float: 15.5 }
  conditionalStep.args[0].Conditional.right = { Number: 10 }
  let output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 1, 'does not compare GreaterThanOrEqual')

  output.rolls = []
  conditionalStep.args[0].Conditional.comparison = 'GreaterThanOrEqual' 
  conditionalStep.args[0].Conditional.left = { Number: 5 }
  conditionalStep.args[0].Conditional.right = { Number: 15 }
  output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 0, 'does not compare GreaterThanOrEqual')

  output.rolls = []
  conditionalStep.args[0].Conditional.comparison = 'GreaterThanOrEqual' 
  conditionalStep.args[0].Conditional.left = { Number: 15 }
  conditionalStep.args[0].Conditional.right = { Number: 15 }
  output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 1, 'does not compare GreaterThanOrEqual')
})

test('it executes a <= Conditional step', async (t) => {
  conditionalStep.args[0].Conditional.comparison = 'LessThanOrEqual' 
  conditionalStep.args[0].Conditional.left = { Float: 5.25 }
  conditionalStep.args[0].Conditional.right = { Number: 6 }
  let output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 1, 'does not compare LessThanOrEqual')

  conditionalStep.args[0].Conditional.comparison = 'LessThanOrEqual' 
  conditionalStep.args[0].Conditional.left = { Number: 25 }
  conditionalStep.args[0].Conditional.right = { Number: 5 }
  output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 0, 'does not compare LessThanOrEqual')

  conditionalStep.args[0].Conditional.comparison = 'LessThanOrEqual' 
  conditionalStep.args[0].Conditional.left = { Number: 2 }
  conditionalStep.args[0].Conditional.right = { Number: 2 }
  output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 1, 'does not compare LessThanOrEqual')
})

test('it executes a > Conditional step', async (t) => {
  conditionalStep.args[0].Conditional.comparison = 'GreaterThan' 
  conditionalStep.args[0].Conditional.left = { Float: 15.5 }
  conditionalStep.args[0].Conditional.right = { Number: 10 }
  let output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 1, 'does not compare GreaterThan')

  conditionalStep.args[0].Conditional.comparison = 'GreaterThan' 
  conditionalStep.args[0].Conditional.left = { Number: 5 }
  conditionalStep.args[0].Conditional.right = { Number: 15 }
  output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 0, 'does not compare GreaterThan')

  conditionalStep.args[0].Conditional.comparison = 'GreaterThan' 
  conditionalStep.args[0].Conditional.left = { Number: 15 }
  conditionalStep.args[0].Conditional.right = { Number: 15 }
  output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 0, 'does not compare GreaterThan')
})

test('it executes a < Conditional step', async (t) => {
  conditionalStep.args[0].Conditional.comparison = 'LessThan' 
  conditionalStep.args[0].Conditional.left = { Float: 5.25 }
  conditionalStep.args[0].Conditional.right = { Number: 6 }
  let output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 1, 'does not compare LessThan')

  output.rolls = []
  conditionalStep.args[0].Conditional.comparison = 'LessThan' 
  conditionalStep.args[0].Conditional.left = { Number: 25 }
  conditionalStep.args[0].Conditional.right = { Number: 5 }
  output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 0, 'does not compare LessThan')

  output.rolls = []
  conditionalStep.args[0].Conditional.comparison = 'LessThan' 
  conditionalStep.args[0].Conditional.left = { Number: 2 }
  conditionalStep.args[0].Conditional.right = { Number: 2 }
  output = await execute([ conditionalStep ])
  t.is(output.rolls.length, 0, 'does not compare LessThan')
})

test('it executes a Exit step', async (t) => {
  const exitStep = {
    args: [],
    op: 'Exit',
    result: 'Ignore',
  }

  const output = await execute([ exitStep, step1 ])
  t.is(output.rolls.length, 0)
})

test.skip('it executes a Prompt step', (t) => {})

test('it executes a Roll step using the api', async (t) => {
  let output = await execute([ step1 ])
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
  output = await execute([ roll ])
  t.true(executor.api.calledWith('1d8gt6'))
})

test.skip('it executes a Say step', (t) => {})
test.skip('it executes a Target step', (t) => {})
test.skip('it executes a Whisper step', (t) => {})
test.skip('it executes a program cleanly', (t) => {})
test.skip('it executes inline macros', (t) => {})
test.skip('it executes inline macros', (t) => {})
test.skip('it executes primitive ops', (t) => {})
test.skip('it gets saved results', (t) => {})
test.skip('it gets token attributes', (t) => {})
test.skip('it gets variables', (t) => {})

test('it sets saved results', async (t) => {
  const output = await execute([ step1 ])
  t.not(typeof output.variables['1'], 'undefined')
  t.is(output.variables['1'], 12)
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
  await execute([ assignStep ])
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
  await execute([ assignStep2 ])
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
  await execute([ assignStep ])
  t.not(typeof executor.tokens.me.attributes.rar, 'undefined')
  t.is(executor.tokens.me.attributes.rar.Text, 'Test String')
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
  await execute([ assignStep ])
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

  await execute([ assignStep ])
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
  const output = await execute([ assignStep ])
  t.not(typeof output.results.num, 'undefined')
  t.is(output.results.num, 1)

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
  const output2 = await execute([ assignStep2 ])
  t.not(typeof output2.results.num2, 'undefined')
  t.is(output2.results.num2, 5.5)
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
  const output = await execute([ assignStep ])
  t.not(typeof output.results.bar, 'undefined')
  t.is(output.results.bar, 'Test String')
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
          ],
        },
      },
    ],
    op: 'Lambda',
    result: 'Ignore',
  }
  const output = await execute([ assignStep ])
  t.not(typeof output.results.baz, 'undefined')
  t.is(output.results.baz, 'Test String Combined')
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

  const output = await execute([ assignStep ])
  t.not(typeof output.results.foo, 'undefined')
  t.is(output.results.foo, 47)
})

test.skip('it initializes with a TTML object', (t) => {})
