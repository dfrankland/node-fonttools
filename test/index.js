const test = require('tape');
const { resolve: resolvePath } = require('path');
const { readFileSync, writeFileSync, unlinkSync } = require('fs');
const { default: fonttools, DEFAULT_FONTTOOLS } = require('../dist/index');

const paths = {
  fonttools: resolvePath(__dirname, '../fonttools/Lib'),
  src: resolvePath(__dirname, './PlayfairDisplay-Regular.ttf'),
  dist: {
    decompiled: resolvePath(__dirname, './test.ttx'),
    compiled: resolvePath(__dirname, './test.ttf'),
  },
};

const isGoodBuffer = buffer => (
  buffer instanceof Buffer &&
  buffer.length > 0
);

const isGoodFileWrite = (path, buffer) => {
  let fileWriteResult = false;
  try {
    writeFileSync(path, buffer);
    unlinkSync(path);
    fileWriteResult = true;
  } catch (e) {
    // do nothing
  }

  return fileWriteResult;
};

test('Decompile and Compile Fonts', t => {
  t.plan(7);

  t.comment('default fonttools path is set properly');
  t.ok(DEFAULT_FONTTOOLS === paths.fonttools);

  t.comment('`decompile` and `compile` are methods');
  const { decompile, compile } = fonttools();
  t.ok(typeof decompile === 'function');
  t.ok(typeof compile === 'function');

  t.comment('`decompile` takes a font file buffer and returns an XML file buffer');
  const fontXMLBuffer = decompile(readFileSync(paths.src));
  t.ok(isGoodBuffer(fontXMLBuffer));

  t.comment('XML file buffer can be written to a file');
  t.ok(isGoodFileWrite(paths.dist.decompiled, fontXMLBuffer));

  t.comment('`compile` takes an XML file buffer and returns a font file buffer');
  const fontBinaryBuffer = compile(fontXMLBuffer);
  t.ok(isGoodBuffer(fontBinaryBuffer));

  t.comment('font file buffer can be written to a file');
  t.ok(isGoodFileWrite(paths.dist.compiled, fontBinaryBuffer));
});
