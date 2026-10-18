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
