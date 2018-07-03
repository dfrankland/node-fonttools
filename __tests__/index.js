const { readFileSync } = require('fs');
const { resolve: resolvePath } = require('path');
const { decompile } = require('..');

const ttf = readFileSync(resolvePath(__dirname, './PlayfairDisplay-Regular.ttf'));

describe('fonttools', () => {
  it('can decompile TTF fonts to XML', () => {
    const ttx = decompile(ttf).toString('utf8');
    expect(typeof ttx).toBe('string');
  });
});
