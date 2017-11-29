const wasm = require('../src/main.rs')

/**
 * Usage:
 *
 * Available functions:
 *  - parseInput - Parses TTML input
 *  - 
 */
const init = () => wasm.initialize({
  // noExitRuntime: true,
}).then((module) => {
  // parse ttml using rust
  const run = module.cwrap('run_macro', 'string', [ 'string', 'string' ])

  class TTML {
    /**
     * Initialize the class
     */
    constructor () {
      // memoize any macros we receive to reference in other macros
      this.macros = {}

      // reserved token names used by the module
      this.tokens = {}
    }

    /**
     * Get the tokens
     *
     * @return {Array}
     */
    getStaticTokens () {
      return this.tokens;
    }

    /**
     * Add a token to be used in the macros (cannot be referenced unless given a name
     * @param {string} ref; The reference name to the token (e.g. "me" for @me)
     * @param {object} token; The token in Object notation
     * @return void
     */
    setStaticToken (ref, token = {}) {
      // Make sure our token is ready to be ingested by the parser
      const attributes = {}
      Object.keys(token).forEach((attr) => {
        const v = token[attr]
        if (typeof v !== 'object') {
          let key
          if (!isNaN(parseFloat(v)) && isFinite(v)) {
            key = 'Number'
          } else if (typeof v !== 'boolean') {
            key = 'Text'
          }

          if (key) {
            attributes[attr] = {
              [key]: key === 'Number' ? Number(v) : v,
            }
          }
        }
      })

      this.tokens[ref] = {
        attributes,
      }
    }

    /**
     * Adds a token to be used in macros (SHOULD ONLY BE USED INTERNALLY)
     * @param {string} ref; The reference name to the token (e.g. "me" for @me)
     * @param {object} token; The token in Object notation
     * @return void
     */
    setRawStaticToken (ref, token = {}) {
      this.tokens[ref] = token
    }

    /**
     * Run the macro
     * @param {string} input
     * @return Promise<JSON|Error>
     */
    runMacro (input) {
      try {
        console.log(this.tokens)
        const rawOutput = run(input, JSON.stringify(this.tokens))
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

  return new TTML()
})

export default init
