// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Handle-owning configured fixture and handle-free provenance handoff.

use super::{core_types, recipe};
use crate::{AuthoritativeNetworkRuntime, open_authoritative_network_runtime, open_runtime_store};
use core_types::{
    chainstate::{
        BasicFilterInputs, ChainPosition, CoinsView, FlushMode, FlushPolicyTime,
        HistoricalBlockUndo, PruneMode,
    },
    consensus::{ScriptVerifyFlags, block_hash},
};
use open_bitcoin_node::{
    DurableSyncRuntime, FjallChainstateStore, FjallCoinsView, FjallNodeStore, PersistMode,
    SyncNetwork, SyncRuntimeConfig,
};
use open_bitcoin_rpc::{ManagedRpcContext, config::RuntimeConfig, http::RpcHttpState};
use std::{path::PathBuf, sync::Arc, time::Instant};

type SharedContext =
    Arc<tokio::sync::Mutex<ManagedRpcContext<FjallChainstateStore, FjallCoinsView>>>;

struct SeedCleanup {
    maybe_path: Option<PathBuf>,
}

impl Drop for SeedCleanup {
    fn drop(&mut self) {
        if let Some(path) = &self.maybe_path {
            crate::tests::remove_dir_if_exists(path);
        }
    }
}

pub(crate) struct Provenance {
    pub recipe_id: String,
    pub ordered_input_digest: String,
    pub receipts: Vec<ChainPosition>,
    pub oracle: serde_json::Value,
    pub seed_ms: u128,
}

pub(crate) struct Phase162Handoff {
    pub path: PathBuf,
    pub recipe: recipe::Recipe,
    pub provenance: Provenance,
}

pub(crate) struct Phase162Fixture {
    pub path: PathBuf,
    pub recipe: recipe::Recipe,
    pub provenance: Provenance,
    pub config: RuntimeConfig,
    pub store: FjallNodeStore,
    pub opened: AuthoritativeNetworkRuntime<FjallChainstateStore, FjallCoinsView>,
    pub state: RpcHttpState<FjallChainstateStore, FjallCoinsView>,
    pub shared: SharedContext,
}

impl Phase162Fixture {
    pub fn seed() -> Self {
        Self::seed_count(1001)
    }

    pub fn seed_count(count: u32) -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let id = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = crate::tests::temp_store_path(&format!("phase162-daemon-{id}"));
        std::fs::create_dir(&path).expect("unique fixture datadir");
        let mut cleanup = SeedCleanup {
            maybe_path: Some(path.clone()),
        };
        let started = Instant::now();
        let runtime = DurableSyncRuntime::open(
            FjallNodeStore::open(&path).expect("fresh real Fjall"),
            SyncRuntimeConfig {
                network: SyncNetwork::Regtest,
                ..Default::default()
            },
        )
        .expect("unindexed actual runtime");
        let params = runtime.config().network.consensus_params();
        let recipe = recipe::Recipe::new(params, ScriptVerifyFlags::P2SH, count);
        let mut receipts = Vec::with_capacity(recipe.blocks.len());
        for block in &recipe.blocks {
            runtime
                .store()
                .save_block(block, PersistMode::Sync)
                .expect("ordinary body persistence");
            let receipt = runtime
                .network_handle()
                .connect_local_block(block, recipe.flags, recipe.params)
                .expect("normal-maturity managed acceptance");
            assert_eq!(receipt.height as usize, receipts.len());
            assert_eq!(receipt.block_hash, block_hash(&block.header));
            receipts.push(receipt);
        }
        assert!(
            runtime
                .network_handle()
                .maybe_basic_index_summary()
                .expect("disabled progress")
                .is_none()
        );
        let flushed = runtime
            .network_handle()
            .flush_coins(FlushMode::Always, FlushPolicyTime::new(500_000), u64::MAX)
            .expect("genuine coins undo metadata flush");
        assert!(flushed.wrote_coins);
        assert_eq!(
            runtime
                .store()
                .coins_view()
                .best_block()
                .expect("durable B"),
            receipts.last().map(|position| position.block_hash)
        );
        for (height, receipt) in receipts.iter().enumerate().skip(1) {
            let undo = runtime
                .store()
                .load_undo(receipt.block_hash)
                .expect("persisted undo")
                .expect("actual accepted undo");
            let inputs = BasicFilterInputs::from_historical(
                &recipe.blocks[height],
                receipt,
                Some(HistoricalBlockUndo {
                    block_hash: receipt.block_hash,
                    undo: &undo,
                }),
            )
            .expect("actual durable historical facts");
            assert_eq!(
                inputs
                    .spent_scripts()
                    .map(<[u8]>::to_vec)
                    .collect::<Vec<_>>(),
                recipe.spent_scripts[height]
            );
        }
        drop(runtime);
        let oracle = if count == 1001 {
            super::independent_oracle(&recipe)
        } else {
            assert_eq!(count, 102, "bounded wallet preflight only");
            let raw: Vec<_> = recipe
                .blocks
                .iter()
                .map(|block| core_types::codec::encode_block(block).expect("short-prefix raw body"))
                .collect();
            let digest = super::hex(&core_types::consensus::crypto::Sha256::digest(
                &serde_json::to_vec(&raw).expect("ordered short-prefix bytes"),
            ));
            serde_json::json!({"recipe_id": recipe::RECIPE_ID, "ordered_input_digest": digest})
        };
        let provenance = Provenance {
            recipe_id: oracle["recipe_id"].as_str().expect("recipe ID").to_owned(),
            ordered_input_digest: oracle["ordered_input_digest"]
                .as_str()
                .expect("ordered digest")
                .to_owned(),
            receipts,
            oracle,
            seed_ms: started.elapsed().as_millis(),
        };
        let fixture = Self::open(Phase162Handoff {
            path,
            recipe,
            provenance,
        });
        cleanup.maybe_path = None;
        fixture
    }

    pub fn open(handoff: Phase162Handoff) -> Self {
        Self::open_with_prune(handoff, 1)
    }

    pub fn open_with_prune(handoff: Phase162Handoff, prune: u64) -> Self {
        Self::open_policy(handoff, prune, false, false, false, &["in"])
    }

    pub fn open_serving(
        handoff: Phase162Handoff,
        general: bool,
        listener: bool,
        permissions: &[&str],
    ) -> Self {
        Self::open_policy(handoff, 550, general, listener, false, permissions)
    }

    pub fn open_body_serving(handoff: Phase162Handoff) -> Self {
        Self::open_policy(
            handoff,
            550,
            false,
            true,
            true,
            &["in", "download", "blockfilters"],
        )
    }

    fn open_policy(
        handoff: Phase162Handoff,
        prune: u64,
        general: bool,
        listener: bool,
        body_serving: bool,
        permissions: &[&str],
    ) -> Self {
        assert!(
            matches!(prune, 1 | 550),
            "only approved actual prune settings"
        );
        let Phase162Handoff {
            path,
            recipe,
            provenance,
        } = handoff;
        std::fs::write(
            path.join("open-bitcoin.jsonc"),
            format!("{{\"prune\":{prune}}}"),
        )
        .expect("actual parsed prune configuration");
        let mut args = vec![
            format!("-datadir={}", path.display()).into(),
            "-regtest".into(),
            "-blockfilterindex=basic".into(),
            "-rpcuser=phase162".into(),
            "-rpcpassword=private-test-password".into(),
            format!("-peerblockfilters={}", u8::from(general)).into(),
            format!("-openbitcoininbound={}", u8::from(listener)).into(),
            format!("-openbitcoinblockserving={}", u8::from(body_serving)).into(),
            "-openbitcoincompactrelay=0".into(),
            "-openbitcoinlisten=127.0.0.1:0".into(),
            "-openbitcoinmaxinbound=2".into(),
            "-openbitcoinreservedslots=1".into(),
        ];
        if permissions != ["in"] {
            args.push(
                format!(
                    "-openbitcoininboundpermissionclass=phase162@127.0.0.1={}",
                    permissions.join(",")
                )
                .into(),
            );
        }
        let config = open_bitcoin_rpc::config::load_runtime_config_for_args(&args, &path)
            .expect("actual daemon config parser");
        assert!(!config.sync.is_enabled());
        assert_eq!(config.inbound.enabled, listener);
        assert_eq!(config.peer_block_filters, general);
        assert_eq!(config.inbound.listen_addresses, ["127.0.0.1:0"]);
        assert_eq!(config.inbound.max_peers, 2);
        assert_eq!(config.inbound.reserved_slots, 1);
        assert!(!config.inbound.allow_public);
        let expected_classes = if permissions == ["in"] {
            open_bitcoin_network::PeerPermissionClassRegistry::default()
        } else {
            open_bitcoin_network::PeerPermissionClassRegistry::new([
                open_bitcoin_network::ParsedPeerPermissionClass::parse(
                    "phase162",
                    ["127.0.0.1"],
                    permissions.iter().copied(),
                )
                .expect("independent requested tokens"),
            ])
        };
        assert_eq!(config.inbound.permission_classes, expected_classes);
        assert_eq!(
            config.prune_mode,
            if prune == 1 {
                PruneMode::ManualOnly
            } else {
                PruneMode::Automatic { target_mib: 550 }
            }
        );
        assert_eq!(config.relay, RuntimeConfig::default().relay);
        let mut expected_serving = RuntimeConfig::default().block_serving;
        expected_serving.block_serving.enabled = body_serving;
        assert_eq!(config.block_serving, expected_serving);
        assert_eq!(
            config.sync.runtime.network.consensus_params(),
            recipe.params
        );
        let store = open_runtime_store(&config)
            .expect("actual store selection")
            .expect("durable store");
        let opened = open_authoritative_network_runtime(&config, Some(store.clone()))
            .expect("configured BASIC startup")
            .expect_durable();
        let context = ManagedRpcContext::from_runtime_config_with_network_handle(
            &config,
            opened.network.clone(),
            Some(store.clone()),
        )
        .expect("actual daemon RPC context");
        let shared = Arc::new(tokio::sync::Mutex::new(context));
        let state = open_bitcoin_rpc::http::build_http_state_with_shared_context(
            config.rpc_server.auth.clone(),
            Arc::clone(&shared),
        )
        .expect("actual authenticated HTTP state");
        Self {
            path,
            recipe,
            provenance,
            config,
            store,
            opened,
            state,
            shared,
        }
    }

    pub fn assert_initial_prefix(&self) {
        assert_eq!(self.recipe.params, SyncNetwork::Regtest.consensus_params());
        assert_eq!(self.recipe.params.coinbase_maturity, 100);
        assert_eq!(self.provenance.recipe_id, recipe::RECIPE_ID);
        assert_eq!(self.provenance.ordered_input_digest.len(), 64);
        assert_eq!(
            self.provenance.oracle["cases"]
                .as_array()
                .expect("raw cases")
                .len(),
            1005
        );
        assert_eq!(self.provenance.receipts.len(), 1001);
        let progress = self
            .opened
            .network
            .maybe_basic_index_summary()
            .expect("read progress")
            .expect("real owner");
        assert_eq!(
            self.opened
                .network
                .maybe_chain_tip()
                .expect("tip")
                .expect("accepted tip")
                .height,
            1000
        );
        assert_eq!(progress.best_block_height, 7);
        assert!(!progress.synced);
        assert_eq!(
            self.opened
                .network
                .network_info()
                .expect("network")
                .connected_peers,
            0
        );
        assert_eq!(
            self.opened
                .network
                .network_info()
                .expect("local service policy")
                .local_services_bits
                & (1 << 6),
            0
        );
        assert!(
            self.opened
                .network
                .peer_manager_snapshot()
                .expect("peers")
                .peer_ids()
                .is_empty()
        );
        assert_eq!(
            self.store.coins_view().best_block().expect("coins B"),
            self.provenance
                .receipts
                .last()
                .map(|position| position.block_hash)
        );
        assert_eq!(
            self.store
                .hydrate_chainstate_for_open()
                .expect("actual durable metadata")
                .expect("persisted accepted chainstate")
                .active_chain,
            self.provenance.receipts
        );
        let funding = self
            .store
            .coins_view()
            .get_coin(&self.recipe.wallet_outpoint)
            .expect("genuine funded output read")
            .expect("unspent descriptor output");
        assert_eq!(funding.created_height, 20);
        assert_eq!(funding.output.script_pubkey, self.recipe.wallet_script);
        assert_eq!(funding.output.value.to_sats(), recipe::WALLET_SATS);
        let open_bitcoin_node::BasicFilterQuery::Found(record) = self
            .opened
            .network
            .basic_filter_query(self.provenance.receipts[7].block_hash)
            .expect("startup record read")
        else {
            panic!("actual startup record")
        };
        assert_eq!(
            self.provenance.oracle["cases"][7]["encoded"],
            super::hex(record.encoded_bytes())
        );
        assert_eq!(
            self.provenance.oracle["cases"][7]["header_raw"],
            super::hex(record.identity().filter_header().as_bytes())
        );
    }

    /// The returned value contains raw data and provenance, never a store/authority clone.
    pub fn close(self) -> Phase162Handoff {
        let Self {
            path,
            recipe,
            provenance,
            config,
            store,
            opened,
            state,
            shared,
        } = self;
        drop(state);
        drop(shared);
        drop(opened);
        drop(store);
        drop(config);
        Phase162Handoff {
            path,
            recipe,
            provenance,
        }
    }
}
