const path = require('path');

const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
  entry: {
    popup: path.join(__dirname, '../src/popup/bootstrap.js'),
    background: path.join(__dirname, '../src/background/background.ts'),
    content: path.join(__dirname, '../src/content/content.ts'),
  },
  output: {
    path: path.join(__dirname, '../dist/js'),
    filename: '[name].js',
  },
  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.wasm'],
  },
  module: {
    rules: [
      {
        test: /\.(ts|tsx)$/,
        loader: 'ts-loader',
      },
      {
        enforce: 'pre',
        test: /\.js$/,
        loader: 'source-map-loader',
      },
      {
        test: /\.scss$/,
        use: [
          'style-loader',
          {
            loader: 'css-loader',
            options: {
              minimize: true,
            }
          },
          'sass-loader?sourceMap',
        ],
      },
    ]
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.join(__dirname, '../crate'),
    }),
    new HtmlWebpackPlugin({
      template: path.join(__dirname, '../src/popup/index.html'),
    }),
  ],
};
