Experimental source map parser built in Rust using [sourcemap](https://docs.rs/sourcemap/6.0.1/sourcemap/) crate and compiled to native Node binary using [Neon bindings](https://neon-bindings.com/).

This package is built for benchmarking purpose and is not actively supported, for most use cases you should be using [source-map](https://www.npmjs.com/package/source-map), which has part of it built also in Rust and compiled to WASM. However, since loading WASM module and warming up JS and WASM code (JIT) takes time you may want to consider alternatives. Since this library does not suffer from load times and does not need to wait for JIT to optimize its code, you can expect up 4 times better performance on colds starts, but once [source-map](https://www.npmjs.com/package/source-map) becomes warmed up (usually 1 parsing run is sufficient) it will surpass this library's performance in factor of 2 to 3 times.

# Installation:
`npm install git+https://github.com/ErkoKnoll/node-source-map-parser-native.git`

or

`yarn add git+https://github.com/ErkoKnoll/node-source-map-parser-native.git`

You also need Rust compiler which you can install by following these [instructions](https://doc.rust-lang.org/book/ch01-01-installation.html). 

On Amazon Linux 2 you can use `sudo amazon-linux-extras install rust1`.

# Usage:
```
import { parseSourceMap } from "node-source-map-parser-native";

const file = readFileSync("file.js.map").toString("utf-8");

// Parse the file and get back a reference to parsed source map for later use
const sourceMap = parseSourceMap(file);

// Lookup original position
const { source, line, column } = sourceMap.lookupOriginalPosition(1, 2);

// Print out original source, line and column
console.log(source, line, column)

// Dispose parsed source map resources, if not done then you will leak memory
sourceMap.dispose();
```