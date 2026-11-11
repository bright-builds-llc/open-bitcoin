// Bun test preload: routes phase-checker test files through the sampling
// stand-in for "bun:test". The files themselves stay byte-identical, because
// other checkers pin their text.

import { plugin } from "bun";
import path from "node:path";

const SAMPLED_BUN_TEST = path.join(import.meta.dir, "sampled-bun-test.ts");
const BUN_TEST_IMPORT = /from "bun:test"/g;

plugin({
  name: "sample-checker-tests",
  setup(build) {
    build.onLoad({ filter: /\/scripts\/check-phase[^/]*(?:\/[^/]+)?\.test\.ts$/ }, async (args) => {
      const source = await Bun.file(args.path).text();
      return { contents: source.replace(BUN_TEST_IMPORT, `from ${JSON.stringify(SAMPLED_BUN_TEST)}`), loader: "ts" };
    });
  },
});
