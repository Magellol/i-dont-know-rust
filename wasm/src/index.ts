(async wasmModule => {
  const rustModule = await wasmModule;

  const input = [1, 2];
  const result = rustModule.sum(new Uint32Array(input));

  document.body.textContent = `Result of ${input.join(' + ')} is ${result}.`;
})(import('./wasm-pkg/lib_wasm'));
