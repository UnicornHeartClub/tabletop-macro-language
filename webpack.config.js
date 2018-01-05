const path = require('path');
const webpack = require('webpack');

 module.exports = {
   entry: `${__dirname}/index.js`,
   output: {
     filename: 'bundle.js',
     libraryTarget: 'commonjs2',
     path: __dirname + '/dist',
   },

   module: {
     rules: [
       {
         test: /\.rs$/,
         use: {
           loader: 'rust-wasm-loader',
           options: {
             args: '--features web',
             wasmBinaryFile: '/macro.wasm',
             release: true,
             path: 'dist',
           },
         },
       },
       {
         test: /\.js$/,
         exclude: /(node_modules)/,
         use: {
           loader: 'babel-loader',
           options: {
             presets: ['env'],
           },
         },
       },
     ],
   },

   externals: {
     'fs': true,
     'path': true,
     'window': 'window',
   },

   // devtool: 'source-map',
   target: 'web',
 };
