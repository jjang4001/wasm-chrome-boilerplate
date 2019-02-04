const path = require('path');
const CopyWebpackPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist/js"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "crate")
    }),
    new CopyWebpackPlugin(['index.html', 'style.css'])
  ],
};
