import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { checkPanicSites, sourceFiles } from "./check-panic-sites.ts";

const ROOT = "packages/open-bitcoin-node/src/";
const BOOT = ROOT + "storage/canonical_bootstrap";
const SYNC = ROOT + "sync/canonical_genesis";
const LIVE = ROOT + "lib.rs";
const crash = 'fn fixture() { None::<u8>.expect("test fixture"); }';

function fixture(): Map<string,string> {
  return new Map([
    [LIVE, "pub fn ordinary() {}"],
    [BOOT+".rs", "#[cfg(test)]\nmod tests;"],
    [BOOT+"/tests.rs", '#[path = "qualification_tests.rs"]\nmod qualification_tests;\n#[path = "test_support.rs"]\nmod test_support;'],
    [BOOT+"/qualification_tests.rs", crash],
    [BOOT+"/test_support.rs", crash],
    [SYNC+".rs", "#[cfg(test)]\nmod basic_tests;"],
    [SYNC+"/basic_tests.rs", crash],
  ]);
}

test("three split files earn their actual transitive test ownership", () => {
  // Arrange
  const sources = fixture();
  // Act / Assert
  expect(checkPanicSites(sources, "")).toEqual([]);
});

test.each([
  ["bootstrap root gate", BOOT+".rs", "#[cfg(test)]", ""],
  ["basic root gate", SYNC+".rs", "#[cfg(test)]", ""],
  ["qualification path", BOOT+"/tests.rs", '"qualification_tests.rs"', '"other.rs"'],
  ["support path", BOOT+"/tests.rs", '"test_support.rs"', '"other.rs"'],
  ["qualification declaration", BOOT+"/tests.rs", "mod qualification_tests;", "mod other;"],
  ["support declaration", BOOT+"/tests.rs", "mod test_support;", "mod other;"],
  ["basic declaration", SYNC+".rs", "mod basic_tests;", "mod other;"],
  ["dead root", BOOT+".rs", "#[cfg(test)]", "#[cfg(any())]\n#[cfg(test)]"],
  ["literal root decoy", BOOT+".rs", "#[cfg(test)]\nmod tests;", 'const DECOY: &str = "#[cfg(test)] mod tests;";\nmod tests;'],
  ["comment root decoy", BOOT+".rs", "#[cfg(test)]\nmod tests;", "/* #[cfg(test)] mod tests; */\nmod tests;"],
  ["nested root decoy", BOOT+".rs", "#[cfg(test)]\nmod tests;", "#[cfg(test)] mod decoy { #[cfg(test)] mod tests; }"],
  ["path comment decoy", BOOT+"/tests.rs", '#[path = "test_support.rs"]', '// #[path = "test_support.rs"]'],
] as const)("split authority rejects changed %s", (_name,file,before,after) => {
  // Arrange
  const sources = fixture();
  expect(sources.get(file)).toContain(before);
  sources.set(file,sources.get(file)!.replace(before,()=>after));
  // Act / Assert
  expect(()=>checkPanicSites(sources,"")).toThrow();
});

for(const child of ["storage/canonical_bootstrap/qualification_tests.rs","storage/canonical_bootstrap/test_support.rs","sync/canonical_genesis/basic_tests.rs"]) {
  test(`ordinary path alias cannot borrow ${child} test authority`, () => {
    // Arrange
    const sources=fixture();
    sources.set(LIVE,`#[path = "${child}"]\nmod ordinary_alias;`);
    // Act / Assert
    expect(()=>checkPanicSites(sources,"")).toThrow("ordinary inbound module alias");
  });
}

test("ordinary include alias cannot borrow test authority", () => {
  // Arrange
  const sources=fixture();
  sources.set(LIVE,'include!("storage/canonical_bootstrap/test_support.rs");');
  // Act / Assert
  expect(()=>checkPanicSites(sources,"")).toThrow("ordinary include!");
});

test.each(["module","include"] as const)("ordinary %s alias to the intermediate test owner cannot authorize leaves",kind=>{
  // Arrange
  const sources=fixture(),alias=kind==="module"?'#[path = "canonical_bootstrap/tests.rs"] mod ordinary_alias;':'include!("canonical_bootstrap/tests.rs");';
  sources.set(BOOT+".rs",sources.get(BOOT+".rs")!+"\n"+alias);
  // Act / Assert
  expect(()=>checkPanicSites(sources,"")).toThrow("ordinary");
});

test.each(["module","include"] as const)("absolute ordinary %s alias cannot borrow actual known-child authority",kind=>{
  // Arrange
  const sources=fixture(),absolute=join(process.cwd(),BOOT+"/test_support.rs");
  sources.set(LIVE,kind==="module"?`#[path = ${JSON.stringify(absolute)}] mod alias;`:`include!(${JSON.stringify(absolute)});`);
  // Act / Assert
  expect(()=>checkPanicSites(sources,"")).toThrow("absolute");
});

test.each(["C:\\source\\test_support.rs","\\\\host\\source\\test_support.rs"])("foreign absolute source paths refuse unsupported ownership: %s",path=>{
  // Arrange
  const sources=fixture();sources.set(LIVE,`#[path = ${JSON.stringify(path)}] mod alias;`);
  // Act / Assert
  expect(()=>checkPanicSites(sources,"")).toThrow("absolute");
});

test("dynamic ordinary include refuses unresolved source ownership", () => {
  // Arrange
  const sources=fixture();sources.set(LIVE,"include!(selected_path!());");
  // Act / Assert
  expect(()=>checkPanicSites(sources,"")).toThrow("unsupported include!");
});

test("ordinary concat include resolves its actual target", () => {
  // Arrange
  const sources=fixture();sources.set(LIVE,'include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/storage/canonical_bootstrap/test_support.rs"));');
  // Act / Assert
  expect(()=>checkPanicSites(sources,"")).toThrow("ordinary include!");
});

test("test-only alias does not authorize a changed required owner", () => {
  // Arrange
  const sources=fixture();sources.set(BOOT+".rs","mod tests;");
  sources.set(LIVE,'#[cfg(test)] #[path = "storage/canonical_bootstrap/test_support.rs"] mod decoy;');
  // Act / Assert
  expect(()=>checkPanicSites(sources,"")).toThrow();
});

test("comment and literal aliases do not revoke real test authority", () => {
  // Arrange
  const sources=fixture();sources.set(LIVE,'// #[path = "storage/canonical_bootstrap/test_support.rs"] mod alias;\nconst TEXT: &str = "include!(selected_path!());";');
  // Act / Assert
  expect(checkPanicSites(sources,"")).toEqual([]);
});

test("test macro function is excluded while production on both sides remains", () => {
  // Arrange
  const source='fn before() { panic!("before"); }\nmacro_rules! harness { () => { #[test] fn check() { None::<u8>.expect("test"); } }; }\nfn after() { unreachable!("after"); }';
  // Act
  const findings=checkPanicSites(new Map([[LIVE,source]]),"");
  // Assert
  expect(findings).toEqual([{file:LIVE,line:1,text:source.split("\n")[0]},{file:LIVE,line:3,text:source.split("\n")[2]}]);
});

test("cfg test item masking resumes after its balanced body", () => {
  // Arrange
  const source='#[cfg(test)]\nmod helpers { fn test() { panic!("test"); } }\nfn after() { todo!("production"); }';
  // Act
  const findings=checkPanicSites(new Map([[LIVE,source]]),"");
  // Assert
  expect(findings).toEqual([{file:LIVE,line:3,text:source.split("\n")[2]}]);
});

test("cfg test statement never hides the production suffix", () => {
  // Arrange
  const source='fn work() {\n #[cfg(test)] hook();\n None::<u8>.unwrap();\n}';
  // Act / Assert
  expect(checkPanicSites(new Map([[LIVE,source]]),"").map(r=>r.line)).toEqual([3]);
});

test("removing test function attribute restores its panic finding", () => {
  // Arrange
  const source='macro_rules! harness { () => { fn check() { None::<u8>.expect("production"); } }; }';
  // Act / Assert
  expect(checkPanicSites(new Map([[LIVE,source]]),"").map(r=>r.line)).toEqual([1]);
});

test("new filename patterns confer no test authority", () => {
  // Arrange
  const file=ROOT+"arbitrary_tests.rs";
  // Act / Assert
  expect(checkPanicSites(new Map([[file,crash]]),"").map(r=>r.file)).toEqual([file]);
});

test("existing tests.rs and tests directory boundaries remain explicit legacy policy", () => {
  // Arrange / Act / Assert
  expect(checkPanicSites(new Map([[ROOT+"tests.rs",crash],[ROOT+"tests/legacy.rs",crash]]),"")).toEqual([]);
});

test("comments and quoted Rust strings grant no item authority", () => {
  // Arrange
  const source='// #[cfg(test)]\nconst TEXT: &str = r#"#[test] fn decoy() { panic!(\"text\"); }"#;\nfn live() { panic!("production"); }';
  // Act / Assert
  expect(checkPanicSites(new Map([[LIVE,source]]),"").map(r=>r.line)).toEqual([3]);
});

test("malformed delimiters fail before exemptions", () => {
  // Arrange / Act / Assert
  expect(()=>checkPanicSites(new Map([[LIVE,'#[cfg(test)] mod tests { panic!("test");']]),"")).toThrow("unterminated");
});

test.each(["/* incomplete",'const TEXT: &str = "incomplete;', 'const TEXT: &str = r##"incomplete;']) ("incomplete lexeme refuses classification: %s",source=>{
  // Arrange / Act / Assert
  expect(()=>checkPanicSites(new Map([[LIVE,source]]),"")).toThrow("unterminated Rust");
});

test("ordinary variant and field attributes do not need test-item parsing", () => {
  // Arrange: the current soak report has tuple and named source fields.
  const source='#[derive(Debug, thiserror::Error)]\nenum Error {\n #[error("encode")] Encode(#[source] serde_json::Error),\n #[error("io")] Io { #[source] source: std::io::Error },\n}\nfn live() { panic!("production"); }';
  // Act / Assert
  expect(checkPanicSites(new Map([[LIVE,source]]),"").map(r=>r.line)).toEqual([6]);
});

test("test-only comma-delimited fields and variants do not hide later production", () => {
  // Arrange
  const source='struct State { #[cfg(test)] counter: usize, ordinary: usize }\nenum Mode { #[cfg(test)] Test(usize), Ordinary }\nfn live() { panic!("production"); }';
  // Act / Assert
  expect(checkPanicSites(new Map([[LIVE,source]]),"").map(r=>r.line)).toEqual([3]);
});

test("test function and impl generic commas do not truncate their owned bodies", () => {
  // Arrange: real authority helpers use comma-separated bounds and result types.
  const source='#[cfg(test)] fn test<S,V>() -> Result<S,V> where S: Send, V: Send, { panic!("test"); }\n#[cfg(test)] impl<S: Store,V: View> Handle<S,V> { fn test() { panic!("test"); } }\nfn live() { panic!("production"); }';
  // Act / Assert
  expect(checkPanicSites(new Map([[LIVE,source]]),"").map(r=>r.line)).toEqual([3]);
});

test("inline path overrides cannot hide an ordinary known-child alias", () => {
  // Arrange
  const sources=fixture();sources.set(LIVE,'#[path = "storage"] mod inline { #[path = "canonical_bootstrap/test_support.rs"] mod alias; }');
  // Act / Assert
  expect(()=>checkPanicSites(sources,"")).toThrow("inline module path ownership");
});

test("Rust lifetimes stay structural rather than becoming a character literal", () => {
  // Arrange
  const source='struct Mapping { names: &\'static [&\'static str] }\nfn live() { panic!("production"); }';
  // Act / Assert
  expect(checkPanicSites(new Map([[LIVE,source]]),"").map(r=>r.line)).toEqual([2]);
});

test.each([
  ["exact path and text",`${LIVE}|.expect("proven")|local invariant\n`,0],
  ["other path",`${ROOT}other.rs|.expect("proven")|local invariant\n`,1],
  ["other needle",`${LIVE}|.expect("other")|local invariant\n`,1],
  ["comment",`#${LIVE}|.expect("proven")|ignored\n`,1],
  ["empty", "\n",1],
  ["rationale is not a needle",`${LIVE}|not present|.expect("proven")\n`,1],
] as const)("allowlist preserves %s matching",(_name,allowlist,count)=>{
  // Arrange
  const source='fn live() { None::<u8>.expect("proven"); }';
  // Act / Assert
  expect(checkPanicSites(new Map([[LIVE,source]]),allowlist)).toHaveLength(count);
});

test("inner CLI reports real finding lines and fails without the allowlist", () => {
  // Arrange
  const root=mkdtempSync(join(tmpdir(),"open-bitcoin-panic-test-"));
  try {
    const target=join(root,LIVE);mkdirSync(dirname(target),{recursive:true});
    writeFileSync(target,'#[cfg(test)] fn test() { panic!("test"); }\nfn live() { panic!("production"); }\n');
    mkdirSync(join(root,"scripts"));writeFileSync(join(root,"scripts/panic-sites.allowlist"),"");
    // Act
    const proc=Bun.spawnSync([process.execPath,join(import.meta.dir,"check-panic-sites.ts")],{cwd:root,stdout:"pipe",stderr:"pipe"});
    const missing=Bun.spawnSync([process.execPath,join(import.meta.dir,"check-panic-sites.ts"),"missing.allowlist"],{cwd:root,stdout:"pipe",stderr:"pipe"});
    const absolute=join(root,"absolute.allowlist");writeFileSync(absolute,`${LIVE}|panic!("production")|fixture acceptance\n`);
    const overridden=Bun.spawnSync([process.execPath,join(import.meta.dir,"check-panic-sites.ts"),absolute],{cwd:root,stdout:"pipe",stderr:"pipe"});
    // Assert
    expect(proc.exitCode).toBe(1);expect(proc.stderr.toString()).toContain(`${LIVE}:2:fn live() { panic!("production"); }`);
    expect(missing.exitCode).toBe(1);expect(missing.stderr.toString()).toContain("check-panic-sites:");
    expect(overridden.exitCode).toBe(0);expect(overridden.stdout.toString()).toContain("no unclassified production");
  } finally { rmSync(root,{recursive:true,force:true}); }
});

test("source enumeration stays within regular first-party src files", () => {
  // Arrange
  const root=mkdtempSync(join(tmpdir(),"open-bitcoin-panic-files-"));
  try {
    for(const file of [LIVE,ROOT+"nested/tests.rs","packages/target/cache.rs","packages/bitcoin-knots/src/ignored.rs"]){mkdirSync(dirname(join(root,file)),{recursive:true});writeFileSync(join(root,file),"");}
    // Act / Assert
    expect(sourceFiles(root)).toEqual([LIVE,ROOT+"nested/tests.rs"]);
    expect(readFileSync(join(root,LIVE),"utf8")).toBe("");
  } finally { rmSync(root,{recursive:true,force:true}); }
});
