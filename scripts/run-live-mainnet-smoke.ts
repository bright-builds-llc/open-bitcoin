#!/usr/bin/env bun

import { main } from "./run-live-mainnet-smoke/cli";

main().catch((error: unknown) => {
  const message = error instanceof Error ? error.message : String(error);
  console.error(message);
  process.exit(1);
});
