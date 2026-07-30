import {
  PHASE134_APPLY_BOUNDARY_DIAGNOSTIC,
  PHASE134_APPLY_SOURCE_FILES,
} from "../check-phase134-apply-boundaries";
import {
  aggregateReachabilityMutations,
  aggregateReachabilityPositiveMutations,
} from "./apply-helpers/aggregate-reachability";
import {
  strictReachabilityMutations,
  strictReachabilityPositiveMutations,
} from "./apply-helpers/strict-reachability";

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
const MEMPOOL_LIFECYCLE_FILE =
  "packages/open-bitcoin-node/src/network/mempool_lifecycle.rs";

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
      name: "unresolved helper fails closed",
      mutate: (files) => {
        insertAtFunctionStart(
          files,
          COMPACT_FILE,
          "apply_prepared_compact",
          "\n        unresolved_projection_helper();",
        );
      },
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
      name: "removed public sealed transition API remains forbidden",
      mutate: (files) => {
        insertAtFunctionStart(
          files,
          AUTHORITY_FILE,
          "apply_prepared_lifecycle",
          "\n        self.mempool.mempool_mut().commit_sealed_mempool_transition(core);",
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
            "            .commit_prepared_mempool_transition_with(core, || ());",
          ].join("\n"),
        );
      },
    },
    {
      name: "connected-block transaction requires the atomic core commit",
      mutate: (files) => {
        replaceInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          ".commit_prepared_mempool_transition_with(core, || {",
          ".removed_mempool_transaction(core, || {",
        );
      },
    },
    {
      name: "connected-block transaction requires prepared chainstate commit",
      mutate: (files) => {
        replaceInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "chainstate.commit_prepared_connect(prepared_chainstate);",
          "// removed prepared chainstate commit",
        );
      },
    },
    {
      name: "connected-block transaction requires active-tip reset",
      mutate: (files) => {
        replaceInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "peer_manager.on_active_tip_changed(",
          "peer_manager.removed_active_tip_changed(",
        );
      },
    },
    {
      name: "connected-block transaction requires block recording",
      mutate: (files) => {
        replaceInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "blocks_by_hash.insert(position.block_hash, block.clone());",
          "// removed block recording",
        );
      },
    },
    {
      name: "connected-block transaction requires local-position recording",
      mutate: (files) => {
        replaceInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "peer_manager.note_local_position(position);",
          "// removed local-position recording",
        );
      },
    },
    {
      name: "connected-block dependent apply must follow the transaction",
      mutate: (files) => {
        replaceInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        let ((), delta) = self",
          [
            "        self.apply_prepared_lifecycle(dependent);",
            "        let ((), delta) = self",
          ].join("\n"),
        );
        replaceInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        self.apply_prepared_lifecycle(dependent);\n        Ok(delta)",
          "        Ok(delta)",
        );
      },
    },
    {
      name: "connected-block callback rejects fallible work",
      mutate: (files) => {
        replaceInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "chainstate.commit_prepared_connect(prepared_chainstate);",
          [
            "chainstate.commit_prepared_connect(prepared_chainstate);",
            "                derive_after_chainstate()?;",
          ].join("\n"),
        );
      },
    },
    {
      name: "connected-block post-transaction work remains infallible",
      mutate: (files) => {
        replaceInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        self.apply_prepared_lifecycle(dependent);",
          [
            "        derive_after_transaction()?;",
            "        self.apply_prepared_lifecycle(dependent);",
          ].join("\n"),
        );
      },
    },
    ...aggregateReachabilityMutations(),
    ...strictReachabilityMutations(),
    {
      name: "local block seam cannot bypass the transaction root",
      mutate: (files) => {
        replaceInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          "self.commit_connected_block_lifecycle_transaction(",
          "self.commit_sealed_lifecycle(",
        );
      },
    },
    {
      name: "block seam requires prepared chainstate",
      mutate: (files) => {
        replaceInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          ".prepare_connect_block(",
          ".removed_prepare_connect_block(",
        );
      },
    },
    {
      name: "block seam requires prepared mempool lifecycle",
      mutate: (files) => {
        replaceInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          ".prepare_connected_block_transition(",
          ".removed_connected_block_transition(",
        );
      },
    },
    {
      name: "block seam requires sealed dependent lifecycle",
      mutate: (files) => {
        replaceInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          ".prepare_maintenance_step(",
          ".removed_maintenance_step(",
        );
      },
    },
    {
      name: "block seam enforces preparation before sealing",
      mutate: (files) => {
        replaceInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          "prepare_connected_block_transition",
          "temporary_preparation_step",
        );
        replaceInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          "prepare_maintenance_step",
          "prepare_connected_block_transition",
        );
        replaceInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          "temporary_preparation_step",
          "prepare_maintenance_step",
        );
      },
    },
    {
      name: "stored block seam cannot bypass the transaction root",
      mutate: (files) => {
        replaceInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_stored_block",
          "self.commit_connected_block_lifecycle_transaction(",
          "self.commit_sealed_lifecycle(",
        );
      },
    },
    {
      name: "block seam cannot mutate between preparation and sealing",
      mutate: (files) => {
        replaceInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          "        let sealed_lifecycle = self.prepare_maintenance_step(prepared_lifecycle)?;",
          [
            "        self.chainstate.commit_prepared_connect(prepared_chainstate);",
            "        let sealed_lifecycle = self.prepare_maintenance_step(prepared_lifecycle)?;",
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

export function applyHelperPositiveMutations(): ApplyHelperMutation[] {
  return [
    ...aggregateReachabilityPositiveMutations(),
    ...strictReachabilityPositiveMutations(),
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

function replaceInFunction(
  files: ApplyFixtureFiles,
  relativePath: string,
  functionName: string,
  search: string,
  replacement: string,
): void {
  const source = requireFile(files, relativePath);
  const body = functionBodyBounds(source, functionName);
  const functionBody = source.slice(body.open + 1, body.close);
  const match = functionBody.indexOf(search);
  if (match < 0) {
    throw new Error(`missing fixture text in ${functionName}: ${search}`);
  }
  const absoluteMatch = body.open + 1 + match;
  files.set(
    relativePath,
    source.slice(0, absoluteMatch) +
      replacement +
      source.slice(absoluteMatch + search.length),
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
