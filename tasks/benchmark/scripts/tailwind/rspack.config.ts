import path from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig } from "@rspack/cli";
import { Compilation, type Compiler, sources } from "@rspack/core";

const projectDirectory = path.dirname(fileURLToPath(import.meta.url));
const fixtureDirectory = path.resolve(projectDirectory, "../..", "files");

class CssOnlyOutputPlugin {
  apply(compiler: Compiler) {
    compiler.hooks.thisCompilation.tap("CssOnlyOutputPlugin", (compilation) => {
      compilation.hooks.processAssets.tap(
        {
          name: "CssOnlyOutputPlugin",
          stage: Compilation.PROCESS_ASSETS_STAGE_SUMMARIZE,
        },
        () => {
          for (const asset of compilation.getAssets()) {
            if (asset.name.endsWith(".js")) {
              compilation.deleteAsset(asset.name);
            } else if (asset.name.endsWith(".css")) {
              const css = asset.source.source().toString().trimEnd();
              compilation.updateAsset(
                asset.name,
                new sources.RawSource(`${css}\n`),
              );
            }
          }
        },
      );
    });
  }
}

export default defineConfig({
  mode: "production",
  entry: {
    tailwind: "./src/index.css",
  },
  context: projectDirectory,
  devtool: false,
  experiments: {
    css: true,
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        type: "css",
        use: ["postcss-loader"],
      },
    ],
  },
  optimization: {
    minimize: false,
  },
  performance: {
    hints: false,
  },
  output: {
    path: fixtureDirectory,
    filename: "[name].js",
    cssFilename: "[name].css",
    clean: false,
  },
  plugins: [new CssOnlyOutputPlugin()],
});
