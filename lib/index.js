const wasm = require('../src/main.rs')

// Private module to run wasm functions from TTML
const Module = {}

// TableTop Macro Language Object
const TTML = {
  /**
   * Add callback action for prompt method
   * @param {function} action; The action to be run when prompt is called
   * @return void
   */
  addPromptAction: (action = (i, o) => {}) => {
    Module.prompt = action
  },

  /**
   * Add callback action for target method
   * @param {function} action; The action to be run when prompt is called
   * @return void
   */
  addTargetAction: (action = (i) => {}) => {
    Module.target = action
  },

  /** Macro dictionary */
  macros: {},

  /** Token dictionary */
  tokens: {},

  prompt: (input, options = {}) => {
    if (typeof Module.prompt === 'function') {
      return Module.prompt(input, options)
    }
  },

  target: (input) => {
    if (typeof Module.target === 'function') {
      return Module.target(input)
    }
  },

  /**
   * Get the tokens
   * @return {Array}
   */
  getStaticTokens: () => TTML.tokens,

  /**
   * Add a token to be used in the macros (cannot be referenced unless given a name
   * @param {string} ref; The reference name to the token (e.g. "me" for @me)
   * @param {object} token; The token in Object notation
   * @param {object} inlineMacros; Macros to be added to the user
   * @return void
   */
  setStaticToken: (ref, token = {}, inlineMacros = {}) => {
    // Make sure our token is ready to be ingested by the parser
    const attributes = {}
    Object.keys(token).forEach((attr) => {
      const v = token[attr]
      if (typeof v !== 'object') {
        let key
        if (!isNaN(parseFloat(v)) && isFinite(v)) {
          key = v % 1 === 0 ? 'Number' : 'Float'
        } else if (typeof v !== 'boolean') {
          key = 'Text'
        } else {
          key = 'Boolean'
        }

        if (key) {
          attributes[attr] = {
            [key]: (key === 'Number' || key === 'Float') ? Number(v) : v,
          }
        }
      }
    })

    const macros = {}
    Object.keys(inlineMacros).forEach((name) => {
      macros[name] = {
        Text: inlineMacros[name]
      }
    })

    TTML.tokens[ref] = {
      attributes,
      macros,
    }
  },

  /**
   * Adds a token to be used in macros (SHOULD ONLY BE USED INTERNALLY)
   * @param {string} ref; The reference name to the token (e.g. "me" for @me)
   * @param {object} token; The token in Object notation
   * @return void
   */
  setRawStaticToken: (ref, token = {}) => {
    TTML.tokens[ref] = token
  },

  /**
   * Run the macro
   * @param {string} input
   * @return Promise<JSON|Error>
   */
  runMacro (input) {
    try {
      const rawOutput = Module.run(input, JSON.stringify(TTML.tokens))
      const output = JSON.parse(rawOutput)
      if (output.error) {
        // @todo throw specialized errors
        throw new Error(output.error.message)
      }
      return output
    } catch (e) {
      throw e // bubble the error
    }
  }
}

// Give access to the window
if (typeof window !== 'undefined') {
  window.TTML = TTML
}

/**
 * Usage:
 * 
 * ```
 * const t = await ttml.init()
 * t.runMacro('#test !r 1d20')
 * ```
 */
const init = () => wasm.initialize({
  // noExitRuntime: true,
}).then((module) => {
  // Run macros from wasm
  Module.run = module.cwrap('run_macro', 'string', [ 'string', 'string' ])

  return TTML
})

export default init
