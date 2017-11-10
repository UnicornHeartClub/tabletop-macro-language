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
  const run = module.cwrap('run_macro', 'string', [ 'string' ])

  class TTML {
    /**
     * Initialize the class
     */
    constructor () {
      // memoize any macros we receive to reference in other macros
      this.macros = {}

      // tokens to be used in our macros
      this.tokens = {}

      // reserved token names, referenced by id
      this.reserved = {
        me: undefined,
        selected: undefined,
      }
    }

    /**
     * Add a token to be used in the macros (cannot be referenced unless given a name
     * @param {string|undefined} id; The token ID
     * @param {object} token; The token in Object notation
     * @param {string} name; The static name of the token
     * @return void
     */
    addToken (id, token = {}, name = undefined) {
      this.tokens[id] = token
      if (name) {
        this.reserved[name] = id
      }
    }

    /**
     * Updates the name of the reserved token
     * @param {string} name; The static name of the token
     * @param {string|undefined} id; The token ID
     * @return void
     */
    setStaticTokenName (name, id = undefined) {
      this.reserved[name] = id
    }

    /**
     * Run the macro
     * @param {string} input
     * @return Promise<JSON|Error>
     */
    runMacro (input) {
      try {
        const rawOutput = run(input)
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
