/* eslint-disable */
   
'use strict';
const common = require('./webpack.common');
const { merge } = require('webpack-merge');
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');

module.exports = merge(common, {
  mode: `production`,
  devtool: false,
  stats: {
    errorDetails: true,
  },
  plugins: [new ForkTsCheckerWebpackPlugin()],
});