const wasm = require('../src/main.rs')

/**
 * Usage:
 *
 * Available functions:
 *  - parseInput - Parses TTML input
 *  - 
 */
const init = () => wasm.initialize({
  noExitRuntime: true,
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

      // reserved token names, referenced by id
      this.tokens = {
        // me: undefined,
        // selected: undefined,
      }
    }

    /**
     * Add a token to be used in the macros (cannot be referenced unless given a name
     * @param {string} ref; The reference name to the token (e.g. "me" for @me)
     * @param {object} token; The token in Object notation
     * @return void
     */
    setStaticToken (ref, token = {}) {
      // Make sure our token is ready to be ingested by the parser
      const attrs = Object.keys(token).map((attr) => ({
        attribute: attr,
        value: token[attr],
      }))
      this.tokens[ref] = attrs
    }

    /**
     * Run the macro
     * @param {string} input
     * @return Promise<JSON|Error>
     */
    runMacro (input) {
      try {
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

module.exports.init = init
global.window.ttml = {
  init,
}
