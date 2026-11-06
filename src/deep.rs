//! The Deep — Dark Pool Topology & Subterranean Liquidity Migration.
//! Maps the unlit routing rivers between Internalizers, ATS Dark Pools, and Lit Exchanges.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Internalizer,
    Wholesaler,
    DarkPoolATS,
    LitExchange,
    ConsolidatedFeed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbyssalNode {
    pub name: &'static str,
    pub node_type: NodeType,
    pub market_share_pct: f64,
    pub adverse_selection_toxicity: f64, // [0.0, 1.0]
    pub direct_latency_us: u32,          // Microseconds
    pub connects_to: Vec<&'static str>,
    pub is_predatory: bool,
}

#[allow(dead_code)]
pub struct TheDeep {
    pub nodes: HashMap<&'static str, AbyssalNode>,
    pub sip_latency_lag_us: u32, // Typical SIP latency lag vs direct feeds (e.g. 3500us)
}

#[allow(dead_code)]
impl TheDeep {
    pub fn new() -> Self {
        let mut nodes = HashMap::new();

        // 1. Predatory Internalizers & Wholesalers
        nodes.insert(
            "CITADEL",
            AbyssalNode {
                name: "Citadel Securities",
                node_type: NodeType::Internalizer,
                market_share_pct: 22.0,
                adverse_selection_toxicity: 0.78,
                direct_latency_us: 180,
                connects_to: vec!["VIRTU", "G1X", "NASDAQ", "NYSE", "UBS_ATS"],
                is_predatory: true,
            },
        );

        nodes.insert(
            "VIRTU",
            AbyssalNode {
                name: "Virtu Americas",
                node_type: NodeType::Internalizer,
                market_share_pct: 16.5,
                adverse_selection_toxicity: 0.72,
                direct_latency_us: 210,
                connects_to: vec!["CITADEL", "NASDAQ", "NYSE", "SIGMA_X"],
                is_predatory: true,
            },
        );

        nodes.insert(
            "JANE_STREET",
            AbyssalNode {
                name: "Jane Street Capital",
                node_type: NodeType::Wholesaler,
                market_share_pct: 12.0,
                adverse_selection_toxicity: 0.85,
                direct_latency_us: 140,
                connects_to: vec!["CITADEL", "NASDAQ", "NYSE", "BATS"],
                is_predatory: true,
            },
        );

        nodes.insert(
            "TWO_SIGMA",
            AbyssalNode {
                name: "Two Sigma Wholesale",
                node_type: NodeType::Wholesaler,
                market_share_pct: 7.5,
                adverse_selection_toxicity: 0.65,
                direct_latency_us: 240,
                connects_to: vec!["CITADEL", "VIRTU", "NASDAQ", "NYSE"],
                is_predatory: true,
            },
        );

        // 2. ATS Dark Pools
        nodes.insert(
            "UBS_ATS",
            AbyssalNode {
                name: "UBS ATS",
                node_type: NodeType::DarkPoolATS,
                market_share_pct: 4.8,
                adverse_selection_toxicity: 0.45,
                direct_latency_us: 950,
                connects_to: vec!["CITADEL", "VIRTU", "NYSE", "NASDAQ"],
                is_predatory: false,
            },
        );

        nodes.insert(
            "SIGMA_X",
            AbyssalNode {
                name: "Sigma X (Goldman Sachs)",
                node_type: NodeType::DarkPoolATS,
                market_share_pct: 3.9,
                adverse_selection_toxicity: 0.52,
                direct_latency_us: 1100,
                connects_to: vec!["CITADEL", "VIRTU", "NYSE"],
                is_predatory: false,
            },
        );

        nodes.insert(
            "LEVEL_ATS",
            AbyssalNode {
                name: "Level ATS",
                node_type: NodeType::DarkPoolATS,
                market_share_pct: 3.2,
                adverse_selection_toxicity: 0.38,
                direct_latency_us: 880,
                connects_to: vec!["NASDAQ", "NYSE", "BATS"],
                is_predatory: false,
            },
        );

        // 3. Lit Exchanges (Direct Microwave & Fiber)
        nodes.insert(
            "NASDAQ",
            AbyssalNode {
                name: "NASDAQ (Direct ITCH)",
                node_type: NodeType::LitExchange,
                market_share_pct: 18.0,
                adverse_selection_toxicity: 0.25,
                direct_latency_us: 420,
                connects_to: vec!["NYSE", "BATS", "SIP"],
                is_predatory: false,
            },
        );

        nodes.insert(
            "NYSE",
            AbyssalNode {
                name: "NYSE (Pillar Direct)",
                node_type: NodeType::LitExchange,
                market_share_pct: 19.5,
                adverse_selection_toxicity: 0.28,
                direct_latency_us: 480,
                connects_to: vec!["NASDAQ", "BATS", "SIP"],
                is_predatory: false,
            },
        );

        nodes.insert(
            "BATS",
            AbyssalNode {
                name: "Cboe BATS (Direct PITCH)",
                node_type: NodeType::LitExchange,
                market_share_pct: 11.0,
                adverse_selection_toxicity: 0.22,
                direct_latency_us: 520,
                connects_to: vec!["NASDAQ", "NYSE", "SIP"],
                is_predatory: false,
            },
        );

        // 4. Consolidated Public SIP (Consolidated Tape Association)
        nodes.insert(
            "SIP",
            AbyssalNode {
                name: "Consolidated Tape (SIP Lag)",
                node_type: NodeType::ConsolidatedFeed,
                market_share_pct: 100.0,
                adverse_selection_toxicity: 0.95,
                direct_latency_us: 3800, // 3.8ms public latency buffer
                connects_to: vec![],
                is_predatory: false,
            },
        );

        Self {
            nodes,
            sip_latency_lag_us: 3300, // 3.3ms latency advantage for direct feeds vs SIP
        }
    }

    /// Calculate latency arbitrage vulnerability window (in microseconds)
    pub fn latency_arbitrage_window_us(&self, node_key: &str) -> Option<u32> {
        let node = self.nodes.get(node_key)?;
        let sip = self.nodes.get("SIP")?;
        Some(sip.direct_latency_us.saturating_sub(node.direct_latency_us))
    }

    /// Calculate total off-exchange dark pool volume percentage
    pub fn total_dark_volume_pct(&self) -> f64 {
        self.nodes
            .values()
            .filter(|n| n.node_type == NodeType::Internalizer || n.node_type == NodeType::DarkPoolATS || n.node_type == NodeType::Wholesaler)
            .map(|n| n.market_share_pct)
            .sum()
    }
}
