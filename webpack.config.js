const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = (env, args) => {
  const isProduction = (args.mode === 'production');
  return {
    mode: "development",
    entry: {
      index: "./js/bootstrap.js",
    },
    output: {
      path: dist,
      filename: "[name].js"
      //filename: isProduction ? '[name].[contenthash].js' : '[name].[hash].js',
    },
    //entry: './index.js',

    experiments: {
      asyncWebAssembly: true,
    },
    plugins: [
      new CopyPlugin({
        patterns: [
          { from: path.resolve(__dirname, "static"), to: dist },
        ]
      }),
      new WasmPackPlugin({
        outName: 'webgfx',
        crateDirectory: __dirname,
      }),
    ]
  };
}
