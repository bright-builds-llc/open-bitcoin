import type { ApplyHelperMutation } from "../apply-helpers";
import { append, insertBeforeInFunction } from "./aggregate-reachability";

type ApplyFixtureFiles = Map<string, string>;

const AUTHORITY_FILE =
  "packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs";
const COMPACT_FILE =
  "packages/open-bitcoin-node/src/network/compact_receive_candidates.rs";
const MEMPOOL_LIFECYCLE_FILE =
  "packages/open-bitcoin-node/src/network/mempool_lifecycle.rs";

export function tokenScannerMutations(): ApplyHelperMutation[] {
  return [
    {
      name: "parenthesized receiver mutation runs before the transaction",
      mutate: (files) => {
        insertBeforeAuthorityTransaction(
          files,
          "        (blocks_by_hash).clear();\n",
        );
      },
    },
    {
      name: "effectful turbofish helper runs before the transaction",
      mutate: (files) => {
        insertBeforeAuthorityTransaction(
          files,
          "        hidden_generic_effect::<u8>();\n",
        );
        appendFreeHelper(
          files,
          AUTHORITY_FILE,
          "hidden_generic_effect<T>",
          'let _ = std::fs::write("state", b"generic");',
        );
      },
    },
    {
      name: "unresolved nested turbofish helper fails closed",
      mutate: (files) => {
        insertBeforeAuthorityTransaction(
          files,
          "        missing_generic_helper::<Vec<Option<u8>>>();\n",
        );
      },
    },
    {
      name: "unresolved qualified turbofish helper fails closed",
      mutate: (files) => {
        insertBeforeAuthorityTransaction(
          files,
          "        super::missing::generic_helper::<Vec<Option<u8>>>();\n",
        );
      },
    },
    {
      name: "parenthesized effectful function call runs before the transaction",
      mutate: (files) => {
        insertBeforeAuthorityTransaction(
          files,
          "        (hidden_parenthesized_effect)();\n",
        );
        appendFreeHelper(
          files,
          AUTHORITY_FILE,
          "hidden_parenthesized_effect",
          'let _ = std::fs::write("state", b"parenthesized");',
        );
      },
    },
    {
      name: "tuple-projected collection mutation runs before the transaction",
      mutate: (files) => {
        insertBeforeAuthorityTransaction(
          files,
          [
            "        let mut hidden = (&mut *blocks_by_hash,);",
            "        hidden.0.clear();",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "const-expression signature cannot hide a reached effect",
      mutate: (files) => {
        insertBeforeAuthorityTransaction(
          files,
          "        hidden_const_signature();\n",
        );
        append(
          files,
          AUTHORITY_FILE,
          [
            "",
            "fn hidden_const_signature() -> [u8; { 1 }] {",
            '    let _ = std::fs::write("state", b"const-signature");',
            "    [1]",
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "nested BTreeSet type cannot spoof outer receiver purity",
      mutate: (files) => {
        insertBeforeAuthorityTransaction(
          files,
          "        invoke_nested_receiver_spoof();\n",
        );
        append(
          files,
          AUTHORITY_FILE,
          [
            "",
            "struct EffectfulSet<T>(std::marker::PhantomData<T>);",
            "",
            "impl<T> EffectfulSet<T> {",
            "    fn is_empty(&self) {",
            '        let _ = std::fs::write("state", b"nested-type");',
            "    }",
            "}",
            "",
            "fn invoke_nested_receiver_spoof() {",
            "    let children: EffectfulSet<std::collections::BTreeSet<u8>> =",
            "        EffectfulSet(std::marker::PhantomData);",
            "    inspect_nested_receiver(&children);",
            "}",
            "",
            "fn inspect_nested_receiver(",
            "    children: &EffectfulSet<std::collections::BTreeSet<u8>>,",
            ") {",
            "    children.is_empty();",
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    ...receiverSpoofMutations(),
  ];
}

export function tokenScannerPositiveMutations(): ApplyHelperMutation[] {
  return [
    {
      name: "accepts bit-or, const, static, and typed-array bindings",
      mutate: (files) => {
        insertAtSeam(
          files,
          [
            "        const LOCAL_MASK: u8 = 1_u8 | 2_u8 | 4_u8;",
            "        static STATIC_MASK: u8 = 1_u8;",
            "        let typed: [u8; 1] = [LOCAL_MASK | STATIC_MASK; 1];",
            "        let _ = typed;",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "accepts parenthesized type-proven pure receiver",
      mutate: (files) => {
        insertAtSeam(
          files,
          "        inspect_parenthesized_pure(&self.unbroadcast_members);\n",
        );
        append(
          files,
          MEMPOOL_LIFECYCLE_FILE,
          [
            "",
            "fn inspect_parenthesized_pure<T>(",
            "    children: &std::collections::BTreeSet<T>,",
            ") {",
            "    let _ = (children).is_empty();",
            "}",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "accepts nested generic turbofish helper",
      mutate: (files) => {
        insertBeforeAuthorityTransaction(
          files,
          "        pure_nested_generic::<Vec<Option<u8>>>();\n",
        );
        appendFreeHelper(
          files,
          AUTHORITY_FILE,
          "pure_nested_generic<T>",
          "let _ = 1_u8;",
        );
      },
    },
    {
      name: "accepts exact qualified nested generic turbofish helper",
      mutate: (files) => {
        insertBeforeAuthorityTransaction(
          files,
          "        super::super::compact_receive_candidates::pure_qualified_generic::<Vec<Option<u8>>>();\n",
        );
        appendFreeHelper(
          files,
          COMPACT_FILE,
          "pure_qualified_generic<T>",
          "let _ = 1_u8;",
          "pub(super) ",
        );
      },
    },
    {
      name: "ignores checker tokens inside comments and literals",
      mutate: (files) => {
        insertAtSeam(
          files,
          [
            "        // missing_generic_helper::<u8>();",
            "        /* (blocks_by_hash).clear(); hidden_macro!(); */",
            '        let _normal = "missing::<u8>(); || value = 1;";',
            '        let _raw = r#"(children).retain(|_| false); unsafe {}"#;',
            "        let _character = '|';",
            "        let _byte = b'!';",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "accepts local type aliases",
      mutate: (files) => {
        insertAtSeam(
          files,
          [
            "        type LocalByte = u8;",
            "        let _value: LocalByte = 1;",
            "        type LocalIterator = dyn Iterator<Item = u8>;",
            "        let _maybe_values: Option<&LocalIterator> = None;",
            "",
          ].join("\n"),
        );
      },
    },
    {
      name: "accepts parenthesized pure call with const-expression signature",
      mutate: (files) => {
        insertBeforeAuthorityTransaction(
          files,
          "        let _ = (pure_const_signature)();\n",
        );
        append(
          files,
          AUTHORITY_FILE,
          [
            "",
            "fn pure_const_signature() -> [u8; { 1 }] {",
            "    [1]",
            "}",
            "",
          ].join("\n"),
        );
      },
    },
  ];
}

function receiverSpoofMutations(): ApplyHelperMutation[] {
  return [
    ["maybe_entry", "iter"],
    ["children", "is_empty"],
    ["count", "saturating_sub"],
  ].map(([receiver, method]) => ({
    name: `same-name ${receiver}.${method} receiver cannot spoof purity`,
    mutate: (files) => {
      const typeName = `${capitalize(receiver)}Spoof`;
      const helperName = `invoke_${receiver}_spoof`;
      insertBeforeAuthorityTransaction(files, `        ${helperName}();\n`);
      append(
        files,
        AUTHORITY_FILE,
        [
          "",
          `struct ${typeName};`,
          "",
          `impl ${typeName} {`,
          `    fn ${method}(&self) {`,
          '        let _ = std::fs::write("state", b"spoof");',
          "    }",
          "}",
          "",
          `fn ${helperName}() {`,
          `    let ${receiver} = ${typeName};`,
          `    ${receiver}.${method}();`,
          "}",
          "",
        ].join("\n"),
      );
    },
  }));
}

function insertAtSeam(files: ApplyFixtureFiles, addition: string): void {
  insertBeforeInFunction(
    files,
    MEMPOOL_LIFECYCLE_FILE,
    "connect_local_block",
    "        self.commit_connected_block_lifecycle_transaction(",
    addition,
  );
}

function insertBeforeAuthorityTransaction(
  files: ApplyFixtureFiles,
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
  files: ApplyFixtureFiles,
  relativePath: string,
  helperName: string,
  body: string,
  visibility = "",
): void {
  append(
    files,
    relativePath,
    [
      "",
      `${visibility}fn ${helperName}() {`,
      `    ${body}`,
      "}",
      "",
    ].join("\n"),
  );
}

function capitalize(value: string): string {
  return `${value[0]?.toUpperCase() ?? ""}${value.slice(1)}`;
}
