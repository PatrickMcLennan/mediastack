/* eslint-disable */

'use strict';

const path = require('path');
const nodeExternals = require('webpack-node-externals');
const TerserPlugin = require('terser-webpack-plugin');

module.exports = {
  target: 'node',
  entry: {
    stack: path.resolve(__dirname, `stack.ts`),
  },
  externals: {
    'aws-sdk': 'aws-sdk',
    '/tmp/node_modules/aws-sdk': 'aws-sdk',
    ...nodeExternals(),
  },
  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, './'),
    libraryTarget: 'commonjs',
  },

  resolve: {
    extensions: ['.ts', '.ts', '.js', '.mjs'],
  },
  module: {
    rules: [
      {
        test: /\.ts?$/,
        loader: 'swc-loader',
      },
    ],
  },
  optimization: {
    minimizer: [new TerserPlugin({
      extractComments: false
    })]
  }
};