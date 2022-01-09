const path = require('path');
const dist = path.resolve(__dirname, "dist");

const HtmlWebpackPlugin = require("html-webpack-plugin");
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
            historyApiFallback: {
                index: '/'
            },
            static: {
                directory: path.join(__dirname, "dist"),
            },
        },
        experiments: {
            asyncWebAssembly: true,
        },
        plugins: [
            new WasmPackPlugin({
                crateDirectory: __dirname
            }),
            new HtmlWebpackPlugin({
                template: path.resolve(__dirname, "static/index.hbs")
            }),
        ],
        resolve: {
            extensions: [".ts", ".js", ".wasm"],
            alias: {
              crate: __dirname
            }
        },
        module: {
            rules: [
                {
                    test: /\.hbs$/,
                    use: [
                        {
                            loader: "handlebars-loader",
                            options: {
                                rootRelative: './templates/'
                            }
                        }
                    ]
                },
            ]
        }
    }
}
