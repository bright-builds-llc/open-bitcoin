import {
  PHASE134_APPLY_BOUNDARY_DIAGNOSTIC,
  PHASE134_APPLY_SOURCE_FILES,
} from "../check-phase134-apply-boundaries";

type ApplyFixtureFiles = Map<string, string>;

export type ApplyHelperMutation = {
  name: string;
  mutate: (files: ApplyFixtureFiles) => void;
};

export const APPLY_BOUNDARY_DIAGNOSTIC =
  PHASE134_APPLY_BOUNDARY_DIAGNOSTIC;

export const APPLY_HELPER_SOURCE_FILES = PHASE134_APPLY_SOURCE_FILES;

const COMPACT_FILE =
  "packages/open-bitcoin-node/src/network/compact_receive_candidates.rs";
const AUTHORITY_FILE =
  "packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs";

export function applyHelperMutations(): ApplyHelperMutation[] {
  return [
    {
      name: "one-level helper hides filesystem I/O",
      mutate: addCompactHelper(
        "hidden_projection_io",
        'let _ = std::fs::write("state", bytes);',
      ),
    },
    {
      name: "helper hides a forbidden dependent-target mutation",
      mutate: addCompactHelper(
        "hidden_target_mutation",
        "network.transactions_by_txid.clear();",
        "network: &mut ManagedPeerNetwork<S>",
      ),
    },
    {
      name: "multi-level helper hides filesystem I/O",
      mutate: (files) => {
        insertAtFunctionStart(
          files,
          COMPACT_FILE,
          "apply_prepared_compact",
          "\n        first_projection_helper();",
        );
        append(
          files,
          COMPACT_FILE,
          [
            "",
            "fn first_projection_helper() {",
            "    second_projection_helper();",
            "}",
            "",
            "fn second_projection_helper() {",
            '    let _ = std::fs::write("state", bytes);',
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "unknown repo-owned helper fails closed",
      mutate: addCompactHelper("unclassified_projection_helper", "let _ = 1;"),
    },
    {
      name: "removed validated transition API remains forbidden",
      mutate: (files) => {
        insertAtFunctionStart(
          files,
          AUTHORITY_FILE,
          "apply_prepared_lifecycle",
          "\n        self.mempool.mempool_mut().validate_prepared_mempool_transition(core);",
        );
      },
    },
    {
      name: "atomic commit cannot follow a dependent mutation",
      mutate: (files) => {
        insertAtFunctionStart(
          files,
          AUTHORITY_FILE,
          "commit_sealed_lifecycle",
          "\n        self.apply_prepared_evidence(evidence);",
        );
      },
    },
    {
      name: "atomic commit cannot appear twice",
      mutate: (files) => {
        insertAtFunctionEnd(
          files,
          AUTHORITY_FILE,
          "commit_sealed_lifecycle",
          [
            "",
            "        let _ = self",
            "            .mempool",
            "            .mempool_mut()",
            "            .commit_sealed_mempool_transition(core);",
          ].join("\n"),
        );
      },
    },
    {
      name: "recursive helper cycle terminates and preserves violations",
      mutate: (files) => {
        insertAtFunctionStart(
          files,
          COMPACT_FILE,
          "apply_prepared_compact",
          "\n        cyclic_projection_a();",
        );
        append(
          files,
          COMPACT_FILE,
          [
            "",
            "fn cyclic_projection_a() {",
            "    cyclic_projection_b();",
            "}",
            "",
            "fn cyclic_projection_b() {",
            "    cyclic_projection_a();",
            '    let _ = std::fs::write("state", bytes);',
            "}",
            "",
          ].join("\n"),
        );
      },
    },
  ];
}

function addCompactHelper(
  helperName: string,
  helperBody: string,
  maybeParameters = "",
): (files: ApplyFixtureFiles) => void {
  return (files) => {
    insertAtFunctionStart(
      files,
      COMPACT_FILE,
      "apply_prepared_compact",
      `\n        ${helperName}();`,
    );
    append(
      files,
      COMPACT_FILE,
      [
        "",
        `fn ${helperName}<S: ChainstateStore>(${maybeParameters}) {`,
        `    ${helperBody}`,
        "}",
        "",
      ].join("\n"),
    );
  };
}

function insertAtFunctionStart(
  files: ApplyFixtureFiles,
  relativePath: string,
  functionName: string,
  addition: string,
): void {
  const source = requireFile(files, relativePath);
  const body = functionBodyBounds(source, functionName);
  files.set(
    relativePath,
    source.slice(0, body.open + 1) + addition + source.slice(body.open + 1),
  );
}

function insertAtFunctionEnd(
  files: ApplyFixtureFiles,
  relativePath: string,
  functionName: string,
  addition: string,
): void {
  const source = requireFile(files, relativePath);
  const body = functionBodyBounds(source, functionName);
  files.set(
    relativePath,
    source.slice(0, body.close) + addition + source.slice(body.close),
  );
}

function functionBodyBounds(
  source: string,
  functionName: string,
): { open: number; close: number } {
  const marker = new RegExp(`\\bfn\\s+${functionName}\\b`).exec(source);
  if (!marker) {
    throw new Error(`missing fixture function: ${functionName}`);
  }
  const open = source.indexOf("{", marker.index);
  if (open < 0) {
    throw new Error(`missing fixture body: ${functionName}`);
  }
  let depth = 0;
  for (let index = open; index < source.length; index += 1) {
    if (source[index] === "{") {
      depth += 1;
    } else if (source[index] === "}") {
      depth -= 1;
      if (depth === 0) {
        return { open, close: index };
      }
    }
  }
  throw new Error(`unbalanced fixture body: ${functionName}`);
}

function append(
  files: ApplyFixtureFiles,
  relativePath: string,
  addition: string,
): void {
  files.set(relativePath, requireFile(files, relativePath) + addition);
}

function requireFile(files: ApplyFixtureFiles, relativePath: string): string {
  const maybeSource = files.get(relativePath);
  if (maybeSource === undefined) {
    throw new Error(`missing fixture file: ${relativePath}`);
  }
  return maybeSource;
}
