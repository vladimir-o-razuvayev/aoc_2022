const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: {
    'index': './index.js',
    'day-01': './src/puzzles/day_01/index.js',
    'day-12': './day-12.js'
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].js',
    clean: true
  },
  module: {
    rules: [
      {
        test: /\.txt/,
        use: 'raw-loader'
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      filename: 'index.html',
      chunks: ['index'],
      template: 'index.html',
      title: 'Advent of Code 2022'
    }),
    new HtmlWebpackPlugin({
      filename: 'day-12.html',
      chunks: ['day-12'],
      template: 'day.html',
      title: 'Day 12'
    }),
    new HtmlWebpackPlugin({
      filename: 'day-01.html',
      chunks: ['day-01'],
      template: 'day.html',
      title: 'Day 1'
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, ".")
    }),
    // Have this example work in Edge which doesn't ship `TextEncoder` or
    // `TextDecoder` at this time.
    new webpack.ProvidePlugin({
      TextDecoder: ['text-encoding', 'TextDecoder'],
      TextEncoder: ['text-encoding', 'TextEncoder']
    })
  ],
  mode: 'development',
  experiments: {
    asyncWebAssembly: true
  }
};