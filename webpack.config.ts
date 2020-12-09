import WasmPackPlugin from '@wasm-tool/wasm-pack-plugin';
import CopyPlugin from 'copy-webpack-plugin';
import HtmlWebPackPlugin from 'html-webpack-plugin';
import * as path from 'path';
import webpack from 'webpack';

const dist = path.resolve(__dirname, 'dist');
const packer = (
  env: { [key: string]: string | number | boolean },
  args: { mode: webpack.Configuration['mode'] }
): webpack.Configuration & { [key: string]: any } => {
  const isProd = args.mode === 'production';
  console.log('dist:', dist);
  return {
    mode: isProd ? 'production' : 'development',
    module: {
      rules: [{ test: /\.ts?$/, use: 'ts-loader', exclude: /node_modules/ }],
    },
    resolve: {
      extensions: ['.ts', '.tsx', '.js', '.json'],
    },
    entry: {
      index: './js/index.ts',
    },
    output: {
      path: dist,
      filename: isProd ? '[name].[contenthash].js' : '[name].[contenthash].js',
    },
    devServer: {
      port: 3000,
      contentBase: dist,
      hot: true,
    },
    plugins: [
      new HtmlWebPackPlugin({ template: 'static/index.html' }),
      new CopyPlugin([path.resolve(__dirname, 'static')] as any),
      new WasmPackPlugin({
        crateDirectory: __dirname,
      }) as any,
    ],
    experiments: {
      asyncWebAssembly: true,
    },
  };
};
module.exports = packer;
