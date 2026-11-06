//! High-frequency market microstructure engine: L2 Order Book, OBI, Micro-price, and Kyle's Lambda.

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub timestamp_ns: u64,
    pub price: f64,
    pub size: f64,
    pub side: Side,
    pub is_dark: bool,
    pub venue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookLevel {
    pub price: f64,
    pub size: f64,
    pub order_count: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OrderBookL2 {
    pub bids: BTreeMap<u64, BookLevel>, // key: price * 10000 as int to avoid float key issues
    pub asks: BTreeMap<u64, BookLevel>,
    pub last_update_ns: u64,
}

#[allow(dead_code)]
impl OrderBookL2 {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_update_ns: 0,
        }
    }

    fn price_to_key(price: f64) -> u64 {
        (price * 10000.0).round() as u64
    }

    pub fn set_bid(&mut self, price: f64, size: f64, count: u32) {
        let key = Self::price_to_key(price);
        if size <= 0.0 {
            self.bids.remove(&key);
        } else {
            self.bids.insert(key, BookLevel { price, size, order_count: count });
        }
    }

    pub fn set_ask(&mut self, price: f64, size: f64, count: u32) {
        let key = Self::price_to_key(price);
        if size <= 0.0 {
            self.asks.remove(&key);
        } else {
            self.asks.insert(key, BookLevel { price, size, order_count: count });
        }
    }

    pub fn best_bid(&self) -> Option<&BookLevel> {
        self.bids.values().next_back()
    }

    pub fn best_ask(&self) -> Option<&BookLevel> {
        self.asks.values().next()
    }

    pub fn midpoint(&self) -> Option<f64> {
        let bid = self.best_bid()?.price;
        let ask = self.best_ask()?.price;
        Some((bid + ask) / 2.0)
    }

    pub fn spread_bps(&self) -> Option<f64> {
        let bid = self.best_bid()?.price;
        let ask = self.best_ask()?.price;
        let mid = (bid + ask) / 2.0;
        if mid <= 0.0 {
            return None;
        }
        Some(10000.0 * (ask - bid) / mid)
    }

    /// Volume-weighted micro-price:
    /// P_micro = (V_bid * P_ask + V_ask * P_bid) / (V_bid + V_ask)
    pub fn micro_price(&self) -> Option<f64> {
        let bid = self.best_bid()?;
        let ask = self.best_ask()?;
        let total_vol = bid.size + ask.size;
        if total_vol <= 0.0 {
            return Some((bid.price + ask.price) / 2.0);
        }
        Some((bid.size * ask.price + ask.size * bid.price) / total_vol)
    }

    /// Order Book Imbalance (OBI) at top of book:
    /// OBI = (V_bid - V_ask) / (V_bid + V_ask) in [-1.0, 1.0]
    pub fn order_book_imbalance(&self) -> Option<f64> {
        let bid = self.best_bid()?;
        let ask = self.best_ask()?;
        let total_vol = bid.size + ask.size;
        if total_vol <= 0.0 {
            return Some(0.0);
        }
        Some((bid.size - ask.size) / total_vol)
    }

    /// Cumulative depth up to N levels
    pub fn depth_levels(&self, n: usize) -> (Vec<BookLevel>, Vec<BookLevel>) {
        let bids: Vec<BookLevel> = self.bids.values().rev().take(n).cloned().collect();
        let asks: Vec<BookLevel> = self.asks.values().take(n).cloned().collect();
        (bids, asks)
    }
}
