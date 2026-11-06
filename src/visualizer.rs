//! Terminal HUD & Microstructure Visualizer.

use colored::*;
use crate::archon::ArchonProfile;
use crate::bathymetric::BathymetricCalculation;
use crate::deep::TheDeep;
use crate::microstructure::OrderBookL2;
use crate::serpent::SerpentAlert;

pub struct TerminalVisualizer;

impl TerminalVisualizer {
    pub fn render_banner() {
        let banner = r#"
 ╔═══════════════════════════════════════════════════════════════════════════════╗
 ║                                                                               ║
 ║                   T H E   A B Y S S   //   L E V I A T H A N                  ║
 ║                                                                               ║
 ║     Sub-Microsecond Dark Pool Topology & Order Flow Toxicity Engine           ║
 ║     "Beneath the lit exchange lies the abyss. There, the true price hides."   ║
 ║                                                                               ║
 ╚═══════════════════════════════════════════════════════════════════════════════╝
"#;
        println!("{}", banner.red().bold());
    }

    pub fn render_book_ladder(book: &OrderBookL2, symbol: &str) {
        println!("{}", format!("  ┌─ L2 ORDER BOOK LADDER: {} ─────────────────────────────────┐", symbol).dimmed());
        let (bids, asks) = book.depth_levels(5);

        let mid = book.midpoint().unwrap_or(0.0);
        let micro = book.micro_price().unwrap_or(mid);
        let spread = book.spread_bps().unwrap_or(0.0);
        let obi = book.order_book_imbalance().unwrap_or(0.0);

        // Print Asks (reversed so highest ask is at top)
        for ask in asks.iter().rev() {
            let bar_len = ((ask.size / 500.0).clamp(1.0, 24.0)) as usize;
            let bar = "█".repeat(bar_len);
            println!(
                "  │  ASK {:>8.2}  │ {:>6.0} shares  │ {:<24} │",
                ask.price,
                ask.size,
                bar.red()
            );
        }

        println!("  ├─────────────────────── SPREAD: {:>5.1} bps ────────────────────────┤", spread);
        println!(
            "  │  MID: ${:>8.2}  │ MICRO: ${:>8.2}  │ OBI: {:>+5.2}            │",
            mid, micro, obi
        );
        println!("  ├────────────────────────────────────────────────────────────────────┤");

        // Print Bids
        for bid in bids.iter() {
            let bar_len = ((bid.size / 500.0).clamp(1.0, 24.0)) as usize;
            let bar = "█".repeat(bar_len);
            println!(
                "  │  BID {:>8.2}  │ {:>6.0} shares  │ {:<24} │",
                bid.price,
                bid.size,
                bar.green()
            );
        }
        println!("  └────────────────────────────────────────────────────────────────────┘\n");
    }

    pub fn render_bathymetric_gauge(calc: &BathymetricCalculation) {
        println!("{}", "  ┌─ BATHYMETRIC PRESSURE & DEPTH SIZING ──────────────────────────────┐".dimmed());
        let p = calc.bathymetric_pressure;
        let filled = (p * 36.0).round() as usize;
        let empty = 36usize.saturating_sub(filled);

        let bar_color = if p > 0.75 {
            "█".repeat(filled).red().bold()
        } else if p > 0.45 {
            "█".repeat(filled).yellow().bold()
        } else {
            "█".repeat(filled).cyan()
        };

        println!("  │ Pressure: [{}{}] {:>5.1}%               │", bar_color, "░".repeat(empty).dimmed(), p * 100.0);
        println!("  │ Total Abyssal Depth: {:>5.1}%  │ Dark Pool Share: {:>5.1}%               │", calc.total_abyssal_depth * 100.0, calc.dark_pool_depth * 100.0);
        println!("  │ VPIN Flow Toxicity:  {:>5.1}%  │ Predator Density: {:>4.2}                │", calc.flow_toxicity_vpin * 100.0, calc.predator_density);
        println!("  │ Kelly Multiplier:    {:>5.2}x  │ Sizing Strategy: {:<21} │", calc.kelly_size_multiplier, if p > 0.75 { "CRUSH DEFENSE".red().bold() } else { "NORMAL KELLY".green() });
        println!("  │ Action: {:<58} │", calc.recommended_action.yellow().italic());
        println!("  └────────────────────────────────────────────────────────────────────┘\n");
    }

    pub fn render_dark_pool_matrix(deep: &TheDeep) {
        println!("{}", "  ┌─ SUBTERRANEAN LIQUIDITY ROUTING MATRIX ────────────────────────────┐".dimmed());
        for (key, node) in &deep.nodes {
            let pred_badge = if node.is_predatory { "[PREDATOR]".red().bold() } else { "[PASSIVE]".green() };
            let lat_window = deep.latency_arbitrage_window_us(key).unwrap_or(0);
            println!(
                "  │ {:<12} │ {:<14} │ Mkt: {:>4.1}% │ Tox: {:>4.2} │ Lat: {:>4}µs ({:>4}µs SIP lag) │ {}",
                key.white().bold(),
                format!("{:?}", node.node_type).cyan(),
                node.market_share_pct,
                node.adverse_selection_toxicity,
                node.direct_latency_us,
                lat_window,
                pred_badge
            );
        }
        println!("  └────────────────────────────────────────────────────────────────────┘\n");
    }

    pub fn render_archons(archons: &[ArchonProfile]) {
        println!("{}", "  ┌─ THE SIX MARKET ARCHONS (GOVERNING SPHERES) ───────────────────────┐".dimmed());
        for a in archons {
            let filled = (a.strength * 18.0).round() as usize;
            let empty = 18usize.saturating_sub(filled);
            let bar = format!("{}{}", "■".repeat(filled).yellow(), "·".repeat(empty).dimmed());

            println!(
                "  │ {:<16} │ [{}] {:>5.1}% │ {:<30} │",
                a.name.white().bold(),
                bar,
                a.strength * 100.0,
                a.title.cyan()
            );
        }
        if let Some(dom) = archons.first() {
            println!("  ├────────────────────────────────────────────────────────────────────┤");
            println!("  │ REIGNING ARCHON: {} ({})", dom.name.red().bold(), dom.title.yellow());
            println!("  │ Countermeasure:  {}", dom.countermeasure.italic());
        }
        println!("  └────────────────────────────────────────────────────────────────────┘\n");
    }

    pub fn render_alerts(alerts: &[SerpentAlert]) {
        if alerts.is_empty() {
            return;
        }
        println!("{}", "  ┌─ SERPENT COIL // PREDATORY ALGO ALERTS ────────────────────────────┐".red().bold());
        for a in alerts.iter().rev().take(4) {
            println!(
                "  │ ⚡ {} [{}] Conf: {:.0}% | Tox: {:.0}%",
                a.serpent_type.name().yellow().bold(),
                a.symbol.white().bold(),
                a.confidence * 100.0,
                a.toxicity * 100.0
            );
            println!("  │    ↳ {}", a.diagnosis.dimmed());
        }
        println!("  └────────────────────────────────────────────────────────────────────┘\n");
    }
}
