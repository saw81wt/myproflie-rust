const path = require('path');
const dist = path.resolve(__dirname, "dist");

const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = (env, argv) => {
    return {
        performance: {
            // Don't break compilation because of WASM file bigger than 244 KB.
            hints: false
        },
        entry: {
            app: path.resolve(__dirname, "index.ts")
        },
        output: {
            publicPath: "/",
            path: dist,
            filename: "[name].[contenthash].js"
        },
        devServer: {
            host: "0.0.0.0",
            port: 8000,
            // Route everything to index to support SPA. It should be the same like `publicPath` above.
            historyApiFallback: {
                index: '/'
            }
        },
        experiments: {
            asyncWebAssembly: true,
        },
        plugins: [
            new WasmPackPlugin({
                crateDirectory: __dirname
            }),
        ],
        resolve: {
            extensions: [".ts", ".js", ".wasm"],
            alias: {
              crate: __dirname
            }
        },
    }
}
