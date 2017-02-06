# `node-fonttools`

> Native bindings to [`fonttools`][1] to decompile and compile fonts.

## How It Works

[`fonttools`][1] (also known as TTX) is a library for manipulating fonts,
written in Python.  It supports TrueType, OpenType, AFM and to an extent Type 1,
and some Mac-specific formats. Using [Addons][2] it's possible to bridge Python
to Node, natively, using C++ and Python's [Python/C API][3]&mdash;which is
exactly what `node-fontools` does.

## Current Status

So far, the only functions that have been bridged are the abilities to decompile
fonts to XML and compile XML to font binary.

## How To Use

```js
import fonttools from 'fonttools';
import { readFileSync } from 'fs';

// Get the `decompile` and `compile` methods
const { decompile, compile } = fonttools();

// Read a font file as a buffer
const fontBuffer = readFileSync('font.ttf');

// Decompile the font to another buffer
const fontXMLBuffer = decompile(fontBuffer);

// Insert logic here to manipulate font

// Compile font from an XML file as a buffer
const fontBinaryBuffer = compile(fontXMLBuffer);

// Insert logic to save font file buffer here
```

### `fonttools`

A function that takes a path to [`fonttools/Lib`][4] and returns an object with
two methods: `decompile` and `compile`. By default it uses the [included
submodule][5].

#### `decompile`

Takes a font file buffer and returns another buffer with the XML of the
decompiled font.

#### `compile`

Takes an XML file buffer and returns another buffer with the compiled font file
binary.

> More on [Node Buffers][6]

[1]: https://github.com/fonttools/fonttools
[2]: https://nodejs.org/api/addons.html
[3]: https://docs.python.org/2/c-api/
[4]: https://github.com/fonttools/fonttools/tree/master/Lib
[5]: https://github.com/dfrankland/node-fonttools/blob/master/.gitmodules
[6]: https://nodejs.org/api/buffer.html
