#!/usr/bin/env bun
import { readFileSync, readdirSync } from "node:fs";
import { basename, dirname, isAbsolute, join, normalize, posix } from "node:path";
import { rustCode } from "./check-phase156-prune-coordination/rust-evidence.ts";

export type PanicFinding = { file: string; line: number; text: string };
type Span = { start: number; end: number };
type Item = Span & { declaration: number; attributes: string[]; test: boolean; dead: boolean };
type Parsed = { source: string; code: string; items: Item[] };
const compact = (s: string) => s.replace(/\s+/g, "");
const legacyTestFile = (file: string) => file.endsWith("/tests.rs") || file.includes("/tests/");
const inside = (spans: readonly Span[], at: number) => spans.some(s => s.start <= at && at < s.end);
const BASE = "packages/open-bitcoin-node/src/";
const BOOT = BASE + "storage/canonical_bootstrap";
const SYNC = BASE + "sync/canonical_genesis";
const CHILDREN = [BOOT + "/qualification_tests.rs", BOOT + "/test_support.rs", SYNC + "/basic_tests.rs"];
const TEST_OWNER = BOOT + "/tests.rs";
const AUTHORITY_FILES = [...CHILDREN,TEST_OWNER];
const absoluteSourcePath = (path: string) => isAbsolute(path) || /^[A-Za-z]:[\\/]|^\\\\/.test(path);

function close(code: string, start: number): number {
  const pairs: Record<string, string> = { "(": ")", "[": "]", "{": "}" };
  const stack: string[] = [];
  for (let i = start; i < code.length; i++) {
    const c = code[i];
    if (pairs[c]) stack.push(pairs[c]);
    else if (/[)\]}]/.test(c)) {
      if (stack.pop() !== c) throw Error("unbalanced Rust delimiters");
      if (!stack.length) return i + 1;
    }
  }
  throw Error("unterminated Rust item");
}

function itemEnd(code: string, start: number): number {
  const declaration=/^(?:pub(?:\([^)]*\))?\s+)?(?:(?:async|const|unsafe)\s+)*(?:fn|impl|struct|enum|mod|trait|const|static|type|use)\b/.test(code.slice(start));
  for (let i = start; i < code.length; i++) {
    if (code[i] === ";" || code[i] === ","&&!declaration) return i + 1;
    if (code[i] === "{") return close(code, i);
    if (code[i] === "(" || code[i] === "[") i = close(code, i) - 1;
  }
  throw Error("attribute has no complete owning item");
}

function requireCompleteLexemes(source: string): void {
  for (let i=0;i<source.length;) {
    if(source.startsWith("//",i)) { const end=source.indexOf("\n",i);i=end<0?source.length:end;continue; }
    if(source.startsWith("/*",i)) {
      let depth=1;i+=2;
      while(i<source.length&&depth) {
        if(source.startsWith("/*",i)){depth++;i+=2;}
        else if(source.startsWith("*/",i)){depth--;i+=2;}else i++;
      }
      if(depth)throw Error("unterminated Rust block comment");continue;
    }
    const raw=/^(?:b)?r(#+)?"/.exec(source.slice(i));
    if(raw&&(i===0||!/\w/.test(source[i-1]))) {
      const marker='"'+(raw[1]??""),end=source.indexOf(marker,i+raw[0].length);
      if(end<0)throw Error("unterminated Rust raw string");i=end+marker.length;continue;
    }
    const character=/^'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|[^\n])|[^'\\\n])'/.exec(source.slice(i));
    if(character){i+=character[0].length;continue;}
    if(source[i]==="'"&&source[i+1]==="\\")throw Error("unterminated Rust character literal");
    if(source[i]==='"') {
      i++;let complete=false;
      while(i<source.length) {
        if(source[i]==="\\"){i+=2;continue;}
        if(source[i++]==='"'){complete=true;break;}
      }
      if(!complete)throw Error("unterminated Rust string");continue;
    }
    i++;
  }
}

function parse(source: string): Parsed {
  requireCompleteLexemes(source);
  const code = rustCode(source), items: Item[] = [];
  // Derive no exemption from a structurally incomplete source stream.
  for (let i = 0; i < code.length; i++) {
    if (/[([{]/.test(code[i])) i = close(code, i) - 1;
    else if (/[)\]}]/.test(code[i])) throw Error("unbalanced Rust delimiters");
  }
  for (const match of code.matchAll(/#\s*\[/g)) {
    const start = match.index;
    if (items.some(item => start < item.declaration && start >= item.start)) continue;
    let cursor = start;
    const attributes: string[] = [];
    while (/^#\s*\[/.test(code.slice(cursor))) {
      const open = code.indexOf("[", cursor), end = close(code, open);
      attributes.push(source.slice(cursor, end));
      cursor = end;
      while (/\s/.test(code[cursor] ?? "")) cursor++;
    }
    const test = attributes.some(a => compact(a) === "#[cfg(test)]")
      || attributes.some(a => compact(a) === "#[test]") && /^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\b/.test(code.slice(cursor));
    const dead = attributes.some(a => compact(a) === "#[cfg(any())]");
    if(!test&&!dead&&!attributes.some(a=>/^#\s*\[\s*path\s*=/.test(a)))continue;
    items.push({ start, declaration: cursor, end: itemEnd(code, cursor), attributes, test, dead });
  }
  return { source, code, items };
}

function modulePath(source: Parsed, start: number): string | null {
  const item = source.items.find(i => i.declaration === start || /^pub(?:\([^)]*\))?\s+$/.test(source.code.slice(i.declaration,start)));
  const paths = item?.attributes.filter(a => /^#\s*\[\s*path\s*=/.test(a)) ?? [];
  if (paths.length > 1) throw Error("duplicate module path attributes");
  if (!paths.length) return null;
  const value = paths[0].match(/^#\s*\[\s*path\s*=\s*("(?:[^"\\]|\\.)*")\s*\]$/)?.[1];
  if (!value) throw Error("unsupported module path attribute");
  const path: string = JSON.parse(value);
  if(absoluteSourcePath(path))throw Error("unsupported absolute module source path");
  return path;
}

function selectedModule(parsed: Parsed, name: string, path: string | null, test: boolean): number {
  const matches = [...parsed.code.matchAll(new RegExp(`\\bmod\\s+${name}\\s*;`,"g"))];
  if (matches.length !== 1) throw Error(`missing or ambiguous test owner ${name}`);
  const at = matches[0].index;
  const depth = [...parsed.code.slice(0,at)].reduce((n,c) => n+(c==="{"?1:c==="}"?-1:0),0);
  const item = parsed.items.find(i => i.declaration === at);
  if (depth !== 0 || !item || item.dead || item.test !== test || modulePath(parsed,at) !== path) {
    throw Error(`test owner ${name} has changed gating or path`);
  }
  return at;
}

function includePath(expression: string, file: string): string {
  if (/^\s*"(?:[^"\\]|\\.)*"\s*$/.test(expression)) {
    const path: string=JSON.parse(expression);
    if(absoluteSourcePath(path))throw Error("unsupported absolute include! source path");
    return normalize(join(dirname(file),path));
  }
  const concat = /^\s*concat!\s*\(\s*env!\s*\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,([\s\S]*)\)\s*$/.exec(expression);
  if (!concat) throw Error(`${file}: unsupported include! source expression`);
  const parts = [...concat[1].matchAll(/"(?:[^"\\]|\\.)*"/g)];
  if (!parts.length || concat[1].replace(/"(?:[^"\\]|\\.)*"/g,"").replace(/[\s,]/g,"")) throw Error("unsupported include! path pieces");
  return normalize(file.slice(0,file.indexOf("/src/"))+parts.map(p => JSON.parse(p[0])).join(""));
}

function testChildren(parsed: ReadonlyMap<string,Parsed>): Set<string> {
  const accepted = new Set<string>(), expected = new Map<string,{ parent: string; at: number }>();
  if (CHILDREN.slice(0,2).some(p => parsed.has(p))) {
    const parent = parsed.get(BOOT+".rs"), tests = parsed.get(BOOT+"/tests.rs");
    if (!parent || !tests) throw Error("missing canonical bootstrap test owner");
    expected.set(TEST_OWNER,{parent:BOOT+".rs",at:selectedModule(parent,"tests",null,true)});
    for (const name of ["qualification_tests","test_support"]) {
      const child = BOOT+`/${name}.rs`;
      if (!parsed.has(child)) throw Error(`missing required test child ${child}`);
      expected.set(child,{parent:BOOT+"/tests.rs",at:selectedModule(tests,name,`${name}.rs`,false)});
    }
  }
  if (parsed.has(CHILDREN[2])) {
    const parent = parsed.get(SYNC+".rs");
    if (!parent) throw Error("missing canonical genesis test owner");
    expected.set(CHILDREN[2],{parent:SYNC+".rs",at:selectedModule(parent,"basic_tests",null,true)});
  }
  for (const [file,value] of parsed) {
    const inline = [...value.code.matchAll(/\bmod\s+(\w+)\s*\{/g)].map(m => ({start:m.index,end:close(value.code,value.code.indexOf("{",m.index)),name:m[1]}));
    if(inline.some(m=>modulePath(value,m.start)!==null))throw Error(`${file}: unsupported inline module path ownership`);
    for (const match of value.code.matchAll(/\bmod\s+(\w+)\s*;/g)) {
      const parents = inline.filter(s => s.start<match.index && match.index<s.end).sort((a,b)=>a.start-b.start);
      const directory = ["lib.rs","main.rs","mod.rs"].includes(basename(file)) ? dirname(file) : join(dirname(file),basename(file,".rs"));
      const path = modulePath(value,match.index);
      const candidates = path !== null ? [normalize(join(parents.length?join(directory,...parents.map(p=>p.name)):dirname(file),path))]
        : [join(directory,...parents.map(p=>p.name),match[1]+".rs"),join(directory,...parents.map(p=>p.name),match[1],"mod.rs")];
      for (const child of AUTHORITY_FILES.filter(p=>candidates.includes(p))) {
        const owner = expected.get(child);
        if (owner?.parent === file && owner.at === match.index) { accepted.add(child); continue; }
        if (!inside(value.items.filter(i=>i.test&&!i.dead),match.index)) throw Error(`${child}: ordinary inbound module alias in ${file}`);
      }
    }
    for (const match of value.code.matchAll(/\binclude!\s*\(/g)) {
      if (inside(value.items.filter(i=>i.test),match.index) || legacyTestFile(file)) continue;
      const open=value.code.indexOf("(",match.index),end=close(value.code,open);
      const child=includePath(value.source.slice(open+1,end-1),file);
      if (AUTHORITY_FILES.includes(child)) throw Error(`${child}: ordinary include! alias in ${file}`);
    }
  }
  for (const child of expected.keys()) if (!accepted.has(child)) throw Error(`unresolved test child ${child}`);
  return accepted;
}

function allowed(finding: PanicFinding, allowlist: string): boolean {
  return allowlist.split("\n").some(line => {
    const [path,needle] = line.split("|");
    return path !== "" && !path.startsWith("#") && finding.file === path && finding.text.includes(needle ?? "");
  });
}

/** A source policy with retained legacy test-directory exclusions, not a Rust compiler. */
export function checkPanicSites(sources: ReadonlyMap<string,string>, allowlist: string): PanicFinding[] {
  const parsed = new Map([...sources].map(([p,s])=>{
    try { return [p,parse(s)] as const; }
    catch(error) { throw Error(`${p}: ${error instanceof Error?error.message:String(error)}`); }
  }));
  const children = testChildren(parsed), findings: PanicFinding[] = [];
  for (const [file,value] of parsed) {
    // Preserve the old policy boundary; new names earn no such exemption.
    if (legacyTestFile(file) || children.has(file)) continue;
    const chars=value.code.split("");
    for (const item of value.items.filter(i=>i.test)) for(let at=item.start;at<item.end;at++) if(chars[at]!=="\n")chars[at]=" ";
    const original=value.source.split("\n");
    for(const[index,line]of chars.join("").split("\n").entries()) {
      if(!/(^|[^\w])(unwrap|expect)\s*\(|(panic|unreachable|todo|unimplemented)!\s*\(/.test(line))continue;
      const finding={file,line:index+1,text:original[index]};
      if(!allowed(finding,allowlist))findings.push(finding);
    }
  }
  return findings.sort((a,b)=>a.file<b.file?-1:a.file>b.file?1:a.line-b.line);
}

/** Enumerate the original find predicate's regular first-party src files. */
export function sourceFiles(root="."): string[] {
  const files:string[]=[];
  const visit=(directory:string)=>{
    for(const entry of readdirSync(join(root,directory),{withFileTypes:true})) {
      const path=posix.join(directory,entry.name);
      if(entry.isDirectory())visit(path);
      else if(entry.isFile()&&path.endsWith(".rs"))files.push(path);
    }
  };
  for(const entry of readdirSync(join(root,"packages"),{withFileTypes:true})) {
    if(!entry.isDirectory()||!entry.name.startsWith("open-bitcoin-"))continue;
    const path=posix.join("packages",entry.name);
    if(readdirSync(join(root,path),{withFileTypes:true}).some(e=>e.name==="src"&&e.isDirectory()))visit(posix.join(path,"src"));
  }
  return files.sort();
}

export function runPanicCheck(root:string,allowlistPath:string): PanicFinding[] {
  const sources=new Map(sourceFiles(root).map(p=>[p,readFileSync(join(root,p),"utf8")]));
  return checkPanicSites(sources,readFileSync(isAbsolute(allowlistPath)?allowlistPath:join(root,allowlistPath),"utf8"));
}

if(import.meta.main) {
  const allowlist=process.argv[2]??"scripts/panic-sites.allowlist";
  try {
    const findings=runPanicCheck(".",allowlist);
    if(findings.length) {
      console.error("Unclassified production panic-like sites found.\n");
      for(const row of findings)console.error(`${row.file}:${row.line}:${row.text}`);
      console.error(`\nEither replace the crash path with typed control flow or add a narrow entry to ${allowlist}:\npath|needle|rationale`);
      process.exitCode=1;
    }else console.log("check-panic-sites.sh: no unclassified production panic-like sites");
  }catch(error) {
    console.error(`check-panic-sites: ${error instanceof Error?error.message:String(error)}`);
    process.exitCode=1;
  }
}
