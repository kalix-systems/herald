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

This is currently a two step process because the TypeScript compiler
[currently cannot emit `.mjs` files](https://github.com/microsoft/TypeScript/issues/18442).

To compile the file, run:

```bash
$ tsc file.ts
```

where `file.ts` is the TypeScript file to be compiled.

This will produce a file named `file.js`. Now simply rename this to `file.mjs`
and [import the module into the QML document](https://doc.qt.io/qt-5/qtqml-javascript-imports.html).
