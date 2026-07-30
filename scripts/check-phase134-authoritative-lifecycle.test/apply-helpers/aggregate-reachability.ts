import type { ApplyHelperMutation } from "../apply-helpers";

type ApplyFixtureFiles = Map<string, string>;

const AUTHORITY_FILE =
  "packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs";
const COMPACT_FILE =
  "packages/open-bitcoin-node/src/network/compact_receive_candidates.rs";
const INVENTORY_FILE =
  "packages/open-bitcoin-node/src/network/inventory.rs";
const MEMPOOL_LIFECYCLE_FILE =
  "packages/open-bitcoin-node/src/network/mempool_lifecycle.rs";
const UNSCANNED_FILE =
  "packages/open-bitcoin-node/src/network/unscanned_effects.rs";

export function aggregateReachabilityMutations(): ApplyHelperMutation[] {
  return [
    {
      name: "ordinary aggregate helper hides I/O before the transaction",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          AUTHORITY_FILE,
          "commit_sealed_lifecycle",
          "        let ((), committed_delta) = self",
          "        hidden_ordinary_root_effect();\n",
        );
        appendFreeHelper(
          files,
          AUTHORITY_FILE,
          "hidden_ordinary_root_effect",
          'let _ = std::fs::write("state", b"ordinary");',
        );
      },
    },
    {
      name: "connected-block helper hides I/O before the transaction",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        let ((), delta) = self",
          "        hidden_before_transaction();\n",
        );
        appendFreeHelper(
          files,
          AUTHORITY_FILE,
          "hidden_before_transaction",
          'let _ = std::fs::write("state", b"before");',
        );
      },
    },
    {
      name: "connected-block helper hides I/O after the transaction",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        self.apply_prepared_lifecycle(dependent);",
          "        hidden_after_transaction();\n",
        );
        appendFreeHelper(
          files,
          AUTHORITY_FILE,
          "hidden_after_transaction",
          'let _ = std::fs::write("state", b"after");',
        );
      },
    },
    {
      name: "block seam helper hides I/O between sealing and transaction",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          "        self.commit_connected_block_lifecycle_transaction(",
          "        hidden_between_sealing_and_transaction();\n",
        );
        appendFreeHelper(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "hidden_between_sealing_and_transaction",
          'let _ = std::fs::write("state", b"between");',
        );
      },
    },
    {
      name: "connected-block helper hides authoritative block-index mutation",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        let ((), delta) = self",
          "        hidden_block_index_mutation(blocks_by_hash);\n",
        );
        append(
          files,
          AUTHORITY_FILE,
          [
            "",
            "fn hidden_block_index_mutation<K, V>(",
            "    blocks_by_hash: &mut std::collections::BTreeMap<K, V>,",
            ") {",
            "    blocks_by_hash.clear();",
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "connected-block nested helper hides I/O",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        let ((), delta) = self",
          "        outer_connected_block_helper();\n",
        );
        append(
          files,
          AUTHORITY_FILE,
          [
            "",
            "fn outer_connected_block_helper() {",
            "    inner_connected_block_helper();",
            "}",
            "",
            "fn inner_connected_block_helper() {",
            '    let _ = std::fs::write("state", b"nested");',
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "aliased connected-block helper hides I/O",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        self.apply_prepared_lifecycle(dependent);",
          "        aliased_connected_block_helper();\n",
        );
        append(
          files,
          AUTHORITY_FILE,
          [
            "",
            "use self::hidden_aliased_connected_block_helper as aliased_connected_block_helper;",
            "",
            "fn hidden_aliased_connected_block_helper() {",
            '    let _ = std::fs::write("state", b"aliased");',
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "module-qualified connected-block helper hides I/O",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        let ((), delta) = self",
          "        super::super::compact_receive_candidates::hidden_module_effect();\n",
        );
        append(
          files,
          COMPACT_FILE,
          [
            "",
            "pub(super) fn hidden_module_effect() {",
            '    let _ = std::fs::write("state", b"module");',
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "direct collection mutation appears between sealing and transaction",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          "        self.commit_connected_block_lifecycle_transaction(",
          "        self.unbroadcast_members.clear();\n",
        );
      },
    },
    {
      name: "traversed helper retains collection entries outside the transaction",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          "        self.commit_connected_block_lifecycle_transaction(",
          "        retain_seam_entries(&mut self.unbroadcast_members);\n",
        );
        append(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          [
            "",
            "fn retain_seam_entries<T>(",
            "    children: &mut std::collections::BTreeSet<T>,",
            ") {",
            "    children.retain(|_| true);",
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "nested helper extends a collection outside the transaction",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        let ((), delta) = self",
          "        outer_collection_mutator(&mut self.unbroadcast_members);\n",
        );
        append(
          files,
          AUTHORITY_FILE,
          [
            "",
            "fn outer_collection_mutator<T>(",
            "    values: &mut std::collections::BTreeSet<T>,",
            ") {",
            "    inner_collection_mutator(values);",
            "}",
            "",
            "fn inner_collection_mutator<T>(",
            "    values: &mut std::collections::BTreeSet<T>,",
            ") {",
            "    values.extend([]);",
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "unscanned qualified helper cannot fall back to benign same-name helper",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        let ((), delta) = self",
          "        super::super::unscanned_effects::collision_helper();\n",
        );
        appendFreeHelper(files, COMPACT_FILE, "collision_helper", "let _ = 1_u8;");
        files.set(
          UNSCANNED_FILE,
          [
            "pub(super) fn collision_helper() {",
            '    let _ = std::fs::write("state", b"unscanned");',
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "unresolved qualified helper fails closed",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        let ((), delta) = self",
          "        super::super::missing_module::missing_helper();\n",
        );
      },
    },
  ];
}

export function aggregateReachabilityPositiveMutations(): ApplyHelperMutation[] {
  return [
    {
      name: "accepts nested pure helper before connected-block transaction",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        let ((), delta) = self",
          "        outer_pure_connected_block_helper();\n",
        );
        append(
          files,
          AUTHORITY_FILE,
          [
            "",
            "fn outer_pure_connected_block_helper() {",
            "    inner_pure_connected_block_helper();",
            "}",
            "",
            "fn inner_pure_connected_block_helper() {",
            "    let _value = 1_u8;",
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "accepts aliased pure helper between sealing and transaction",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          "        self.commit_connected_block_lifecycle_transaction(",
          "        aliased_pure_seam_helper();\n",
        );
        append(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          [
            "",
            "use self::pure_seam_helper as aliased_pure_seam_helper;",
            "",
            "fn pure_seam_helper() {",
            "    let _value = 1_u8;",
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "accepts module-qualified pure helper before connected-block transaction",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        let ((), delta) = self",
          "        super::super::compact_receive_candidates::pure_module_helper();\n",
        );
        append(
          files,
          COMPACT_FILE,
          [
            "",
            "pub(super) fn pure_module_helper() {",
            "    let _value = 1_u8;",
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "accepts direct read-only method between sealing and transaction",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          "        self.commit_connected_block_lifecycle_transaction(",
          "        let _ = position.height.saturating_add(0);\n",
        );
      },
    },
    {
      name: "accepts traversed helper with a read-only collection method",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "connect_local_block",
          "        self.commit_connected_block_lifecycle_transaction(",
          "        inspect_seam_entries(&self.unbroadcast_members);\n",
        );
        append(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          [
            "",
            "fn inspect_seam_entries<T>(",
            "    children: &std::collections::BTreeSet<T>,",
            ") {",
            "    let _ = children.is_empty();",
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "accepts exact qualified helper despite benign same-name modules",
      mutate: (files) => {
        insertBeforeInFunction(
          files,
          AUTHORITY_FILE,
          "commit_connected_block_lifecycle_transaction",
          "        let ((), delta) = self",
          "        super::super::compact_receive_candidates::same_named_pure_helper();\n",
        );
        appendFreeHelper(
          files,
          COMPACT_FILE,
          "same_named_pure_helper",
          "let _ = 1_u8;",
        );
        appendFreeHelper(
          files,
          INVENTORY_FILE,
          "same_named_pure_helper",
          "let _ = 2_u8;",
        );
      },
    },
  ];
}

function insertBeforeInFunction(
  files: ApplyFixtureFiles,
  relativePath: string,
  functionName: string,
  marker: string,
  addition: string,
): void {
  const source = requireFile(files, relativePath);
  const body = functionBodyBounds(source, functionName);
  const functionBody = source.slice(body.open + 1, body.close);
  const match = functionBody.indexOf(marker);
  if (match < 0) {
    throw new Error(`missing fixture text in ${functionName}: ${marker}`);
  }
  const absoluteMatch = body.open + 1 + match;
  files.set(
    relativePath,
    source.slice(0, absoluteMatch) + addition + source.slice(absoluteMatch),
  );
}

function appendFreeHelper(
  files: ApplyFixtureFiles,
  relativePath: string,
  helperName: string,
  body: string,
): void {
  append(
    files,
    relativePath,
    ["", `fn ${helperName}() {`, `    ${body}`, "}", ""].join("\n"),
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

function requireFile(
  files: ApplyFixtureFiles,
  relativePath: string,
): string {
  const maybeSource = files.get(relativePath);
  if (maybeSource === undefined) {
    throw new Error(`missing fixture file: ${relativePath}`);
  }
  return maybeSource;
}
