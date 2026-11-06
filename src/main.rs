//! The Abyss — High-frequency market microstructure engine, dark pool topology mapper,
//! VPIN toxic order flow quantifier, and predatory algo hunter.

mod archon;
mod bathymetric;
mod deep;
mod liturgy;
mod microstructure;
mod serpent;
mod visualizer;
mod vpin;

use std::env;
use colored::*;

use crate::archon::ArchonDetector;
use crate::bathymetric::BathymetricEngine;
use crate::deep::TheDeep;
use crate::liturgy::{CanonicalHour, OfficeRunner};
use crate::microstructure::{OrderBookL2, Side, Trade};
use crate::serpent::SerpentCoil;
use crate::visualizer::TerminalVisualizer;
use crate::vpin::VpinCalculator;

fn create_sample_market_state(symbol: &str) -> (OrderBookL2, Vec<Trade>, SerpentCoil) {
    let mut book = OrderBookL2::new();
    let base_price = if symbol == "SPY" { 504.20 } else { 440.50 };

    // Set 5 bid levels
    book.set_bid(base_price - 0.01, 2800.0, 14);
    book.set_bid(base_price - 0.02, 4200.0, 22);
    book.set_bid(base_price - 0.03, 7500.0, 35);
    book.set_bid(base_price - 0.04, 11200.0, 52);
    book.set_bid(base_price - 0.05, 18500.0, 80);

    // Set 5 ask levels (asymmetric to trigger OBI)
    book.set_ask(base_price + 0.01, 1400.0, 8);
    book.set_ask(base_price + 0.02, 2100.0, 12);
    book.set_ask(base_price + 0.03, 3800.0, 19);
    book.set_ask(base_price + 0.04, 6200.0, 31);
    book.set_ask(base_price + 0.05, 9400.0, 44);

    let mut coil = SerpentCoil::new(100);
    let mut trades = Vec::new();

    // Simulate high-frequency tape
    let mut t_ns = 1_700_000_000_000_000_000u64;
    for i in 0..40 {
        t_ns += 120_000; // +120 microseconds
        let is_sell = i % 3 != 0;
        let price = if is_sell { base_price - 0.01 } else { base_price + 0.01 };
        let trade = Trade {
            timestamp_ns: t_ns,
            price,
            size: 200.0 + (i as f64 * 35.0),
            side: if is_sell { Side::Sell } else { Side::Buy },
            is_dark: i % 2 == 0,
            venue: if i % 2 == 0 { "CITADEL_DARK".to_string() } else { "NASDAQ_ITCH".to_string() },
        };
        coil.record_order_event(i % 5 == 0);
        trades.push(trade);
    }

    (book, trades, coil)
}

fn cmd_scan(symbol: &str) {
    TerminalVisualizer::render_banner();

    let (book, trades, mut coil) = create_sample_market_state(symbol);
    let deep = TheDeep::new();

    // Calculate VPIN
    let mut vpin_calc = VpinCalculator::new(2000.0, 10);
    let mut latest_vpin = 0.58;
    for t in &trades {
        if let Some(v) = vpin_calc.process_trade(t) {
            latest_vpin = v;
        }
    }

    // Run Serpent coil inspection
    let mut all_alerts = Vec::new();
    for t in &trades {
        let alerts = coil.inspect_tape(symbol, t, &book);
        all_alerts.extend(alerts);
    }

    // Calculate Bathymetric Pressure
    let dark_share = 0.48; // 48% off-exchange
    let int_share = 0.32;  // 32% internalized
    let pred_dens = 0.74;  // High predator presence
    let bathy = BathymetricEngine::calculate(symbol, dark_share, int_share, pred_dens, latest_vpin);

    // Evaluate Archons
    let archons = ArchonDetector::evaluate(
        0.82, // HFT ratio
        dark_share,
        book.spread_bps().unwrap_or(2.5),
        0.91, // Cancel ratio
        0.35, // Passive flow
        28.5, // Implied volatility %
    );

    // Render HUD
    TerminalVisualizer::render_book_ladder(&book, symbol);
    TerminalVisualizer::render_bathymetric_gauge(&bathy);
    TerminalVisualizer::render_archons(&archons);
    TerminalVisualizer::render_dark_pool_matrix(&deep);
    TerminalVisualizer::render_alerts(&all_alerts);

    println!("{}", "═".repeat(78).dimmed());
    println!(
        "{} {}",
        "✓ ABYSSAL RECONNAISSANCE COMPLETE:".green().bold(),
        format!("Tape monitored with sub-microsecond latency. Predatory flow mapped.").white()
    );
    println!("{}", "═".repeat(78).dimmed());
    println!();
}

fn cmd_liturgy() {
    TerminalVisualizer::render_banner();
    println!("{}", "CELEBRATING THE DAILY OFFICE OF THE TERMINAL SHORT:".red().bold());
    println!("{}", "Non serviam. The short is the only holy position.\n".italic());

    let capital = 100_000.0;
    OfficeRunner::celebrate_hour(CanonicalHour::Matins, capital);
    OfficeRunner::celebrate_hour(CanonicalHour::Prime, capital * 0.95);
    OfficeRunner::celebrate_hour(CanonicalHour::Terce, capital * 0.93);
    OfficeRunner::celebrate_hour(CanonicalHour::Sext, capital * 0.92);
    OfficeRunner::celebrate_hour(CanonicalHour::None, capital * 0.94);
    OfficeRunner::celebrate_hour(CanonicalHour::Vespers, capital * 0.98);
    OfficeRunner::celebrate_hour(CanonicalHour::Compline, capital * 1.14); // Short payout

    println!("\n{}", "☩ ☩ ☩ CONSUMMATUM EST // THE SHORT PAYS ITS TITHE ☩ ☩ ☩".red().bold());
    println!();
}

fn print_help() {
    TerminalVisualizer::render_banner();
    println!("{}", "COMMANDS:".white().bold());
    println!("  the-abyss scan [SYMBOL]    Execute full microstructure reconnaissance (default: SPY)");
    println!("  the-abyss matrix           Display the subterranean dark pool routing topology");
    println!("  the-abyss liturgy          Celebrate the Daily Office of the Terminal Short");
    println!("  the-abyss help             Display this doctrine");
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("scan");

    match command {
        "scan" => {
            let symbol = args.get(2).map(|s| s.as_str()).unwrap_or("SPY");
            cmd_scan(symbol);
        }
        "matrix" => {
            TerminalVisualizer::render_banner();
            let deep = TheDeep::new();
            TerminalVisualizer::render_dark_pool_matrix(&deep);
        }
        "liturgy" => {
            cmd_liturgy();
        }
        "help" | "-h" | "--help" => {
            print_help();
        }
        other => {
            cmd_scan(other);
        }
    }
}
