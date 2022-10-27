import * as pkg from "./pkg/rsa_accumulators_wasm.js"

const before = performance.now();
console.log(pkg.genSafePrime(1024))
console.log(performance.now() - before)
