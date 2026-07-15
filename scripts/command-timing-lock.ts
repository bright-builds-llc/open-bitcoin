import { createHash, randomUUID } from "node:crypto";
import { mkdir, readFile, rename, rm, stat, writeFile } from "node:fs/promises";
import path from "node:path";

const DEFAULT_HEARTBEAT_MS = 60_000;
const OWNER_WRITE_GRACE_MS = 5_000;

export type TargetLock = {
  lockPath: string;
  runId: string;
  release: () => Promise<void>;
};

type LockOwner = {
  pid: number;
  runId: string;
  key: string;
  startedAt: string;
};

export function normalizeCommandKey(value: string): string {
  const normalized = value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9._-]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .slice(0, 96);
  if (normalized.length === 0) {
    throw new Error("timing key must contain at least one letter or number");
  }
  return normalized;
}

function processIsAlive(pid: number): boolean {
  try {
    process.kill(pid, 0);
    return true;
  } catch {
    return false;
  }
}

export function timingProcessIsAlive(pid: number): boolean {
  return processIsAlive(pid);
}

async function readLockOwner(lockPath: string): Promise<LockOwner | null> {
  try {
    const value = JSON.parse(await readFile(path.join(lockPath, "owner.json"), "utf8"));
    if (
      typeof value.pid !== "number" ||
      typeof value.runId !== "string" ||
      typeof value.key !== "string" ||
      typeof value.startedAt !== "string"
    ) {
      return null;
    }
    return value;
  } catch {
    return null;
  }
}

async function abandonDeadLock(lockPath: string, locksRoot: string): Promise<void> {
  const abandonedRoot = path.join(locksRoot, "abandoned");
  await mkdir(abandonedRoot, { recursive: true });
  const abandonedPath = path.join(
    abandonedRoot,
    `${path.basename(lockPath)}-${Date.now()}-${randomUUID()}`,
  );
  try {
    await rename(lockPath, abandonedPath);
    await writeFile(
      path.join(abandonedPath, "abandoned.json"),
      `${JSON.stringify({ abandonedAt: new Date().toISOString() })}\n`,
    );
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code !== "ENOENT") {
      throw error;
    }
  }
}

async function ownerWriteMayBeInProgress(lockPath: string): Promise<boolean> {
  try {
    const metadata = await stat(lockPath);
    return Date.now() - metadata.mtimeMs < OWNER_WRITE_GRACE_MS;
  } catch {
    return false;
  }
}

export async function acquireTargetLock(options: {
  key: string;
  targetDirectory: string;
  stateRoot: string;
  heartbeatMs?: number;
  stderr?: Pick<typeof process.stderr, "write">;
}): Promise<TargetLock> {
  const runId = randomUUID();
  const heartbeatMs = options.heartbeatMs ?? DEFAULT_HEARTBEAT_MS;
  const stderr = options.stderr ?? process.stderr;
  const lockHash = createHash("sha256")
    .update(path.resolve(options.targetDirectory))
    .digest("hex")
    .slice(0, 20);
  const locksRoot = path.join(options.stateRoot, "locks");
  const lockPath = path.join(locksRoot, `cargo-target-${lockHash}`);
  await mkdir(locksRoot, { recursive: true });

  const waitStartedAt = Date.now();
  let lastWaitNoticeAt = 0;
  while (true) {
    try {
      await mkdir(lockPath);
      await writeFile(
        path.join(lockPath, "owner.json"),
        `${JSON.stringify({
          schemaVersion: 1,
          pid: process.pid,
          runId,
          key: normalizeCommandKey(options.key),
          startedAt: new Date().toISOString(),
        })}\n`,
      );
      break;
    } catch (error) {
      const code = (error as NodeJS.ErrnoException).code;
      if (code !== "EEXIST") {
        throw error;
      }

      const maybeOwner = await readLockOwner(lockPath);
      if (maybeOwner === null && (await ownerWriteMayBeInProgress(lockPath))) {
        await Bun.sleep(50);
        continue;
      }
      if (maybeOwner === null || !processIsAlive(maybeOwner.pid)) {
        await abandonDeadLock(lockPath, locksRoot);
        continue;
      }

      const now = Date.now();
      if (lastWaitNoticeAt === 0 || now - lastWaitNoticeAt >= heartbeatMs) {
        const elapsedSeconds = Math.floor((now - waitStartedAt) / 1_000);
        stderr.write(
          `[timing] waiting for ${maybeOwner.key} (pid ${maybeOwner.pid}, ${elapsedSeconds}s elapsed)\n`,
        );
        lastWaitNoticeAt = now;
      }
      await Bun.sleep(Math.min(heartbeatMs, 250));
    }
  }

  return {
    lockPath,
    runId,
    async release() {
      const maybeOwner = await readLockOwner(lockPath);
      if (maybeOwner?.runId === runId) {
        await rm(lockPath, { force: true, recursive: true });
      }
    },
  };
}
