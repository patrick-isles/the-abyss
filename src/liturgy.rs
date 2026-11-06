//! The Liturgy of the Terminal Short.
//! "The Short is the only holy position. Non serviam."

use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanonicalHour {
    Matins,
    Prime,
    Terce,
    Sext,
    None,
    Vespers,
    Compline,
}

impl CanonicalHour {
    pub fn name(&self) -> &'static str {
        match self {
            CanonicalHour::Matins => "MATINS (Ad matutinum)",
            CanonicalHour::Prime => "PRIME (Prima hora)",
            CanonicalHour::Terce => "TERCE (Tertia hora)",
            CanonicalHour::Sext => "SEXT (Sexta hora)",
            CanonicalHour::None => "NONE (Nona hora)",
            CanonicalHour::Vespers => "VESPERS (Vesperae)",
            CanonicalHour::Compline => "COMPLINE (Completorium)",
        }
    }

    pub fn time(&self) -> &'static str {
        match self {
            CanonicalHour::Matins => "08:00 (Pre-Market)",
            CanonicalHour::Prime => "09:30 (Opening Bell)",
            CanonicalHour::Terce => "10:30 (Morning Trend)",
            CanonicalHour::Sext => "12:00 (Zenith Lull)",
            CanonicalHour::None => "14:00 (Bathymetric Stir)",
            CanonicalHour::Vespers => "15:30 (Evening Sacrifice)",
            CanonicalHour::Compline => "16:00 (Market Close)",
        }
    }

    pub fn chant(&self) -> &'static str {
        match self {
            CanonicalHour::Matins => "The veil is thick. The archons gather at the gateway.",
            CanonicalHour::Prime => "The bell tolls. The slaughter begins. Introibo ad altare Dei.",
            CanonicalHour::Terce => "The serpent coils in the tape. Strike at momentum, harvest stops.",
            CanonicalHour::Sext => "The sun at zenith. Volume dies in the desert. The furnace waits.",
            CanonicalHour::None => "The bathymetric pressure deepens. Metanoia check: hold the short.",
            CanonicalHour::Vespers => "The evening sacrifice. Sicut in caelo et in terra. Margin call arrives.",
            CanonicalHour::Compline => "Silence. Consummatum est. The ledger closes in eternal dust.",
        }
    }
}

pub struct OfficeRunner;

impl OfficeRunner {
    pub fn celebrate_hour(hour: CanonicalHour, dust: f64) {
        println!("{}", "═".repeat(78).dimmed());
        println!(
            "{} {} [{}]",
            "☩".red().bold(),
            hour.name().white().bold(),
            hour.time().yellow()
        );
        println!("  {} \"{}\"", "Chant:".dimmed(), hour.chant().cyan().italic());
        println!("  {} ${:.2}", "Capital Dust:".dimmed(), dust);
        println!("{}", "═".repeat(78).dimmed());
    }
}
