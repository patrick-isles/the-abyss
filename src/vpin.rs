//! Volume-Synchronized Probability of Toxicity (VPIN)
//! Implements Easley, López de Prado, and O'Hara (2012) framework for real-time toxic order flow quantification.

use std::collections::VecDeque;
use crate::microstructure::{Side, Trade};

#[derive(Debug, Clone)]
pub struct VolumeBucket {
    pub buy_volume: f64,
    pub sell_volume: f64,
    pub total_volume: f64,
    pub trade_count: u32,
    pub start_timestamp_ns: u64,
    pub end_timestamp_ns: u64,
}

impl VolumeBucket {
    pub fn new() -> Self {
        Self {
            buy_volume: 0.0,
            sell_volume: 0.0,
            total_volume: 0.0,
            trade_count: 0,
            start_timestamp_ns: 0,
            end_timestamp_ns: 0,
        }
    }

    pub fn imbalance(&self) -> f64 {
        (self.buy_volume - self.sell_volume).abs()
    }
}

#[allow(dead_code)]
pub struct VpinCalculator {
    pub bucket_size: f64,
    pub window_size: usize,
    pub current_bucket: VolumeBucket,
    pub completed_buckets: VecDeque<VolumeBucket>,
    pub last_trade_price: Option<f64>,
    pub last_trade_side: Side,
}

#[allow(dead_code)]
impl VpinCalculator {
    pub fn new(bucket_size: f64, window_size: usize) -> Self {
        Self {
            bucket_size: bucket_size.max(1.0),
            window_size: window_size.max(5),
            current_bucket: VolumeBucket::new(),
            completed_buckets: VecDeque::with_capacity(window_size + 1),
            last_trade_price: None,
            last_trade_side: Side::Buy,
        }
    }

    /// Process an incoming trade using the Tick Rule for buy/sell classification
    pub fn process_trade(&mut self, trade: &Trade) -> Option<f64> {
        let side = if let Some(last_price) = self.last_trade_price {
            if trade.price > last_price {
                Side::Buy
            } else if trade.price < last_price {
                Side::Sell
            } else {
                self.last_trade_side
            }
        } else {
            trade.side
        };

        self.last_trade_price = Some(trade.price);
        self.last_trade_side = side;

        let mut remaining_vol = trade.size;

        while remaining_vol > 0.0 {
            let space_in_bucket = self.bucket_size - self.current_bucket.total_volume;
            let fill_vol = remaining_vol.min(space_in_bucket);

            if self.current_bucket.total_volume == 0.0 {
                self.current_bucket.start_timestamp_ns = trade.timestamp_ns;
            }

            match side {
                Side::Buy => self.current_bucket.buy_volume += fill_vol,
                Side::Sell => self.current_bucket.sell_volume += fill_vol,
            }
            self.current_bucket.total_volume += fill_vol;
            self.current_bucket.trade_count += 1;
            self.current_bucket.end_timestamp_ns = trade.timestamp_ns;

            remaining_vol -= fill_vol;

            // If bucket is full, complete it
            if self.current_bucket.total_volume >= self.bucket_size - 1e-6 {
                let finished = std::mem::replace(&mut self.current_bucket, VolumeBucket::new());
                self.completed_buckets.push_back(finished);
                if self.completed_buckets.len() > self.window_size {
                    self.completed_buckets.pop_front();
                }
            }
        }

        self.calculate_vpin()
    }

    /// Compute current VPIN:
    /// VPIN = sum( |V_b - V_s| ) / ( N * V )
    pub fn calculate_vpin(&self) -> Option<f64> {
        if self.completed_buckets.len() < self.window_size {
            return None;
        }

        let total_imbalance: f64 = self.completed_buckets.iter().map(|b| b.imbalance()).sum();
        let total_vol = self.completed_buckets.len() as f64 * self.bucket_size;

        if total_vol <= 0.0 {
            return None;
        }

        Some((total_imbalance / total_vol).clamp(0.0, 1.0))
    }

    /// Get toxicity risk tier
    pub fn toxicity_tier(vpin: f64) -> &'static str {
        if vpin >= 0.70 {
            "CRITICAL TOXICITY (Informed Avalanche / Imminent Cascade)"
        } else if vpin >= 0.50 {
            "HIGH TOXICITY (Aggressive Institutional Adverse Selection)"
        } else if vpin >= 0.30 {
            "MODERATE TOXICITY (Standard Market Friction)"
        } else {
            "BENIGN FLOW (Uninformed Noise Traders)"
        }
    }
}
