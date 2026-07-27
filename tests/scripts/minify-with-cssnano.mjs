import fs from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(scriptDirectory, "../..");
const cssnanoRoot = process.env.CSSNANO_DIR
  ? path.resolve(process.env.CSSNANO_DIR)
  : path.resolve(repositoryRoot, "../cssnano");
const [inputArgument, outputArgument] = process.argv.slice(2);

if (!inputArgument || !outputArgument) {
  throw new Error(
    "usage: node tests/scripts/minify-with-cssnano.mjs <input.css> <output.css>",
  );
}

const input = path.resolve(inputArgument);
const output = path.resolve(outputArgument);
const cssnanoModule = path.join(cssnanoRoot, "packages/cssnano/src/index.js");
const { default: cssnano } = await import(pathToFileURL(cssnanoModule));
const source = fs.readFileSync(input, "utf8");
const result = await cssnano({ preset: "default" }).process(source, {
  from: input,
  to: output,
});

fs.writeFileSync(output, `${result.css}\n`);
