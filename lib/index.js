const wasm = require('../src/main.rs')

/**
 * Usage:
 *
 * ```
 * import { init } from 'tabletop-macro-language'
 *
 * try {
 *   const ttml = await init()
 *   const output = ttml.parseInput(input)
 * } catch (err) {
 *   console.error(err)
 * }
 * ```
 *
 * Available functions:
 *  - parseInput - Parses TTML input
 *  - 
 */
const init = () => wasm.initialize({
  noExitRuntime: true,
}).then((module) => {
  // parse ttml using rust
  const parse = module.cwrap('parse', 'string', [ 'string' ])

  function parseInput (input) {
    const rawOutput = parse(input)
    try {
      const output = JSON.parse(rawOutput)
      if (output.error) {
        // @todo throw specialized errors
        throw new Error(output.error.message)
      }
      return output
    } catch (e) {
      // Bubble the error
      throw e
    }
  }

  return {
    parseInput,
  }
})

module.exports.init = init

init().then(({ parseInput }) => {
  console.log(parseInput('#hello'))
})
