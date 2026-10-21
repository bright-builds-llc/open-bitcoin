import { hasOrdinaryRuntimeOwner } from "./impl-attributes.ts";
import {
  compact,
  maybeFunction,
  ordinaryRust,
} from "../check-phase157-index-catch-up/rust-evidence.ts";

/** Proves requested-body persistence and failure isolation before active-tip announcements. */
export function hasDurableSaveBeforeTip(response: string): boolean {
  const maybeDisposition = maybeFunction(response, "record_block_disposition", true);
  const maybeActiveTip = maybeFunction(response, "block_is_current_active_tip", true);
  if (maybeDisposition === undefined || maybeActiveTip === undefined) return false;
  const code = ordinaryRust(response);
  const maybeScope = maybeRuntimeScope(code, maybeDisposition.start);
  if (maybeScope === undefined) return false;
  if (maybeRuntimeScope(code, maybeActiveTip.start) !== maybeScope) return false;
  if (compact(maybeActiveTip.body) !== compact(`
    let block_hash = block_hash(&block.header);
    let maybe_active_tip = self.network.maybe_chain_tip()?;
    Ok(maybe_active_tip.is_some_and(|tip| tip.block_hash == block_hash))
  `)) return false;
  const body = compact(maybeDisposition.body);
  const prefix = compact(`
    if was_requested && let Some(block) = maybe_block {
      if let Err(error) = self.store.save_block(block, self.config.persist_mode) {
  `);
  if (!body.startsWith(prefix)) return false;
  let depth = 1;
  let end = prefix.length;
  while (end < body.length && depth > 0) {
    if (body[end] === "{") depth += 1;
    if (body[end] === "}") depth -= 1;
    end += 1;
  }
  if (depth !== 0) return false;
  const failure = body.slice(prefix.length, end - 1);
  // Both failures must exit; notification is permitted without granting accepted progress.
  const expectedFailure = compact(ordinaryRust(`
    if let Err(notification) = self.network.note_basic_index_failure(
      crate::chainstate::filter_index::AcceptedBasicIndexFailure::RequiredBody,
    ) {
      return Err(crate::storage::StorageError::BackendFailure {
        namespace: crate::storage::StorageNamespace::BlockIndex,
        message: format!("{error}; BASIC failure notification: {notification}"),
        action: crate::storage::StorageRecoveryAction::Restart,
      }.into());
    }
    return Err(error.into());
  `));
  if (failure !== expectedFailure) return false;
  return body.slice(end) === compact(`
    self.network.note_local_block_hash(block_hash(&block.header))?;
    }
    match disposition {
      BlockConnectDisposition::Connected(_) => {
        if was_requested {
          let Some(block) = maybe_block else { return Ok(()); };
          progress.record_accepted_block();
          if self.block_is_current_active_tip(block)? {
            self.queue_durable_tip_advanced(block.clone());
          }
        }
      }
      BlockConnectDisposition::Duplicate(_) => progress.record_duplicate_block(),
      BlockConnectDisposition::Disconnected { .. } => progress.record_disconnected_block(),
      BlockConnectDisposition::NonExtending { .. } => progress.record_non_extending_block(),
    }
    Ok(())
  `);
}

function maybeRuntimeScope(code: string, functionStart: number): number | undefined {
  const scopes: number[] = [];
  for (let index = 0; index < functionStart; index += 1) {
    if (code[index] === "{") scopes.push(index);
    if (code[index] === "}") scopes.pop();
  }
  const maybeScope = scopes.at(-1);
  if (maybeScope === undefined) return undefined;
  return hasOrdinaryRuntimeOwner(code.slice(0, maybeScope)) ? maybeScope : undefined;
}

// Preserve the reviewed durable-trigger prefix while extending its source analysis.
import { rustCode } from "../check-phase157-index-catch-up/rust-evidence.ts";

/** Bind earned genesis context and every transport owner to the consumed durable runtime. */
export function hasCanonicalDaemonComposition(daemon: string, canonicalSource: string): boolean {
  const main = maybeOwnedFunction(daemon, "main");
  const application = maybeOwnedFunction(daemon, "run_application");
  const serve = maybeOwnedFunction(daemon, "serve_authoritative_runtime");
  const factory = maybeOwnedFunction(canonicalSource, "open_authoritative_network_runtime");
  const preflight = maybeOwnedFunction(canonicalSource, "preflight");
  const assembly = maybeOwnedFunction(canonicalSource, "assemble_durable_runtime");
  if (!main || !application || !serve || !factory || !preflight || !assembly) return false;
  const app = compact(application.body);
  const assembled = compact(assembly.body);
  const factoryHeader = ordinaryRust(canonicalSource).slice(factory.start, canonicalSource.indexOf("{", factory.start));
  return ownsDeclaration(daemon, "mod canonical_startup;")
    && hasOwningAttribute(daemon, ordinaryRust(daemon).indexOf("mod canonical_startup;"), '#[path = "open_bitcoind/canonical_startup.rs"]')
    && ownsDeclaration(daemon, "use canonical_startup::{open_authoritative_network_runtime, open_runtime_store};")
    && hasOwningAttribute(daemon, application.start, "#[tokio::main]")
    && compact(main.body) === compact(`
      if let Some(code) = FjallNodeStore::maybe_run_inspection_worker() { std::process::exit(code); }
      run_application()
    `)
    && factoryHeader.includes("maybe_opened: Option<OpenedCanonicalGenesis>")
    && compact(preflight.body) === compact(`
      runtime.validate_peer_block_filters().map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?;
      let chain = selected_canonical_chain(runtime).map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?;
      let activation = runtime.sync.is_enabled() || runtime.inbound.enabled || runtime.relay.enabled
        || runtime.block_serving.block_serving.enabled || runtime.block_serving.compact_relay.enabled;
      if activation { chain.spec().require_ordinary_history().map_err(|_| compatibility_refusal(runtime, chain))?; }
      Ok(chain)
    `)
    && compact(factory.body) === compact(`
      let chain = preflight(runtime)?;
      let Some(opened) = maybe_opened else { return Err(compatibility_refusal(runtime, chain)); };
      let (context, sync_runtime) = ManagedRpcContext::from_canonical_genesis_runtime_config(runtime, opened)
        .map_err(|_| compatibility_refusal(runtime, chain))?;
      match assemble_durable_runtime(runtime, sync_runtime)? {
        OpenedAuthoritativeRuntime::Durable(mut authoritative) => {
          authoritative.maybe_prebuilt_context = Some(context);
          Ok(OpenedAuthoritativeRuntime::Durable(authoritative))
        }
        #[cfg(test)] OpenedAuthoritativeRuntime::Transient(_) => Err(compatibility_refusal(runtime, chain)),
      }
    `)
    && ownsDeclaration(application.body, "let opened =")
    && app.includes(compact(`
      let maybe_runtime_store = open_runtime_store(&runtime)?;
      let opened = open_authoritative_network_runtime(&runtime, maybe_runtime_store)?;
      let maybe_runtime_store = match &opened {
        OpenedAuthoritativeRuntime::Durable(authoritative) => authoritative.maybe_sync_runtime.as_ref().map(|sync| sync.store().clone()),
        #[cfg(test)] OpenedAuthoritativeRuntime::Transient(_) => None,
      };
    `))
    && ownsDeclaration(application.body, "match opened {")
    && app.endsWith(compact(`match opened {
      OpenedAuthoritativeRuntime::Durable(authoritative_runtime) => {
        serve_authoritative_runtime(runtime, bind_address, auth, maybe_runtime_store, authoritative_runtime,).await
      }
      #[cfg(test)] OpenedAuthoritativeRuntime::Transient(authoritative_runtime) => {
        serve_authoritative_runtime(runtime, bind_address, auth, maybe_runtime_store, authoritative_runtime,).await
      }
    }`))
    && assembled.startsWith("letmutnetwork=sync_runtime.network_handle();")
    && (assembly.body.match(/\bnetwork\s*=/g) ?? []).length === 1
    && !/\blet\s+(?:mut\s+)?sync_runtime\b|\breturn\b/.test(assembly.body)
    && assembled.endsWith(compact(`Ok(OpenedAuthoritativeRuntime::Durable(
      AuthoritativeNetworkRuntime {
        maybe_prebuilt_context: None,
        network,
        peer_identity_authority: sync_runtime.peer_identity_authority(),
        announcement_outboxes: sync_runtime.announcement_outboxes(),
        maybe_sync_runtime: Some(sync_runtime),
      },
    ))`))
    && ownsDeclaration(serve.body, "let context =")
    && ownsDeclaration(serve.body, "let shared_context =")
    && compact(serve.body).includes(compact(`
      let canonical_genesis = authoritative_runtime.maybe_prebuilt_context.is_some();
      let context = match authoritative_runtime.maybe_prebuilt_context.take() {
        Some(context) => context,
        None => ManagedRpcContext::from_runtime_config_with_network_handle(
          &runtime, authoritative_runtime.network.clone(), maybe_runtime_store.clone(),
        )?,
      };
      let shared_context = Arc::new(tokio::sync::Mutex::new(context));
    `))
    && ownsDeclaration(serve.body, "let mut inbound_listener =")
    && compact(serve.body).includes(compact(`
      let mut inbound_listener = start_inbound_listener_for_runtime_with_context_and_announcements(
        &runtime, Arc::clone(&shared_context), authoritative_runtime.peer_identity_authority,
        authoritative_runtime.announcement_outboxes, authoritative_runtime.network,
      ).await;
    `));
}

function maybeOwnedFunction(source: string, symbol: string): ReturnType<typeof maybeFunction> {
  const maybeSelected = maybeFunction(source, symbol, true);
  if (maybeSelected === undefined) return undefined;
  const code = ordinaryRust(source);
  const prefix = code.slice(0, maybeSelected.start);
  if (prefix.split("{").length !== prefix.split("}").length) return undefined;
  const attributes = prefix.slice(Math.max(prefix.lastIndexOf(";"), prefix.lastIndexOf("}")) + 1);
  return /#\[\s*cfg\s*\(/.test(attributes) ? undefined : maybeSelected;
}

function ownsDeclaration(source: string, marker: string): boolean {
  const code = ordinaryRust(source);
  const start = code.indexOf(marker);
  if (start < 0 || code.split(marker).length !== 2) return false;
  const prefix = code.slice(0, start);
  if (prefix.split("{").length !== prefix.split("}").length) return false;
  return !/#\[\s*cfg\s*\(/.test(prefix.slice(Math.max(prefix.lastIndexOf(";"), prefix.lastIndexOf("}")) + 1));
}

function hasOwningAttribute(source: string, declarationStart: number, expected: string): boolean {
  const code = rustCode(source);
  if (declarationStart < 0) return false;
  const maybeAsync = /\basync\s*$/.exec(code.slice(0, declarationStart));
  let cursor = maybeAsync?.index ?? declarationStart;
  while (cursor > 0) {
    while (cursor > 0 && /\s/.test(code[cursor - 1])) cursor -= 1;
    if (code[cursor - 1] !== "]") return false;
    const end = cursor;
    let depth = 1;
    cursor -= 1;
    while (cursor > 0 && depth > 0) {
      cursor -= 1;
      if (code[cursor] === "]") depth += 1;
      if (code[cursor] === "[") depth -= 1;
    }
    if (depth !== 0 || code[cursor - 1] !== "#") return false;
    cursor -= 1;
    if (source.slice(cursor, end) === expected) return true;
  }
  return false;
}
