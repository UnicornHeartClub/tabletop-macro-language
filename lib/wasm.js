mergeInto(LibraryManager.library, {
  prompt: function(x) {
    var str = window.prompt(Pointer_stringify(x));
    var len = lengthBytesUTF8(str);
    var buffer = Module._malloc(len);
    Module.stringToUTF8(str, buffer, len);

    return buffer;
  },
});
