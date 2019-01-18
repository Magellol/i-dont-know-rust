const module = import('./wasm-pkg/lib_wasm');

module.then(rust => {
  const r = rust.sum(new Uint32Array([1, 2]));

  console.log(r);
});
