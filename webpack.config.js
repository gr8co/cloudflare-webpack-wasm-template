const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: {
    index: "./src/worker.js"
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
};