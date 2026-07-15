export type ProcessEvidence = {
  targetTree: string;
  cargoJobs: string;
};

type ProcessRow = {
  pid: number;
  parentPid: number;
  text: string;
};

function parseProcessRows(output: string): { header: string; rows: ProcessRow[] } {
  const [header = "PID PPID STATE ELAPSED %CPU %MEM COMMAND", ...lines] = output.split("\n");
  const rows = lines.flatMap((line) => {
    const maybeMatch = line.match(/^\s*(\d+)\s+(\d+)\s+/);
    if (maybeMatch === null) {
      return [];
    }
    return [
      {
        pid: Number(maybeMatch[1]),
        parentPid: Number(maybeMatch[2]),
        text: line,
      },
    ];
  });
  return { header, rows };
}

export function filterProcessEvidence(output: string, targetPid: number): ProcessEvidence {
  const { header, rows } = parseProcessRows(output);
  const targetPids = new Set([targetPid]);
  let addedDescendant = true;
  while (addedDescendant) {
    addedDescendant = false;
    for (const row of rows) {
      if (targetPids.has(row.parentPid) && !targetPids.has(row.pid)) {
        targetPids.add(row.pid);
        addedDescendant = true;
      }
    }
  }
  const targetTreeRows = rows
    .filter((row) => targetPids.has(row.pid))
    .map((row) => row.text);
  const cargoRows = rows
    .filter((row) => /(cargo|rustc|rustdoc|target\/debug\/deps)/.test(row.text))
    .map((row) => row.text);
  return {
    targetTree: `${[header, ...targetTreeRows].join("\n")}\n`,
    cargoJobs: `${[header, ...cargoRows].join("\n")}\n`,
  };
}
