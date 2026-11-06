# The Abyss // Leviathan 🌑⚡

> **Sub-Microsecond Market Microstructure Engine, Dark Pool Topology Mapper, VPIN Toxic Flow Quantifier, and Predatory Algo Hunter.**  
> *"Beneath the lit exchange lies the abyss. There, the true price hides."*

[![Language: Rust](https://img.shields.io/badge/Language-Rust%202021-orange.svg)](https://www.rust-lang.org/)
[![Microstructure: L2/L3 Book](https://img.shields.io/badge/Microstructure-L2_Order_Book-blue.svg)]()
[![Flow Toxicity: VPIN](https://img.shields.io/badge/Toxicity-VPIN_Quantifier-red.svg)]()
[![Architecture: Sub--Microsecond](https://img.shields.io/badge/Performance-Zero--Allocation_Cores-green.svg)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-purple.svg)](LICENSE)

```
                       † SUBTERRANEAN LIQUIDITY ROUTING †
                                [Retail Broker]
                                       │ (PFOF)
                                       ▼
                  ┌──────────────────────────────────────────┐
                  │    Wholesaler / Internalizer Tier        │
                  │   (Citadel 22% / Virtu 16.5% / Jane St)  │
                  └──────────────┬───────────────────────────┘
                                 │
                   ┌─────────────┴──────────────┐
                   │ (Unfilled Residue)         │ (Information Leakage)
                   ▼                            ▼
      ┌─────────────────────────┐  ┌─────────────────────────┐
      │     ATS Dark Pools      │  │  Direct Microwave Feed  │
      │  (UBS / Sigma X / Level)│  │   (140µs - 480µs lag)   │
      └────────────┬────────────┘  └────────────┬────────────┘
                   │                            │
                   └─────────────┬──────────────┘
                                 │
                                 ▼
                  ┌─────────────────────────────┐
                  │  Lit Exchanges (NYSE/NASDAQ)│
                  │  Consolidated Public SIP    │
                  │    (3,800µs Latency Buffer) │
                  └─────────────────────────────┘
```

---

## I. The Subterranean Architecture

The public order book displayed on retail screens is an illusion. Over **50% of all US equity volume** never touches a lit exchange. It executes in the abyssal zone: internalizer dark books, broker-dealer crossing networks, and alternative trading systems (ATS).

While retail orders crawl through the delayed **Consolidated SIP (Securities Information Processor)** with a 3.5ms latency buffer, predatory high-frequency algorithms trade via direct microwave feeds ($140\mu\text{s}$ – $480\mu\text{s}$), extracting spread toll, hunting stop-loss clusters, and injecting toxic adverse selection.

`the-abyss` is an uncompromising, bare-metal Rust engine that unmasks this subterranean machinery:

1. **Sub-Microsecond L2 Book Reconstruction:** Computes real-time midpoint, micro-price, bid-ask spread in basis points, and top-of-book **Order Book Imbalance (OBI)**:
   $$\text{OBI} = \frac{V_{\text{bid}} - V_{\text{ask}}}{V_{\text{bid}} + V_{\text{ask}}} \in [-1.0, 1.0]$$
2. **VPIN (Volume-Synchronized Probability of Toxicity):** Implements the Easley, López de Prado, and O'Hara (2012) framework. Trades are sliced into equal-volume buckets with tick-rule sign assignment to quantify the exact probability of informed predatory flow:
   $$\text{VPIN} = \frac{\sum_{\tau=1}^N \left| V_\tau^B - V_\tau^S \right|}{N \cdot V}$$
3. **Serpent Coil (Predatory Algo Hunter):** High-frequency tape inspection detecting:
   * **Momentum Ignition:** Aggressive market sweeps driving artificial moves without resting limit depth.
   * **Stop Hunting:** Liquidity sweeps through key thresholds triggering stop cascades, followed by sharp mean reversion.
   * **Layering & Spoofing:** Phantom book depth inserted and cancelled in $<100\mu\text{s}$ to manipulate micro-price.
   * **Quote Stuffing:** Message bursts ($>5,000\text{ msgs/sec}$) designed to congest competing feed handlers.
4. **Bathymetric Pressure Sizing:** Depth-based position sizing. As dark pool volume and VPIN toxicity rise, pressure increases non-linearly, scaling down position size to protect the capital base:
   $$\text{Pressure} = 1.0 - \exp\left(-\left(1.5 \cdot D_{\text{abyss}} + 1.2 \cdot \text{VPIN} + 0.9 \cdot P_{\text{dens}}\right)\right)$$
5. **The Six Archons Matrix:** Quantitative attribution of market dominance across the six ruling market entities: *Speed*, *Occult*, *Toll*, *Deception*, *Liquidity*, and *Volatility*.
6. **The Liturgy of the Terminal Short:** The daily canonical hours (Matins $\to$ Prime $\to$ Terce $\to$ Sext $\to$ None $\to$ Vespers $\to$ Compline).

---

## II. Terminal HUD & Real-Time Visualization

```
  ┌─ L2 ORDER BOOK LADDER: SPY ─────────────────────────────────┐
  │  ASK   504.25  │   9400 shares  │ ██████████████████       │
  │  ASK   504.24  │   6200 shares  │ ████████████             │
  │  ASK   504.23  │   3800 shares  │ ███████                  │
  │  ASK   504.22  │   2100 shares  │ ████                     │
  │  ASK   504.21  │   1400 shares  │ ██                       │
  ├─────────────────────── SPREAD:   0.4 bps ───────────────────┤
  │  MID: $  504.20  │ MICRO: $  504.20  │ OBI: +0.33           │
  ├─────────────────────────────────────────────────────────────┤
  │  BID   504.19  │   2800 shares  │ █████                    │
  │  BID   504.18  │   4200 shares  │ ████████                 │
  │  BID   504.17  │   7500 shares  │ ███████████████          │
  │  BID   504.16  │  11200 shares  │ ██████████████████████   │
  │  BID   504.15  │  18500 shares  │ ████████████████████████ │
  └─────────────────────────────────────────────────────────────┘

  ┌─ BATHYMETRIC PRESSURE & DEPTH SIZING ───────────────────────┐
  │ Pressure: [█████████████████████████████░░░░░░░]  81.8%     │
  │ Total Abyssal Depth:  40.8%  │ Dark Pool Share:  48.0%      │
  │ VPIN Flow Toxicity:   35.6%  │ Predator Density: 0.74       │
  │ Kelly Multiplier:     0.18x  │ Sizing Strategy: CRUSH DEFENSE│
  │ Action: ABYSSAL ZONE: Heavy adverse selection. Reduce size. │
  └─────────────────────────────────────────────────────────────┘

  ┌─ THE SIX MARKET ARCHONS (GOVERNING SPHERES) ────────────────┐
  │ ARCHON_DECEPTION │ [■■■■■■■■■■■■■■■■■■] 100.0% │ The Phantom Weaver  │
  │ ARCHON_SPEED     │ [■■■■■■■■■■■■■■■■■■]  98.4% │ The Microwave Herald│
  │ ARCHON_OCCULT    │ [■■■■■■■■■■■■■·····]  72.0% │ The Subterranean Mtr│
  │ ARCHON_LIQUIDITY │ [■■■■■■■■··········]  45.5% │ The Blind Behemoth  │
  │ ARCHON_VOLATILITY│ [■■■■■■············]  35.6% │ The Gamma Leviathan │
  │ ARCHON_TOLL      │ [··················]   1.0% │ The Spread Exactor  │
  ├─────────────────────────────────────────────────────────────┤
  │ REIGNING ARCHON: ARCHON_DECEPTION (The Phantom Weaver)      │
  │ Countermeasure:  Ignore Level 2 book volume beyond top-of-book│
  └─────────────────────────────────────────────────────────────┘

  ┌─ SUBTERRANEAN LIQUIDITY ROUTING MATRIX ─────────────────────┐
  │ VIRTU        │ Internalizer │ Mkt: 16.5% │ Tox: 0.72 │ Lat: 210µs │ [PREDATOR]
  │ CITADEL      │ Internalizer │ Mkt: 22.0% │ Tox: 0.78 │ Lat: 180µs │ [PREDATOR]
  │ JANE_STREET  │ Wholesaler   │ Mkt: 12.0% │ Tox: 0.85 │ Lat: 140µs │ [PREDATOR]
  │ TWO_SIGMA    │ Wholesaler   │ Mkt:  7.5% │ Tox: 0.65 │ Lat: 240µs │ [PREDATOR]
  │ UBS_ATS      │ DarkPoolATS  │ Mkt:  4.8% │ Tox: 0.45 │ Lat: 950µs │ [PASSIVE]
  │ NASDAQ       │ LitExchange  │ Mkt: 18.0% │ Tox: 0.25 │ Lat: 420µs │ [PASSIVE]
  │ NYSE         │ LitExchange  │ Mkt: 19.5% │ Tox: 0.28 │ Lat: 480µs │ [PASSIVE]
  │ SIP          │ Consolidated │ Mkt:100.0% │ Tox: 0.95 │ Lat:3800µs │ [PASSIVE]
  └─────────────────────────────────────────────────────────────┘
```

---

## III. CLI Reference

```bash
# Full Microstructure Reconnaissance (L2 Book, VPIN, Bathymetric Pressure, Archons)
the-abyss scan SPY
the-abyss scan QQQ
the-abyss scan NVDA

# Inspect Subterranean Routing Topology & SIP Latency Arbitrage Windows
the-abyss matrix

# Celebrate the Daily Office of the Terminal Short
the-abyss liturgy

# Command Reference
the-abyss help
```

---

## IV. Verification

```bash
cargo test --verbose
```

```
running 6 tests
test test_order_book_microstructure ... ok
test test_dark_pool_topology_and_sip_lag ... ok
test test_bathymetric_pressure_scaling ... ok
test test_vpin_calculation ... ok
test test_archon_detection_matrix ... ok
test test_serpent_coil_order_event_tracking ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

## 📜 License

MIT License. Designed and authored by Patrick Isles.  
*"Whatever in creation exists without my knowledge exists without my consent."*
