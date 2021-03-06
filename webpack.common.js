const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require("path");

module.exports = {
  entry: "./src/index.tsx",
  output: {
    publicPath: "/",
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.[hash].js",
  },
  module: {
    rules: [
      {
        test: /.(js|jsx|ts|tsx)$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader",
        },
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: __dirname + "/public/index.html",
      filename: "index.html",
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "wasm"),
    }),
  ],
  resolve: {
    extensions: [".js", ".ts", ".jsx", ".tsx"],
  },
  experiments: {
    asyncWebAssembly: true,
  },
};
