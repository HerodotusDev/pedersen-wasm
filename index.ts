const { pedersen } = require("./pkg/pedersen_wasm.js");

// console.log(
//   `pedersen hash: '${pedersen(
//     "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
//     "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
//   )}'`
// );

const before = Date.now();

for (let i = 0; i < 100; ++i) {
  pedersen(
    "03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb",
    "0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a"
  );
}

const after = Date.now();

console.log("Elapsed time:", after - before, "ms");
