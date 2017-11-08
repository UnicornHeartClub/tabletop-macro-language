const path = require('path');
const webpack = require('webpack');

 module.exports = {
   entry: `${__dirname}/lib/index.js`,
   output: {
     filename: 'bundle.js',
     path: __dirname + '/dist',
   },

   module: {
     rules: [
       {
         test: /\.rs$/,
         use: {
           loader: 'rust-wasm-loader',
           options: {
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
   },

   devtool: 'source-map',
 };
