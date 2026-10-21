function maskNonCode(source: string): string {
  const chars = [...source];
  let state: "code" | "line" | "block" | "string" | "char" | "raw" =
    "code";
  let blockDepth = 0;
  let rawHashCount = 0;
  let escaped = false;
  for (let index = 0; index < chars.length; index += 1) {
    const current = chars[index];
    const next = chars[index + 1] ?? "";
    if (state === "code") {
      if (current === "/" && next === "/") {
        state = "line";
        chars[index] = chars[index + 1] = " ";
        index += 1;
      } else if (current === "/" && next === "*") {
        state = "block";
        blockDepth = 1;
        chars[index] = chars[index + 1] = " ";
        index += 1;
      } else if (current === "r") {
        let delimiterEnd = index + 1;
        while (chars[delimiterEnd] === "#") delimiterEnd += 1;
        if (chars[delimiterEnd] === '"') {
          state = "raw";
          rawHashCount = delimiterEnd - index - 1;
          for (let offset = index; offset <= delimiterEnd; offset += 1) {
            chars[offset] = " ";
          }
          index = delimiterEnd;
        }
      } else if (current === '"') {
        state = "string";
        chars[index] = " ";
      } else if (
        current === "'" &&
        (chars[index + 2] === "'" ||
          (next === "\\" && chars[index + 3] === "'"))
      ) {
        state = "char";
        chars[index] = " ";
      }
      continue;
    }
    if (current === "\n" && state === "line") {
      state = "code";
      continue;
    }
    if (state === "block") {
      if (current === "/" && next === "*") {
        blockDepth += 1;
        chars[index + 1] = " ";
        index += 1;
      } else if (current === "*" && next === "/") {
        blockDepth -= 1;
        chars[index + 1] = " ";
        index += 1;
        if (blockDepth === 0) state = "code";
      }
      if (current !== "\n") chars[index] = " ";
      continue;
    }
    if (state === "line") {
      chars[index] = " ";
      continue;
    }
    if (state === "raw") {
      const closingHashes = chars.slice(
        index + 1,
        index + 1 + rawHashCount,
      );
      const closes =
        current === '"' &&
        closingHashes.length === rawHashCount &&
        closingHashes.every((character) => character === "#");
      if (current !== "\n") chars[index] = " ";
      if (closes) {
        for (let offset = 1; offset <= rawHashCount; offset += 1) {
          chars[index + offset] = " ";
        }
        index += rawHashCount;
        state = "code";
      }
      continue;
    }
    if (escaped) {
      escaped = false;
    } else if (current === "\\") {
      escaped = true;
    } else if (
      (state === "string" && current === '"') ||
      (state === "char" && current === "'")
    ) {
      state = "code";
    }
    if (current !== "\n") chars[index] = " ";
  }
  return chars.join("");
}

export function body(source: string, marker: string): string {
  const masked = maskNonCode(source);
  const start = masked.indexOf(marker);
  if (start < 0) return "";
  const brace = masked.indexOf("{", start);
  if (brace < 0) return "";
  let depth = 0;
  for (let index = brace; index < masked.length; index += 1) {
    if (masked[index] === "{") depth += 1;
    if (masked[index] !== "}") continue;
    depth -= 1;
    if (depth === 0) return masked.slice(brace + 1, index);
  }
  return "";
}

export function statementHasDisablingAttribute(
  maskedBody: string,
  statementIndex: number,
): boolean {
  let cursor = statementIndex - 1;
  while (
    cursor >= 0 &&
    (maskedBody[cursor] === " " || maskedBody[cursor] === "\t")
  ) {
    cursor -= 1;
  }
  if (cursor < 0) return false;
  if (maskedBody[cursor] !== "\n") {
    const lineStart = maskedBody.lastIndexOf("\n", cursor) + 1;
    return maskedBody.slice(lineStart, statementIndex).includes("#[");
  }

  let lineEnd = cursor;
  while (lineEnd > 0) {
    const previousBreak = maskedBody.lastIndexOf("\n", lineEnd - 1);
    const line = maskedBody.slice(previousBreak + 1, lineEnd);
    if (line.trim().length === 0) {
      lineEnd = previousBreak;
      continue;
    }
    return /^\s*#\[/.test(line);
  }
  return false;
}

export function directStatementIndex(
  maskedBody: string,
  statement: string,
): number {
  if (statement.length === 0) return -1;
  let depth = 0;
  let firstIndex = -1;
  for (let index = 0; index < maskedBody.length; index += 1) {
    const current = maskedBody[index];
    if (current === "{") {
      depth += 1;
      continue;
    }
    if (current === "}") {
      if (depth === 0) return -1;
      depth -= 1;
      continue;
    }
    if (
      depth === 0 &&
      firstIndex < 0 &&
      maskedBody.startsWith(statement, index) &&
      beginsDirectStatement(maskedBody, index) &&
      !statementHasDisablingAttribute(maskedBody, index)
    ) {
      firstIndex = index;
    }
  }
  return depth === 0 ? firstIndex : -1;
}

function beginsDirectStatement(source: string, index: number): boolean {
  for (let cursor = index - 1; cursor >= 0; cursor -= 1) {
    if (source[cursor] === "\n") return true;
    if (!/\s/.test(source[cursor])) return false;
  }
  return true;
}

export function exactStructFields(source: string, marker: string): string[] {
  const fields = body(source, marker).matchAll(
    /^\s*(?:pub(?:\([^)]*\))?\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:/gm,
  );
  return [...fields].map((match) => match[1]).sort();
}

export function sameFields(
  actual: string[],
  expected: readonly string[],
): boolean {
  return (
    actual.length === expected.length &&
    actual.every((field, index) => field === [...expected].sort()[index])
  );
}

export function hasAll(source: string, markers: readonly string[]): boolean {
  return markers.every((marker) => source.includes(marker));
}

export function addFailure(
  failures: string[],
  failed: boolean,
  diagnostic: string,
): void {
  if (failed && !failures.includes(diagnostic)) failures.push(diagnostic);
}

export function count(source: string, marker: string): number {
  return source.split(marker).length - 1;
}

/** Current ordinary refusal and explicitly synthetic recovery keep separate publication routes. */
export function contextStartupIsOrdered(canonical: string, network: string): boolean {
  const build = body(canonical, "pub(super) fn build_context_network(");
  const ordinary = body(canonical, "pub fn try_from_runtime_config_with_store(");
  const fixture = body(canonical, "pub fn from_fixture_runtime_config_with_store(");
  const ordinaryHandle = body(network, "pub fn from_runtime_config_with_network_handle(");
  const fixtureHandle = body(network, "pub(super) fn assemble_fixture_context(");
  const before = (source: string, first: string, last: string) => source.indexOf(first) >= 0 && source.indexOf(last) > source.indexOf(first);
  return before(build, "wallet_scan_chainstate_snapshot()?", "ManagedPeerNetwork::new_with_block_relay_activation(")
    && !build.includes("recover_mempool_snapshot")
    && before(ordinary, "require_ordinary_scope(chain)?", "build_wallet_state_with_store(")
    && before(ordinary, "Self::build_context_network(", "ManagedNetworkHandle::from_canonical_network(")
    && before(ordinary, "ManagedNetworkHandle::from_canonical_network(", "Self::from_runtime_config_with_network_handle(")
    && before(ordinaryHandle, "require_ordinary_scope(chain)?", "recover_mempool_snapshot_from_store_handle(")
    && before(ordinaryHandle, "recover_mempool_snapshot_from_store_handle(", "Self::assemble_context(")
    && before(fixture, "Self::build_context_network(", "Self::assemble_fixture_context(")
    && fixture.includes("ManagedNetworkHandle::from_network_fixture(network)")
    && before(fixtureHandle, "recover_fixture_mempool_snapshot_from_store_handle(", "Self::assemble_context(")
    && body(network, "fn assemble_context(").includes("Self {");
}

export function canonicalWorkerLifecycle(serve: string): boolean {
  const code = serve.replace(/\s+/g, "");
  return ["let canonical_genesis =", "let maybe_checkpoint_worker =", "let maybe_coins_flush_worker =", "let maybe_retry_worker ="]
    .every(marker => directStatementIndex(serve, marker) >= 0)
    && (code.match(/canonical_genesis=/g) ?? []).length === 1
    && code.includes("letcanonical_genesis=authoritative_runtime.maybe_prebuilt_context.is_some();")
    && code.includes("letmaybe_checkpoint_worker=ifcanonical_genesis{None}else{start_mempool_checkpoint_worker(handle.clone(),store.clone())};")
    && code.includes("letmaybe_coins_flush_worker=ifcanonical_genesis{None}else{start_coins_flush_worker(handle.clone(),store)};")
    && code.includes("letmaybe_retry_worker=(!canonical_genesis).then(||start_initial_broadcast_retry_worker(handle));")
    && (code.match(/letmaybe_(?:checkpoint|coins_flush|retry)_worker=/g) ?? []).length === 3;
}

export function productionBody(source: string, marker: string): string {
  const masked = body(`fn phase135_source() {\n${source}\n}`, "fn phase135_source()");
  const start = directStatementIndex(masked, marker);
  if (start < 0 || count(masked, marker) !== 1) return "";
  return body(masked.slice(start), marker);
}

export function ordinaryWorkerLifecycle(serve: string): boolean {
  const code = serve.replace(/\s+/g, "");
  return !/\b(?:canonical_genesis|maybe_prebuilt_context)\b/.test(serve)
    && code.includes("letmaybe_checkpoint_worker=start_mempool_checkpoint_worker(handle.clone(),store.clone());")
    && code.includes("letmaybe_coins_flush_worker=start_coins_flush_worker(handle.clone(),store);")
    && code.includes("letretry_worker=start_initial_broadcast_retry_worker(handle);");
}

export function shutdownCallIsExact(call: string, retry: string): boolean {
  const expected = `settle_daemon_shutdown(
    serve_result.map_err(Into::into),
    || Ok(maybe_sync_worker.map_or(Ok(()), |worker| worker.shutdown())?),
    || Ok(maybe_coins_flush_worker.map_or(Ok(()), |worker| worker.shutdown_always())?),
    ${retry}
    || Ok(maybe_checkpoint_worker.map_or(Ok(()), |worker| worker.shutdown_settle())?),
    || { maybe_runtime_store.map_or(Ok(()), |store| {
      store.mark_clean_shutdown(open_bitcoin_node::PersistMode::Sync)
    }) },
  )`;
  return call.replace(/\s+/g, "") === expected.replace(/\s+/g, "");
}
