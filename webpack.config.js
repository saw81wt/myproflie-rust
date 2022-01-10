const path = require("path");
const dist = path.resolve(__dirname, "dist");

const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const WebpackBar = require("webpackbar");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");

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
    ignoreWarnings: [
      (warning) =>
        warning.message ===
        "Critical dependency: the request of a dependency is an expression",
    ],
    devtool: "eval",
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
      new WebpackBar(),
      new CleanWebpackPlugin(),
      new WasmPackPlugin({
        crateDirectory: __dirname
      }),
      new HtmlWebpackPlugin({
        template: path.resolve(__dirname, "static/index.hbs")
      }),
      new MiniCssExtractPlugin({
        filename: '[name].[contenthash].css'
      }),
      new CopyWebpackPlugin({
        patterns: [
          {
            from: "static",
            to: "static"
          },
        ]
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
        {
          test: /\.ts$/,
          use: [
            {
              loader: "ts-loader",
              options: {
                configFile: "tsconfig.json"
              }
            }
          ]
        },
        {
          test: /\.css$/,
          use: [
            MiniCssExtractPlugin.loader,
            "css-loader",
            "postcss-loader",
          ]
        }
      ]
    }
  }
}
