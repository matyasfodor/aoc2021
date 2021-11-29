const { merge } = require("webpack-merge");

const common = require("./webpack.common.js");

module.exports = merge(common, {
  devServer: {
    compress: true,
    port: 8080,
    hot: true,
    static: "./dist",
    historyApiFallback: true,
    open: true,
  },
  mode: "development",
  devtool: "inline-source-map",
});
