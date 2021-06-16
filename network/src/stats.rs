// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use metrics::{GaugeValue, Key, Recorder, Unit};

use std::sync::atomic::{AtomicU64, Ordering};

pub const INBOUND_ALL_SUCCESSES: &str = "snarkos_inbound_all_successes_total";
pub const INBOUND_ALL_FAILURES: &str = "snarkos_inbound_all_failures_total";
pub const INBOUND_BLOCKS: &str = "snarkos_inbound_blocks_total";
pub const INBOUND_GETBLOCKS: &str = "snarkos_inbound_getblocks_total";
pub const INBOUND_GETMEMORYPOOL: &str = "snarkos_inbound_getmemorypool_total";
pub const INBOUND_GETPEERS: &str = "snarkos_inbound_getpeers_total";
pub const INBOUND_GETSYNC: &str = "snarkos_inbound_getsync_total";
pub const INBOUND_MEMORYPOOL: &str = "snarkos_inbound_memorypool_total";
pub const INBOUND_PEERS: &str = "snarkos_inbound_peers_total";
pub const INBOUND_PINGS: &str = "snarkos_inbound_pings_total";
pub const INBOUND_PONGS: &str = "snarkos_inbound_pongs_total";
pub const INBOUND_SYNCS: &str = "snarkos_inbound_syncs_total";
pub const INBOUND_SYNCBLOCKS: &str = "snarkos_inbound_syncblocks_total";
pub const INBOUND_TRANSACTIONS: &str = "snarkos_inbound_transactions_total";
pub const INBOUND_UNKNOWN: &str = "snarkos_inbound_unknown_total";

pub const OUTBOUND_ALL_SUCCESSES: &str = "snarkos_outbound_all_successes_total";
pub const OUTBOUND_ALL_FAILURES: &str = "snarkos_outbound_all_failures_total";

pub const CONNECTIONS_ALL_ACCEPTED: &str = "snarkos_connections_all_accepted_total";
pub const CONNECTIONS_ALL_INITIATED: &str = "snarkos_connections_all_initiated_total";
pub const CONNECTIONS_ALL_REJECTED: &str = "snarkos_connections_all_rejected_total";
pub const CONNECTIONS_CONNECTING: &str = "snarkos_connections_connecting_total";
pub const CONNECTIONS_CONNECTED: &str = "snarkos_connections_connected_total";
pub const CONNECTIONS_DISCONNECTED: &str = "snarkos_connections_disconnected_total";

pub const HANDSHAKES_FAILURES_INIT: &str = "snarkos_handshakes_failures_init_total";
pub const HANDSHAKES_FAILURES_RESP: &str = "snarkos_handshakes_failures_resp_total";
pub const HANDSHAKES_SUCCESSES_INIT: &str = "snarkos_handshakes_successes_init_total";
pub const HANDSHAKES_SUCCESSES_RESP: &str = "snarkos_handshakes_successes_resp_total";
pub const HANDSHAKES_TIMEOUTS_INIT: &str = "snarkos_handshakes_timeouts_init_total";
pub const HANDSHAKES_TIMEOUTS_RESP: &str = "snarkos_handshakes_timeouts_resp_total";

pub const QUEUES_INBOUND: &str = "snarkos_queues_inbound_total";
pub const QUEUES_OUTBOUND: &str = "snarkos_queues_outbound_total";

pub const MISC_BLOCK_HEIGHT: &str = "snarkos_misc_block_height_total";
pub const MISC_BLOCKS_MINED: &str = "snarkos_misc_blocks_mined_total";
pub const MISC_DUPLICATE_BLOCKS: &str = "snarkos_misc_duplicate_blocks_total";
pub const MISC_DUPLICATE_SYNC_BLOCKS: &str = "snarkos_misc_duplicate_sync_blocks_total";
pub const MISC_RPC_REQUESTS: &str = "snarkos_misc_rpc_requests_total";

pub static NODE_STATS: Stats = Stats::new();

// TODO: make members private and make gathering of stats feature-gated and possibly
// interchangeable with prometheus metrics.
pub struct Stats {
    /// Stats related to messages received by the node.
    inbound: InboundStats,
    /// Stats related to messages sent by the node.
    outbound: OutboundStats,
    /// Stats related to the node's connections.
    connections: ConnectionStats,
    /// Stats related to the node's handshakes.
    handshakes: HandshakeStats,
    /// Stats related to the node's queues.
    queues: QueueStats,
    /// Miscellaneous stats related to the node.
    misc: MiscStats,
}

impl Stats {
    const fn new() -> Self {
        Self {
            inbound: InboundStats::new(),
            outbound: OutboundStats::new(),
            connections: ConnectionStats::new(),
            handshakes: HandshakeStats::new(),
            queues: QueueStats::new(),
            misc: MiscStats::new(),
        }
    }
}

pub struct InboundStats {
    /// The number of successfully processed inbound messages.
    all_successes: Counter,
    /// The number of inbound messages that couldn't be processed.
    all_failures: Counter,

    /// The number of all received `Block` messages.
    blocks: Counter,
    /// The number of all received `GetBlocks` messages.
    getblocks: Counter,
    /// The number of all received `GetMemoryPool` messages.
    getmemorypool: Counter,
    /// The number of all received `GetPeers` messages.
    getpeers: Counter,
    /// The number of all received `GetSync` messages.
    getsync: Counter,
    /// The number of all received `MemoryPool` messages.
    memorypool: Counter,
    /// The number of all received `Peers` messages.
    peers: Counter,
    /// The number of all received `Ping` messages.
    pings: Counter,
    /// The number of all received `Pong` messages.
    pongs: Counter,
    /// The number of all received `Sync` messages.
    syncs: Counter,
    /// The number of all received `SyncBlock` messages.
    syncblocks: Counter,
    /// The number of all received `Transaction` messages.
    transactions: Counter,
    /// The number of all received `Unknown` messages.
    unknown: Counter,
}

impl InboundStats {
    const fn new() -> Self {
        Self {
            all_successes: Counter::new(),
            all_failures: Counter::new(),
            blocks: Counter::new(),
            getblocks: Counter::new(),
            getmemorypool: Counter::new(),
            getpeers: Counter::new(),
            getsync: Counter::new(),
            memorypool: Counter::new(),
            peers: Counter::new(),
            pings: Counter::new(),
            pongs: Counter::new(),
            syncs: Counter::new(),
            syncblocks: Counter::new(),
            transactions: Counter::new(),
            unknown: Counter::new(),
        }
    }
}

pub struct OutboundStats {
    /// The number of messages successfully sent by the node.
    all_successes: Counter,
    /// The number of messages that failed to be sent to peers.
    all_failures: Counter,
}

impl OutboundStats {
    const fn new() -> Self {
        Self {
            all_successes: Counter::new(),
            all_failures: Counter::new(),
        }
    }
}

pub struct ConnectionStats {
    /// The number of all connections the node has accepted.
    all_accepted: Counter,
    /// The number of all connections the node has initiated.
    all_initiated: Counter,
    /// The number of rejected inbound connection requests.
    all_rejected: Counter,
}

impl ConnectionStats {
    const fn new() -> Self {
        Self {
            all_accepted: Counter::new(),
            all_initiated: Counter::new(),
            all_rejected: Counter::new(),
        }
    }
}

pub struct HandshakeStats {
    /// The number of failed handshakes as the initiator.
    failures_init: Counter,
    /// The number of failed handshakes as the responder.
    failures_resp: Counter,
    /// The number of successful handshakes as the initiator.
    successes_init: Counter,
    /// The number of successful handshakes as the responder.
    successes_resp: Counter,
    /// The number of handshake timeouts as the initiator.
    timeouts_init: Counter,
    /// The number of handshake timeouts as the responder.
    timeouts_resp: Counter,
}

impl HandshakeStats {
    const fn new() -> Self {
        Self {
            failures_init: Counter::new(),
            failures_resp: Counter::new(),
            successes_init: Counter::new(),
            successes_resp: Counter::new(),
            timeouts_init: Counter::new(),
            timeouts_resp: Counter::new(),
        }
    }
}

pub struct QueueStats {
    /// The number of messages queued in the common inbound channel.
    inbound: Gauge,
    /// The number of messages queued in the individual outbound channels.
    outbound: Gauge,
}

impl QueueStats {
    const fn new() -> Self {
        Self {
            inbound: Gauge::new(),
            outbound: Gauge::new(),
        }
    }
}

pub struct MiscStats {
    block_height: Gauge,
    /// The number of mined blocks.
    blocks_mined: Counter,
    /// The number of duplicate blocks received.
    duplicate_blocks: Counter,
    /// The number of duplicate sync blocks received.
    duplicate_sync_blocks: Counter,
    /// The number of RPC requests received.
    rpc_requests: Counter,
}

impl MiscStats {
    const fn new() -> Self {
        Self {
            block_height: Gauge::new(),
            blocks_mined: Counter::new(),
            duplicate_blocks: Counter::new(),
            duplicate_sync_blocks: Counter::new(),
            rpc_requests: Counter::new(),
        }
    }
}

impl Recorder for Stats {
    // The following are unused in Stats
    fn register_counter(&self, _key: &Key, _unit: Option<Unit>, _desc: Option<&'static str>) {}

    fn register_gauge(&self, _key: &Key, _unit: Option<Unit>, _desc: Option<&'static str>) {}

    fn register_histogram(&self, _key: &Key, _unit: Option<Unit>, _desc: Option<&'static str>) {}

    fn record_histogram(&self, _key: &Key, _value: f64) {}

    fn increment_counter(&self, key: &Key, value: u64) {
        let metric = match key.name() {
            // inbound
            INBOUND_ALL_SUCCESSES => &self.inbound.all_successes,
            INBOUND_ALL_FAILURES => &self.inbound.all_failures,
            INBOUND_BLOCKS => &self.inbound.blocks,
            INBOUND_GETBLOCKS => &self.inbound.getblocks,
            INBOUND_GETMEMORYPOOL => &self.inbound.getmemorypool,
            INBOUND_GETPEERS => &self.inbound.getpeers,
            INBOUND_GETSYNC => &self.inbound.getsync,
            INBOUND_MEMORYPOOL => &self.inbound.memorypool,
            INBOUND_PEERS => &self.inbound.peers,
            INBOUND_PINGS => &self.inbound.pings,
            INBOUND_PONGS => &self.inbound.pongs,
            INBOUND_SYNCS => &self.inbound.syncs,
            INBOUND_SYNCBLOCKS => &self.inbound.syncblocks,
            INBOUND_TRANSACTIONS => &self.inbound.transactions,
            INBOUND_UNKNOWN => &self.inbound.unknown,
            // outbound
            OUTBOUND_ALL_SUCCESSES => &self.outbound.all_successes,
            OUTBOUND_ALL_FAILURES => &self.outbound.all_failures,
            // connections
            CONNECTIONS_ALL_ACCEPTED => &self.connections.all_accepted,
            CONNECTIONS_ALL_INITIATED => &self.connections.all_initiated,
            CONNECTIONS_ALL_REJECTED => &self.connections.all_rejected,
            // handshakes
            HANDSHAKES_FAILURES_INIT => &self.handshakes.failures_init,
            HANDSHAKES_FAILURES_RESP => &self.handshakes.failures_resp,
            HANDSHAKES_SUCCESSES_INIT => &self.handshakes.successes_init,
            HANDSHAKES_SUCCESSES_RESP => &self.handshakes.successes_resp,
            HANDSHAKES_TIMEOUTS_INIT => &self.handshakes.timeouts_init,
            HANDSHAKES_TIMEOUTS_RESP => &self.handshakes.timeouts_resp,
            // misc
            MISC_BLOCKS_MINED => &self.misc.blocks_mined,
            MISC_DUPLICATE_BLOCKS => &self.misc.duplicate_blocks,
            MISC_DUPLICATE_SYNC_BLOCKS => &self.misc.duplicate_sync_blocks,
            MISC_RPC_REQUESTS => &self.misc.rpc_requests,
            _ => {
                error!("Metrics key {} wasn't assigned an operation and won't work!", key);
                return;
            }
        };
        metric.increment(value);
    }

    fn update_gauge(&self, key: &Key, value: GaugeValue) {
        let metric = match key.name() {
            // queues
            QUEUES_INBOUND => &self.queues.inbound,
            QUEUES_OUTBOUND => &self.queues.outbound,
            // misc
            MISC_BLOCK_HEIGHT => &self.misc.block_height,
            // obtained ad-hoc for the purposes of RPC metrics
            CONNECTIONS_CONNECTING | CONNECTIONS_CONNECTED | CONNECTIONS_DISCONNECTED => {
                todo!("@sadroeck - add me")
            }
            _ => {
                error!("Metrics key {} wasn't assigned an operation and won't work!", key);
                return;
            }
        };
        match value {
            GaugeValue::Increment(val) => metric.increase(val),
            GaugeValue::Decrement(val) => metric.decrease(val),
            GaugeValue::Absolute(val) => metric.set(val),
        }
    }
}

/// Mimics a [`metrics-core`] monotonically increasing [`Counter`] type
pub struct Counter(AtomicU64);

impl Counter {
    const fn new() -> Self {
        Self(AtomicU64::new(0))
    }

    fn increment(&self, val: u64) {
        self.0.fetch_add(val, Ordering::SeqCst);
    }
}

/// Mimics a [`metrics-core`] arbitrarily increasing & decreasing [`Gauge`]
pub struct Gauge(AtomicU64);

impl Gauge {
    const fn new() -> Self {
        Self(AtomicU64::new(0))
    }

    fn set(&self, val: f64) {
        // TODO: @sadroeck - Reinterpret as f64 & do atomic C&S
        self.0.store(val as u64, Ordering::SeqCst);
    }

    fn increase(&self, val: f64) {
        // TODO: @sadroeck - Reinterpret as f64 & do atomic C&S
        self.0.fetch_add(val as u64, Ordering::SeqCst);
    }

    fn decrease(&self, val: f64) {
        // TODO: @sadroeck - Reinterpret as f64 & do atomic C&S
        self.0.fetch_sub(val as u64, Ordering::SeqCst);
    }
}
