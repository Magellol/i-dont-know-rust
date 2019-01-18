const module = import('./wasm-pkg/lib_wasm');

module.then(rust => {
  rust.greet('World!');
});
