import fs from "node:fs";
import path from "node:path";
import readline from "node:readline";
import { fileURLToPath, pathToFileURL } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const cssnanoDir = process.env.CSSNANO_DIR
  ? path.resolve(process.env.CSSNANO_DIR)
  : path.resolve(scriptDir, "../../../../cssnano");
const cssnanoPath = path.join(cssnanoDir, "packages/cssnano/src/index.js");
const { default: cssnano } = await import(pathToFileURL(cssnanoPath));
const source = fs.readFileSync(process.argv[2], "utf8");
const processor = cssnano({ preset: "default" });
const lines = readline.createInterface({
  input: process.stdin,
  crlfDelay: Infinity,
});

process.stdout.write("ready\n");

lines.on("line", async (line) => {
  try {
    const iterations = Number(line);
    if (!Number.isSafeInteger(iterations) || iterations < 1) {
      throw new Error(`invalid iteration count: ${line}`);
    }

    let outputLength = 0;
    const start = process.hrtime.bigint();
    for (let index = 0; index < iterations; index++) {
      const result = await processor.process(source, { from: undefined });
      outputLength = result.css.length;
    }
    const elapsed = process.hrtime.bigint() - start;
    process.stdout.write(`${elapsed} ${outputLength}\n`);
  } catch (error) {
    console.error(error);
    process.exitCode = 1;
    lines.close();
  }
});
