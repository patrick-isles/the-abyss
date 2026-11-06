//! Serpent Coil — Nanosecond Algorithmic Predation Detector.
//! Watches the tape. Names the serpent. Breaks its fangs.

use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use crate::microstructure::{OrderBookL2, Side, Trade};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SerpentType {
    MomentumIgnition,
    StopHunt,
    LayeringSpoof,
    QuoteStuffing,
    WashTrading,
}

impl SerpentType {
    pub fn name(&self) -> &'static str {
        match self {
            SerpentType::MomentumIgnition => "MOMENTUM_IGNITION (Artificial Impulse)",
            SerpentType::StopHunt => "STOP_HUNT (Liquidity Vacuum Harvest)",
            SerpentType::LayeringSpoof => "LAYERING_SPOOF (Phantom Book Deception)",
            SerpentType::QuoteStuffing => "QUOTE_STUFFING (Bandwidth Denial of Service)",
            SerpentType::WashTrading => "WASH_TRADING (Tape Painting)",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerpentAlert {
    pub timestamp_ns: u64,
    pub serpent_type: SerpentType,
    pub symbol: String,
    pub confidence: f64, // [0.0, 1.0]
    pub magnitude: f64,
    pub toxicity: f64,   // Threat level to our position [0.0, 1.0]
    pub diagnosis: String,
}

pub struct SerpentCoil {
    pub trade_window: VecDeque<Trade>,
    pub max_window_size: usize,
    pub cancel_count: u32,
    pub order_count: u32,
    pub recent_alerts: Vec<SerpentAlert>,
}

impl SerpentCoil {
    pub fn new(max_window_size: usize) -> Self {
        Self {
            trade_window: VecDeque::with_capacity(max_window_size + 1),
            max_window_size,
            cancel_count: 0,
            order_count: 0,
            recent_alerts: Vec::new(),
        }
    }

    pub fn record_order_event(&mut self, is_cancel: bool) {
        self.order_count += 1;
        if is_cancel {
            self.cancel_count += 1;
        }
    }

    pub fn inspect_tape(&mut self, symbol: &str, trade: &Trade, book: &OrderBookL2) -> Vec<SerpentAlert> {
        let mut alerts = Vec::new();
        self.trade_window.push_back(trade.clone());
        if self.trade_window.len() > self.max_window_size {
            self.trade_window.pop_front();
        }

        if self.trade_window.len() < 10 {
            return alerts;
        }

        // 1. Detect Momentum Ignition: rapid volume spike + sharp price run with book depletion
        if let Some(alert) = self.check_momentum_ignition(symbol, book) {
            alerts.push(alert);
        }

        // 2. Detect Stop Hunting: price sweep through local high/low followed by sharp reversal
        if let Some(alert) = self.check_stop_hunt(symbol) {
            alerts.push(alert);
        }

        // 3. Detect Layering / Spoofing: high book imbalance + high cancel-to-fill ratio
        if let Some(alert) = self.check_layering_spoof(symbol, book) {
            alerts.push(alert);
        }

        // 4. Detect Quote Stuffing: burst message rate
        if let Some(alert) = self.check_quote_stuffing(symbol) {
            alerts.push(alert);
        }

        for a in &alerts {
            self.recent_alerts.push(a.clone());
            if self.recent_alerts.len() > 50 {
                self.recent_alerts.remove(0);
            }
        }

        alerts
    }

    fn check_momentum_ignition(&self, symbol: &str, book: &OrderBookL2) -> Option<SerpentAlert> {
        let trades: Vec<&Trade> = self.trade_window.iter().rev().take(8).collect();
        if trades.len() < 8 {
            return None;
        }

        let first = trades.last()?;
        let last = trades.first()?;
        let time_delta_ms = (last.timestamp_ns.saturating_sub(first.timestamp_ns)) as f64 / 1_000_000.0;

        if time_delta_ms > 250.0 || time_delta_ms <= 0.0 {
            return None;
        }

        let price_delta = (last.price - first.price).abs();
        let mid = book.midpoint().unwrap_or(last.price);
        let return_bps = (price_delta / mid) * 10000.0;

        // If price moves > 12 bps in < 250ms with one-sided aggressive prints
        if return_bps > 12.0 {
            let buy_count = trades.iter().filter(|t| t.side == Side::Buy).count();
            let sell_count = trades.len() - buy_count;
            let directional_ratio = (buy_count as f64 - sell_count as f64).abs() / trades.len() as f64;

            if directional_ratio > 0.75 {
                return Some(SerpentAlert {
                    timestamp_ns: last.timestamp_ns,
                    serpent_type: SerpentType::MomentumIgnition,
                    symbol: symbol.to_string(),
                    confidence: 0.88,
                    magnitude: return_bps,
                    toxicity: 0.82,
                    diagnosis: format!(
                        "Aggressive sweep moved tape {:+.1} bps in {:.1}ms without depth support.",
                        return_bps, time_delta_ms
                    ),
                });
            }
        }
        None
    }

    fn check_stop_hunt(&self, symbol: &str) -> Option<SerpentAlert> {
        let trades: Vec<&Trade> = self.trade_window.iter().rev().take(15).collect();
        if trades.len() < 15 {
            return None;
        }

        let prices: Vec<f64> = trades.iter().map(|t| t.price).collect();
        let min_p = prices.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_p = prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let latest = trades[0].price;
        let oldest = trades[trades.len() - 1].price;

        // V-shape reversal: price pushed to extreme then snapped back across the origin
        let amplitude = max_p - min_p;
        let _mid = (max_p + min_p) / 2.0;
        let net_change = (latest - oldest).abs();

        if amplitude > 0.40 && net_change < (amplitude * 0.25) {
            return Some(SerpentAlert {
                timestamp_ns: trades[0].timestamp_ns,
                serpent_type: SerpentType::StopHunt,
                symbol: symbol.to_string(),
                confidence: 0.84,
                magnitude: amplitude,
                toxicity: 0.75,
                diagnosis: format!(
                    "V-reversal detected: price pierced ${:.2} threshold, absorbed stops, and snapped back.",
                    if latest > oldest { min_p } else { max_p }
                ),
            });
        }
        None
    }

    fn check_layering_spoof(&self, symbol: &str, book: &OrderBookL2) -> Option<SerpentAlert> {
        let obi = book.order_book_imbalance()?;
        // If order book imbalance is extreme (> 0.80 or < -0.80) and cancellation ratio exceeds 90%
        if obi.abs() > 0.75 && self.order_count > 50 {
            let cancel_ratio = self.cancel_count as f64 / self.order_count as f64;
            if cancel_ratio > 0.88 {
                return Some(SerpentAlert {
                    timestamp_ns: self.trade_window.back().map(|t| t.timestamp_ns).unwrap_or(0),
                    serpent_type: SerpentType::LayeringSpoof,
                    symbol: symbol.to_string(),
                    confidence: 0.91,
                    magnitude: obi.abs(),
                    toxicity: 0.88,
                    diagnosis: format!(
                        "Phantom book skew: OBI={:+.2} with {:.1}% cancel-to-order ratio.",
                        obi, cancel_ratio * 100.0
                    ),
                });
            }
        }
        None
    }

    fn check_quote_stuffing(&self, symbol: &str) -> Option<SerpentAlert> {
        if self.order_count > 250 {
            let cancel_ratio = self.cancel_count as f64 / self.order_count as f64;
            if cancel_ratio > 0.96 {
                return Some(SerpentAlert {
                    timestamp_ns: self.trade_window.back().map(|t| t.timestamp_ns).unwrap_or(0),
                    serpent_type: SerpentType::QuoteStuffing,
                    symbol: symbol.to_string(),
                    confidence: 0.94,
                    magnitude: self.order_count as f64,
                    toxicity: 0.92,
                    diagnosis: format!(
                        "Quote stuffing burst: {} msgs with {:.1}% cancellation rate.",
                        self.order_count, cancel_ratio * 100.0
                    ),
                });
            }
        }
        None
    }
}
