const FontTools = require('./native/index.node').default;

// NOTE: `require` can resolve weiredly if using Jest or other environments. To
// help, we give a reference to `require` to the native module.
const fonttools = new FontTools(require, module);

module.exports = {
  decompile: (...args) => fonttools.decompile(...args),
  compile: (...args) => fonttools.compile(...args),
};
