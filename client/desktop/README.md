# JavaScript and TypeScript

This project uses TypeScript for JavaScript that does not need to be written
inline in QML.

## Installing dependencies

To install the necessary dependencies, run:

```bash
$ npm install
```

from the `heraldqtDesktop` root directory.

## Configuration files

The compiler options for the TypeScript compiler are specified in
`tsconfig.json`. `tsc` will run in strict mode targeting ES2016 (i.e., "ES7") and
generate corresponding declaration files.

An `eslint` configuration for TypeScript is also included in the file
`.eslintrc.js`.

## Naming conventions

When possible, include type guards to verifying that the types are correct a
runtime. Functions with type guards should be prefixed by `safe`. For example,
if `myFunction` has type guards, it should be called `safeMyFunction`.

## Compiling TypeScript files

This is currently requires running a custom build script because TypeScript
[currently cannot emit `.mjs` files](https://github.com/microsoft/TypeScript/issues/18442).

You will need to explicitly add new TypeScript and TypeScript declaration files
by modifying `tsconfig.json`.

To compile the files, rename them, and add new files to `qml.qrc` run:

```bash
$ ../scripts/run_tsc
```

from the `desktop` root directory.

Now simply [import the modules into QML](https://doc.qt.io/qt-5/qtqml-javascript-imports.html).

# Renaming files

To rename QML resource files, you can use the `qmv` script.

Its usage is as follows:

```bash
../scripts/qmv SOURCE DEST QRC_PATH
```

Where `SOURCE` is the file or directory to be renamed, `DEST` is the name new, and `QRC_PATH` is
the path to the `qrc` file that should be updated. This will move the directories and update
the `qrc` file.
