import type { ApplyHelperMutation } from "../apply-helpers";
import { append, insertBeforeInFunction } from "./aggregate-reachability";

const AUTHORITY_FILE =
  "packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs";
const MEMPOOL_LIFECYCLE_FILE =
  "packages/open-bitcoin-node/src/network/mempool_lifecycle.rs";

export function strictReachabilityMutations(): ApplyHelperMutation[] {
  return [
    ...effectfulRepoMethodMutations(),
    {
      name: "macro hides authority mutation between sealing and transaction",
      mutate: (files) => {
        insertAtSeam(files, "        hidden_authority_mutation!(self);\n");
        append(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          [
            "",
            "macro_rules! hidden_authority_mutation {",
            "    ($network:expr) => {",
            "        $network.unbroadcast_members.clear();",
            "    };",
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "qualified unknown macro runs between sealing and transaction",
      mutate: (files) => {
        insertAtSeam(files, "        super::hidden_unknown_macro!(self);\n");
      },
    },
    {
      name: "indexed assignment runs between sealing and transaction",
      mutate: (files) => {
        insertAtSeam(
          files,
          [
            "        let mut hidden = [0_u8; 1];",
            "        hidden[0] = 1;",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "reached helper hides indexed assignment",
      mutate: (files) => {
        insertAtSeam(files, "        hidden_indexed_assignment();\n");
        appendFreeHelper(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          "hidden_indexed_assignment",
          ["let mut hidden = [0_u8; 1];", "hidden[0] = 1;"],
        );
      },
    },
    {
      name: "compound assignment runs between sealing and transaction",
      mutate: (files) => {
        insertAtSeam(
          files,
          ["        let mut hidden = 0_u8;", "        hidden += 1;", ""].join(
            "\n",
          ),
        );
      },
    },
    {
      name: "dereference assignment runs between sealing and transaction",
      mutate: (files) => {
        insertAtSeam(
          files,
          [
            "        let mut hidden = 0_u8;",
            "        let hidden_ref = &mut hidden;",
            "        *hidden_ref = 1;",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "raw mutable borrow runs between sealing and transaction",
      mutate: (files) => {
        insertAtSeam(
          files,
          "        let _hidden = &raw mut self.unbroadcast_members;\n",
        );
      },
    },
    {
      name: "closure runs between sealing and transaction",
      mutate: (files) => {
        insertAtSeam(files, "        let _hidden = || 1_u8;\n");
      },
    },
    {
      name: "async block runs between sealing and transaction",
      mutate: (files) => {
        insertAtSeam(files, "        let _hidden = async { 1_u8 };\n");
      },
    },
    {
      name: "unsafe block runs between sealing and transaction",
      mutate: (files) => {
        insertAtSeam(files, "        let _hidden = unsafe { 1_u8 };\n");
      },
    },
  ];
}

export function strictReachabilityPositiveMutations(): ApplyHelperMutation[] {
  return [
    {
      name: "accepts traversed pure repo getter before the transaction",
      mutate: (files) => {
        insertBeforeAuthorityTransaction(files, "        inspect_pure_getter();\n");
        append(
          files,
          AUTHORITY_FILE,
          [
            "",
            "struct PureGetter;",
            "",
            "impl PureGetter {",
            "    fn get(&self) -> usize {",
            "        1",
            "    }",
            "}",
            "",
            "fn inspect_pure_getter() {",
            "    let getter = PureGetter;",
            "    let _ = getter.get();",
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "accepts exact standard receiver methods before the transaction",
      mutate: (files) => {
        insertBeforeAuthorityTransaction(
          files,
          "        inspect_standard_receivers();\n",
        );
        appendFreeHelper(
          files,
          AUTHORITY_FILE,
          "inspect_standard_receivers",
          [
            "let maybe_entry = Some(1_u8);",
            "let _ = maybe_entry.iter();",
            "let count = 1_usize;",
            "let _ = count.saturating_sub(1);",
          ],
        );
      },
    },
  ];
}

function effectfulRepoMethodMutations(): ApplyHelperMutation[] {
  return ["get", "iter", "len"].map((methodName) => ({
    name: `effectful repo ${methodName} method runs before the transaction`,
    mutate: (files) => {
      const typeName = `Effectful${capitalize(methodName)}`;
      const helperName = `invoke_effectful_${methodName}`;
      insertBeforeAuthorityTransaction(files, `        ${helperName}();\n`);
      append(
        files,
        AUTHORITY_FILE,
        [
          "",
          `struct ${typeName} {`,
          "    value: u8,",
          "}",
          "",
          `impl ${typeName} {`,
          `    fn ${methodName}(&mut self) {`,
          "        self.value = 1;",
          "    }",
          "}",
          "",
          `fn ${helperName}() {`,
          `    let mut target = ${typeName} { value: 0 };`,
          `    target.${methodName}();`,
          "}",
          "",
        ].join("\n"),
      );
    },
  }));
}

function insertAtSeam(
  files: Map<string, string>,
  addition: string,
): void {
  insertBeforeInFunction(
    files,
    MEMPOOL_LIFECYCLE_FILE,
    "connect_local_block",
    "        self.commit_connected_block_lifecycle_transaction(",
    addition,
  );
}

function insertBeforeAuthorityTransaction(
  files: Map<string, string>,
  addition: string,
): void {
  insertBeforeInFunction(
    files,
    AUTHORITY_FILE,
    "commit_connected_block_lifecycle_transaction",
    "        let ((), delta) = self",
    addition,
  );
}

function appendFreeHelper(
  files: Map<string, string>,
  relativePath: string,
  helperName: string,
  body: string[],
): void {
  append(
    files,
    relativePath,
    ["", `fn ${helperName}() {`, ...body.map((line) => `    ${line}`), "}", ""].join(
      "\n",
    ),
  );
}

function capitalize(value: string): string {
  return `${value[0]?.toUpperCase() ?? ""}${value.slice(1)}`;
}
