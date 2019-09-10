# JavaScript and TypeScript

This project uses TypeScript for JavaScript that does not need to be written
inline in QML.

## Installing dependencies

To install the necessary dependencies, run:

```bash
$ npm install
```

from the `heraldqt` root directory.

## Configuration files

The compiler options for the TypeScript compiler are specified in
`tsconfig.json`. `tsc` will run in strict mode targeting ES7/ES2019 and
generate corresponding declaration files.

An `eslint` configuration for TypeScript is also included in the file
`.eslintrc.js`.

## Compiling TypeScript files

This is currently requires running a custom build script because TypeScript 
[currently cannot emit `.mjs` files](https://github.com/microsoft/TypeScript/issues/18442).

To compile the files, rename them, and add new files to `qml.qrc` run:

```bash
$ ./run_tsc.py
```

from the `heraldqt` root directory.

Now simply [import the modules into QML](https://doc.qt.io/qt-5/qtqml-javascript-imports.html).
