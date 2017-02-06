import webpack from 'webpack';
import UglifyJSPlugin from 'uglifyjs-webpack-plugin';
import { resolve as resolvePath } from 'path';

const dir = {
  src: resolvePath(__dirname, './src'),
  dist: resolvePath(__dirname, './dist'),
};

export default {
  context: dir.src,
  entry: dir.src,
  output: {
    path: dir.dist,
    filename: 'index.js',
    libraryTarget: 'commonjs2',
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        include: dir.src,
        use: [
          {
            loader: 'babel-loader',
            options: {
              babelrc: false,
              presets: [
                [
                  'env',
                  {
                    modules: false,
                    targets: {
                      node: 4,
                    },
                  },
                ],
                'stage-0',
                'babili',
              ],
            },
          },
        ],
      },
    ],
  },
  externals: ['../build/Release/fonttools.node'],
  plugins: [
    new webpack.optimize.LimitChunkCountPlugin({ maxChunks: 1 }),
    new webpack.BannerPlugin({
      banner: 'require("source-map-support").install();',
      options: { raw: true, entryOnly: false },
    }),
    new UglifyJSPlugin(),
  ],
  target: 'node',
  node: {
    __dirname: false,
  },
};
