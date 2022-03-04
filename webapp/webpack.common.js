const path = require('path');
const webpack = require('webpack');
const { merge } = require('webpack-merge');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const CopyPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');
const {InjectManifest} = require('workbox-webpack-plugin');

module.exports = {
  entry: path.resolve(__dirname, './src/index.js'),
  module: {
    rules: [
      { test: /\.txt$/, use: 'raw-loader' },
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
    ],
  },
  plugins: [
    new CleanWebpackPlugin(),
    new CopyPlugin({
      patterns: [
        { from: "src/assets", to: "assets" },
        { from: "src/manifest.webmanifest", to: "." },
      ],
    }),
    new HtmlWebpackPlugin({
      title: "twenty48",
      template: "src/index.html"
    }),
    new InjectManifest({
      swSrc: '/src/service-worker.js',
    }),
    new webpack.ProgressPlugin(),
  ],
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].[fullhash].js',
  },
  experiments: {
    asyncWebAssembly: true
  },
  ignoreWarnings: [
    {
      module: /engine_bg\.js/,
      message: /the request of a dependency is an expression/,
    },
  ],
};
