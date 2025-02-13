use std::{collections::HashSet, fmt::Debug};

use self::RunWhen::*;
use anyhow::Context;
use serde::Serialize as _;
use strum::IntoEnumIterator;
pub use svd2rust::Target;

#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    PartialOrd,
    Ord,
    PartialEq,
    Hash,
    Eq,
    Clone,
    Copy,
    strum_macros::EnumIter,
)]
pub enum Group {
    Atmel,
    Freescale,
    Fujitsu,
    FujitsuMB9AF1,
    FujitsuMB9AF3,
    FujitsuMB9AF4,
    FujitsuMB9AFA,
    FujitsuMB9AFB,
    FujitsuMB9B,
    FujitsuMB9BF1,
    FujitsuMB9BF2,
    FujitsuMB9BF3,
    FujitsuMB9BF4,
    FujitsuMB9BF5,
    FujitsuMB9BF6,
    FujitsuMB9BFD,
    Holtek,
    Microchip,
    Nordic,
    Nuvoton,
    NXP,
    SiliconLabs,
    Spansion,
    STMicroF0,
    STMicroF1,
    STMicroF2,
    STMicroF3,
    STMicroF4,
    STMicro,
    Toshiba,
    SiFive,
    TexasInstruments,
    Espressif,
    Unknown,
}

impl Group {
    pub fn all() -> HashSet<Self> {
        Self::iter().collect()
    }

    pub fn name_in_cmsis_svd_repo_as_str(&self) -> Option<String> {
        // By default, use the enum value as a string. In case we have a missmatch
        // between the name in the SVD repo and the vendor name, those cases could be handled
        // explicitely in the future.
        self.vendor().map(|vendor| vendor.to_string())
    }

    pub fn vendor(&self) -> Option<Vendor> {
        if *self == Group::Unknown {
            return None;
        }
        Some(match self {
            Group::Atmel => Vendor::Atmel,
            Group::Freescale => Vendor::Freescale,
            Group::Fujitsu => Vendor::Fujitsu,
            Group::FujitsuMB9AF1 => Vendor::Fujitsu,
            Group::FujitsuMB9AF3 => Vendor::Fujitsu,
            Group::FujitsuMB9AF4 => Vendor::Fujitsu,
            Group::FujitsuMB9AFA => Vendor::Fujitsu,
            Group::FujitsuMB9AFB => Vendor::Fujitsu,
            Group::FujitsuMB9BF1 => Vendor::Fujitsu,
            Group::FujitsuMB9BF2 => Vendor::Fujitsu,
            Group::FujitsuMB9BF3 => Vendor::Fujitsu,
            Group::FujitsuMB9B => Vendor::Fujitsu,
            Group::FujitsuMB9BF4 => Vendor::Fujitsu,
            Group::FujitsuMB9BF5 => Vendor::Fujitsu,
            Group::FujitsuMB9BF6 => Vendor::Fujitsu,
            Group::FujitsuMB9BFD => Vendor::Fujitsu,
            Group::Holtek => Vendor::Holtek,
            Group::Microchip => Vendor::Microchip,
            Group::Nordic => Vendor::Nordic,
            Group::Nuvoton => Vendor::Nuvoton,
            Group::NXP => Vendor::NXP,
            Group::SiliconLabs => Vendor::SiliconLabs,
            Group::Spansion => Vendor::Spansion,
            Group::STMicroF0 => Vendor::STMicro,
            Group::STMicroF1 => Vendor::STMicro,
            Group::STMicroF2 => Vendor::STMicro,
            Group::STMicroF3 => Vendor::STMicro,
            Group::STMicroF4 => Vendor::STMicro,
            Group::STMicro => Vendor::STMicro,
            Group::Toshiba => Vendor::Toshiba,
            Group::SiFive => Vendor::Toshiba,
            Group::TexasInstruments => Vendor::TexasInstruments,
            Group::Espressif => Vendor::Espressif,
            Group::Unknown => panic!("unhandled group {:?}", self),
        })
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.serialize(f)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    strum_macros::EnumIter,
)]
pub enum Vendor {
    Atmel,
    Freescale,
    Fujitsu,
    Holtek,
    Microchip,
    Nordic,
    Nuvoton,
    NXP,
    SiliconLabs,
    Spansion,
    STMicro,
    Toshiba,
    SiFive,
    TexasInstruments,
    Espressif,
    Unknown,
}

impl std::fmt::Display for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.serialize(f)
    }
}
impl Vendor {
    pub fn all() -> HashSet<Self> {
        Self::iter().collect()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum RunWhen {
    #[default]
    Always,
    NotShort,

    // TODO: Never doesn't really do anything right now
    Never,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct TestCase {
    pub arch: Target,
    pub group: Group,
    pub chip: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opts: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svd_url: Option<String>,
    #[serde(default = "true_")]
    pub should_pass: bool,
    #[serde(default)]
    pub skip_check: bool,
    #[serde(default)]
    pub run_when: RunWhen,
}

fn true_() -> bool {
    true
}

impl TestCase {
    pub fn svd_url(&self) -> String {
        match &self.svd_url {
            Some(u) => u.to_owned(),
            None => {
                let vendor_str = self
                    .group
                    .name_in_cmsis_svd_repo_as_str()
                    .unwrap_or_else(|| {
                        panic!(
                            "could not find a vendor in CMSIS SVD repo for group {:?}",
                            self.group
                        )
                    });
                format!("https://raw.githubusercontent.com/cmsis-svd/cmsis-svd-data/main/data/{vendor_str}/{chip}.svd",
                    chip = self.chip
                )
            }
        }
    }

    pub const fn should_run(&self, short_test: bool) -> bool {
        match (&self.run_when, short_test) {
            (&Always, _) => true,
            (&NotShort, true) => false,
            (_, _) => true,
        }
    }

    pub fn name(&self) -> String {
        let mut base_name = format!("{:?}-{}", self.group, self.chip.replace('.', "_"));
        if let Some(suffix) = &self.suffix {
            base_name.push('-');
            base_name.push_str(suffix);
        }
        base_name
    }
}

pub fn tests(test_cases: Option<&std::path::Path>) -> Result<&'static [TestCase], anyhow::Error> {
    pub static TESTS: std::sync::OnceLock<Vec<TestCase>> = std::sync::OnceLock::new();

    if let Some(cases) = TESTS.get() {
        Ok(cases)
    } else {
        let path = test_cases.ok_or_else(|| anyhow::format_err!("no test cases specified"))?;
        let cases: Vec<TestCase> = if path.extension() != Some(std::ffi::OsStr::new("yml")) {
            serde_json::from_reader(
                std::fs::OpenOptions::new()
                    .read(true)
                    .open(path)
                    .with_context(|| format!("couldn't open file {}", path.display()))?,
            )?
        } else if path.extension() != Some(std::ffi::OsStr::new("json")) {
            serde_yaml::from_reader(
                std::fs::OpenOptions::new()
                    .read(true)
                    .open(path)
                    .with_context(|| format!("couldn't open file {}", path.display()))?,
            )?
        } else {
            anyhow::bail!("unknown file extension for {}", path.display());
        };
        Ok(TESTS.get_or_init(|| cases))
    }
}
