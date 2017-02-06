import { resolve as resolvePath } from 'path';
import fonttools from './Release/fonttools.node';

export const DEFAULT_FONTTOOLS = resolvePath(__dirname, './fonttools/Lib');

export default (fonttoolsPath = DEFAULT_FONTTOOLS) => (
  fonttools(fonttoolsPath)
);
