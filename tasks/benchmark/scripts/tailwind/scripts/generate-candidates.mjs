import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { __unstable__loadDesignSystem } from "tailwindcss";

const projectDirectory = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "..",
);
const candidatesPath = path.join(projectDirectory, "src", "candidates.txt");
const themePath = new URL(import.meta.resolve("tailwindcss/theme.css"));
const defaultTheme = await readFile(themePath, "utf8");
const designSystem = await __unstable__loadDesignSystem(
  `${defaultTheme}\n@tailwind utilities;`,
);

const candidates = new Set();
const modifierExamples = new Map();

for (const [candidate, { modifiers }] of designSystem.getClassList()) {
  candidates.add(candidate);

  const parsedCandidate = designSystem.parseCandidate(candidate)[0];
  const utilityRoot = parsedCandidate?.root ?? candidate;

  for (const modifier of modifiers) {
    const key = `${utilityRoot}\0${modifier}`;
    const modifiedCandidate = `${candidate}/${modifier}`;

    if (
      !modifierExamples.has(key) &&
      designSystem.candidatesToCss([modifiedCandidate])[0] !== null
    ) {
      modifierExamples.set(key, modifiedCandidate);
    }
  }
}

for (const candidate of modifierExamples.values()) {
  candidates.add(candidate);
}

for (const variant of designSystem.getVariants()) {
  const variantNames = variant.values.length
    ? variant.values.map(
        (value) => `${variant.name}${variant.hasDash ? "-" : ""}${value}`,
      )
    : [variant.name];

  for (const variantName of variantNames) {
    const candidate = `${variantName}:flex`;
    if (designSystem.candidatesToCss([candidate])[0] !== null) {
      candidates.add(candidate);
    }
  }
}

await mkdir(path.dirname(candidatesPath), { recursive: true });
await writeFile(
  candidatesPath,
  `${Array.from(candidates).sort().join("\n")}\n`,
);

console.log(`Generated ${candidates.size} Tailwind CSS candidates.`);
