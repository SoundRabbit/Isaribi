const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackInlineSourcePlugin = require("html-webpack-inline-source-plugin");

module.exports = {
    entry: "./assets",
    output: {
        path: path.join(__dirname, "./dist")
    },
    resolve: {
        extensions: [".js"]
    },
    module: {},
    plugins: [
        new HtmlWebpackPlugin({
            template: path.join(__dirname, "./assets/index.html"),
            inlineSource: ".(js|css)$"
        }),
        new WasmPackPlugin({
            crateDirectory: path.join(__dirname, "./"),
            forceMode: "release",
            target: "web",
        }),
        new HtmlWebpackInlineSourcePlugin(),
    ],
    devServer: {
        historyApiFallback: true,
    }
};