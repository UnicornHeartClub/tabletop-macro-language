const path = require('path');
const webpack = require('webpack');
const UglifyJsPlugin = require('uglifyjs-webpack-plugin')

 module.exports = {
   entry: ['babel-polyfill', `${__dirname}/index.js` ],
   output: {
     filename: 'bundle.js',
     libraryTarget: 'commonjs2',
     path: __dirname + '/dist',
   },

   module: {
     rules: [
       {
         test: /\.js$/,
         exclude: /(node_modules)/,
         use: {
           loader: 'babel-loader',
           options: {
             presets: ['@babel/preset-env'],
           },
         },
       },
     ],
   },

   plugins: [
     new UglifyJsPlugin({
       uglifyOptions: {
         ie8: false,
         emca: 8,
         output: {
           beautify: false,
         },
       },
     }),
   ],

   externals: {
     'fs': true,
     'path': true,
     'window': 'window',
   },

   // devtool: 'source-map',
   target: 'web',
 };
