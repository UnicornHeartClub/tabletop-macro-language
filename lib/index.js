import {
  fetchAndInstantiate,
  copyCStr,
  newString,
} from './wasm'

import Executor, { buildToken, unbuildToken, execute } from './executor'

// initialize
const executor = Executor({})

// Private module to run wasm functions from TTML
const Module = {}

// TableTop Macro Language Object
export const TTML = {
  /**
   * Add callback action for input method
   * @param {Function<Promise>} action; The action to be run when input is called
   * @return void
   */
  addInputAction: (action = (input) => {}) => {
    executor.input = action
    return executor.input
  },

  /**
   * Add callback action for prompt method
   * @param {Function<Promise>} action; The action to be run when prompt is called
   * @return void
   */
  addPromptAction: (action = (input, options) => {}) => {
    executor.prompt = action
    return executor.prompt
  },

  /**
   * Add callback action for target method
   * @param {Function<Promise>} action; The action to be run when target is called
   * @return void
   */
  addTargetAction: (action = (input) => {}) => {
    executor.target = action
    return executor.target
  },

  /**
   * Remove all tokens
   * @return {void}
   */
  clearTokens: () => {
    executor.tokens = {}
  },

  /**
   * Execute a returned program
   * @param {Object} program; The complete program output
   * @return {Promise}
   */
  execute: (steps = []) => {
    return execute(steps)
  },

  /**
   * Get the tokens
   * @return {Array}
   */
  getRawTokens: () => executor.tokens,

  /**
   * Parse a complete macro
   * @param {String} input; The macro input
   * @return {String}
   * @throws {Error}
   */
  parse: (input) => {
    const input_buffer = newString(Module, input)
    const output_ptr = Module.parse(input_buffer)
    const result = copyCStr(Module, output_ptr)
    Module.dealloc(input_buffer)
    Module.dealloc(output_ptr)
    try {
      return JSON.parse(result)
    } catch (error) {
      throw new Error('There was a problem parsing the output of the program')
    }
  },

  unbuildToken (token) {
    return unbuildToken(token)
  },

  /**
   * Add a token to be used in the macros
   * @param {string} ref; The reference name to the token (e.g. "me" for @me)
   * @param {Object|Array<Object>} token; The token, also accepts a list of tokens (e.g. "selected")
   * @param {Object} inlineMacros; Macros to be added to the given token(s)
   * @return {Object}
   */
  setToken: (ref, token = {}, inlineMacros = {}) => {
    // Make sure our token is ready to be ingested by the parser
    const attributes = buildToken(token)

    const macros = {}
    Object.keys(inlineMacros).forEach((name) => {
      macros[name] = {
        Text: inlineMacros[name]
      }
    })

    executor.tokens[ref] = {
      attributes,
      macros,
    }

    return executor.tokens[ref]
  },

  /**
   * Adds a "static" token, a token that is not actually set but can reference another real token
   * @param {String} id; The ID of the static token
   * @param {String} ref; The ID of the referenced token
   * @return {void}
   */
  setStaticToken (id, ref) {
    if (executor.tokens[ref]) {
      executor.tokens[id] = executor.tokens[ref]
    } else if (Array.isArray(ref)) {
      executor.tokens[id] = ref.filter(r => executor.tokens[r]).map(r => executor.tokens[r])
    }
  },

  /**
   * Adds a token to be used in macros (SHOULD ONLY BE USED INTERNALLY)
   * @param {string} ref; The reference name to the token (e.g. "me" for @me)
   * @param {Object} token; The token in Object notation
   * @return void
   */
  setRawToken: (ref, token = {}) => {
    executor.tokens[ref] = token
    return executor.tokens[ref]
  },
}

const init = async (opts = {}) => {
  const options = Object.assign({}, {
    wasm: '/ttml.wasm',
  }, opts)

  const mod = await fetchAndInstantiate(options.wasm, {})
  Module.alloc   = mod.exports.alloc;
  Module.dealloc = mod.exports.dealloc;
  Module.dealloc_str = mod.exports.dealloc_str;
  Module.memory  = mod.exports.memory;

  // Set the parse method
  Module.parse  = mod.exports.parse;

  return TTML
}

executor.parse = TTML.parse

export default init 
