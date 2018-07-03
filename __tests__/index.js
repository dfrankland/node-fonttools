import { readFileSync } from 'fs';
import { resolve as resolvePath } from 'path';
import fastXmlParser from 'fast-xml-parser';
import { decompile } from '..';

const ttf = readFileSync((
  resolvePath(__dirname, './PlayfairDisplay-Regular.ttf')
));

describe('fonttools', () => {
  it('can decompile TTF fonts to XML', () => {
    const ttx = decompile(ttf).toString('utf8');
    expect(fastXmlParser.validate(ttx)).toBe(true);
  });
});
