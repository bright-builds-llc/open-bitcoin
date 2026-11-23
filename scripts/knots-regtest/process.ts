import { spawn } from "node:child_process";
import { closeSync, openSync } from "node:fs";

export const COMMAND_TIMEOUT_MS = 20 * 60_000;
export const OBSERVER_TIMEOUT_MS = 3 * 60_000;
export const DAEMON_TIMEOUT_MS = 15 * 60_000;

/** A fresh process group lets timeout cleanup include compiler and Cargo descendants. */
export async function loggedCommand(
  cwd: string, command: string[], log: string, env = process.env, timeoutMs = COMMAND_TIMEOUT_MS,
) {
  const fd = openSync(log, "wx", 0o600);
  try {
    await new Promise<void>((resolve, reject) => {
      const child = spawn(command[0], command.slice(1), {
        cwd, env, detached: true, stdio: ["ignore", fd, fd],
      });
      let maybeStop: DOMException | null = null;
      const stop = (reason: DOMException) => {
        if (maybeStop) return;
        maybeStop = reason;
        if (child.pid === undefined) return;
        try {
          process.kill(-child.pid, "SIGKILL");
        } catch (error) {
          if ((error as NodeJS.ErrnoException).code !== "ESRCH") reject(error);
        }
      };
      const timer = setTimeout(() => {
        stop(new DOMException(`${command[0]} exceeded its ${timeoutMs}ms deadline. See ${log}`, "TimeoutError"));
      }, timeoutMs);
      const interrupt = () => stop(new DOMException(`${command[0]} interrupted. See ${log}`, "AbortError"));
      process.once("SIGINT", interrupt);
      process.once("SIGTERM", interrupt);
      const cleanup = () => {
        clearTimeout(timer);
        process.off("SIGINT", interrupt);
        process.off("SIGTERM", interrupt);
      };
      child.once("error", (error) => {
        cleanup();
        reject(error);
      });
      child.once("exit", (code) => {
        cleanup();
        if (maybeStop) {
          reject(maybeStop);
        } else if (code === 0) {
          resolve();
        } else {
          reject(new Error(`${command[0]} ${command[1] ?? ""} failed. See ${log}`));
        }
      });
    });
  } finally {
    closeSync(fd);
  }
}
