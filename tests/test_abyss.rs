#![allow(dead_code)]

#[path = "../src/microstructure.rs"]
mod microstructure;
#[path = "../src/vpin.rs"]
mod vpin;
#[path = "../src/deep.rs"]
mod deep;
#[path = "../src/bathymetric.rs"]
mod bathymetric;
#[path = "../src/archon.rs"]
mod archon;
#[path = "../src/serpent.rs"]
mod serpent;

use microstructure::{OrderBookL2, Side, Trade};
use vpin::VpinCalculator;
use deep::TheDeep;
use bathymetric::BathymetricEngine;
use archon::ArchonDetector;
use serpent::SerpentCoil;

#[test]
fn test_order_book_microstructure() {
    let mut book = OrderBookL2::new();
    book.set_bid(100.0, 500.0, 5);
    book.set_ask(100.10, 500.0, 5);

    assert_eq!(book.midpoint(), Some(100.05));
    assert_eq!(book.order_book_imbalance(), Some(0.0));
    assert_eq!(book.micro_price(), Some(100.05));

    // Skew bid volume to test OBI
    book.set_bid(100.0, 1500.0, 10);
    assert_eq!(book.order_book_imbalance(), Some(0.5));
    assert!(book.micro_price().unwrap() > 100.05); // Micro-price leans toward ask
}

#[test]
fn test_vpin_calculation() {
    let mut vpin = VpinCalculator::new(1000.0, 5);

    // Feed balanced trades
    for i in 0..20 {
        let trade = Trade {
            timestamp_ns: 1000 * i,
            price: 100.0 + (i as f64 * 0.01),
            size: 500.0,
            side: if i % 2 == 0 { Side::Buy } else { Side::Sell },
            is_dark: false,
            venue: "LIT".to_string(),
        };
        vpin.process_trade(&trade);
    }

    let val = vpin.calculate_vpin();
    assert!(val.is_some());
    let v = val.unwrap();
    assert!(v >= 0.0 && v <= 1.0);
}

#[test]
fn test_dark_pool_topology_and_sip_lag() {
    let deep = TheDeep::new();
    assert!(deep.nodes.contains_key("CITADEL"));
    assert!(deep.nodes.contains_key("JANE_STREET"));
    assert!(deep.nodes.contains_key("SIP"));

    // Verify SIP latency arbitrage window exists
    let window = deep.latency_arbitrage_window_us("CITADEL").unwrap();
    assert!(window > 3000); // More than 3ms advantage over SIP

    let dark_pct = deep.total_dark_volume_pct();
    assert!(dark_pct > 50.0); // Over half of trading is off-exchange
}

#[test]
fn test_bathymetric_pressure_scaling() {
    // Surface waters (low dark volume, low toxicity)
    let surface = BathymetricEngine::calculate("SPY", 0.10, 0.05, 0.10, 0.15);
    assert!(surface.bathymetric_pressure < 0.50);
    assert!(surface.kelly_size_multiplier > 0.50);

    // Crush depth (high dark volume, critical VPIN toxicity, predator density)
    let abyss = BathymetricEngine::calculate("SPY", 0.85, 0.80, 0.95, 0.90);
    assert!(abyss.bathymetric_pressure > 0.85);
    assert!(abyss.kelly_size_multiplier <= 0.15);
}

#[test]
fn test_archon_detection_matrix() {
    let archons = ArchonDetector::evaluate(0.90, 0.60, 4.0, 0.95, 0.20, 35.0);
    assert_eq!(archons.len(), 6);
    // Highest strength should lead the vector
    assert!(archons[0].strength >= archons[1].strength);
}

#[test]
fn test_serpent_coil_order_event_tracking() {
    let mut coil = SerpentCoil::new(50);
    for i in 0..100 {
        coil.record_order_event(i % 2 == 0);
    }
    assert_eq!(coil.order_count, 100);
    assert_eq!(coil.cancel_count, 50);
}
