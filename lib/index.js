import {
  fetchAndInstantiate,
  copyCStr,
  newString,
} from './wasm'

// Private module to run wasm functions from TTML
const Module = {}

// TableTop Macro Language Object
const TTML = {
  parse: (input) => {
    const input_buffer = newString(Module, input)
    const output_ptr = Module.parse(input_buffer)
    const result = copyCStr(Module, output_ptr)
    Module.dealloc_str(input_buffer)
    Module.dealloc_str(output_ptr)
    try {
      return JSON.parse(result)
    } catch (error) {
      console.error(error)
      throw new Error('There was a problem parsing the output of the program')
    }
  }
}

const init = async () => {
  const mod = await fetchAndInstantiate("/ttml.wasm", {})
  Module.alloc   = mod.exports.alloc;
  Module.dealloc_str = mod.exports.dealloc_str;
  Module.memory  = mod.exports.memory;
  Module.parse  = mod.exports.parse;

  return TTML
}

export default init 
