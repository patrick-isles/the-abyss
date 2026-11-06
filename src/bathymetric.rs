//! Bathymetric Pressure — Depth-Based Position Sizing.
//! "The deeper the dark pool volume, the greater the pressure.
//! The greater the pressure, the smaller the position."

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BathymetricCalculation {
    pub symbol: String,
    pub dark_pool_depth: f64,       // [0.0, 1.0]
    pub internalizer_depth: f64,    // [0.0, 1.0]
    pub total_abyssal_depth: f64,   // [0.0, 1.0]
    pub predator_density: f64,      // [0.0, 1.0]
    pub flow_toxicity_vpin: f64,    // [0.0, 1.0]
    pub bathymetric_pressure: f64,  // [0.0, 1.0]
    pub kelly_size_multiplier: f64, // Position multiplier [0.05, 1.0]
    pub recommended_action: &'static str,
}

pub struct BathymetricEngine;

impl BathymetricEngine {
    pub fn calculate(
        symbol: &str,
        dark_volume_share: f64,
        internalizer_share: f64,
        predator_density: f64,
        vpin: f64,
    ) -> BathymetricCalculation {
        let dark_depth = dark_volume_share.clamp(0.0, 1.0);
        let int_depth = internalizer_share.clamp(0.0, 1.0);
        let total_depth = (0.55 * dark_depth + 0.45 * int_depth).clamp(0.0, 1.0);
        let pred_dens = predator_density.clamp(0.0, 1.0);
        let toxicity = vpin.clamp(0.0, 1.0);

        // Submarine pressure equation: non-linear exponential increase with depth & toxicity
        let exponent = -(1.5 * total_depth + 1.2 * toxicity + 0.9 * pred_dens);
        let pressure = (1.0 - exponent.exp()).clamp(0.0, 1.0);

        // Sizing contraction: Kelly multiplier decays with pressure
        let kelly_mult = (1.0 - pressure).max(0.05);

        let action = if pressure > 0.85 {
            "CRUSH DEPTH: Full evasive silent running. Minimum sizing (0.05x). Predator trap active."
        } else if pressure > 0.65 {
            "ABYSSAL ZONE: Heavy adverse selection. Reduce size to quarter Kelly (0.25x)."
        } else if pressure > 0.40 {
            "BATHYPELAGIC: Moderate predator presence. Half Kelly sizing (0.50x)."
        } else {
            "SURFACE WATERS: Benign liquidity. Standard Kelly execution (1.00x)."
        };

        BathymetricCalculation {
            symbol: symbol.to_string(),
            dark_pool_depth: dark_depth,
            internalizer_depth: int_depth,
            total_abyssal_depth: total_depth,
            predator_density: pred_dens,
            flow_toxicity_vpin: toxicity,
            bathymetric_pressure: pressure,
            kelly_size_multiplier: kelly_mult,
            recommended_action: action,
        }
    }
}
