use std::collections::HashSet;

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

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum NameInCmsisSvdRepo {
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

impl Group {
    pub fn all() -> HashSet<Self> {
        Self::iter().collect()
    }

    pub fn name_in_cmsis_svd_repo(&self) -> Option<NameInCmsisSvdRepo> {
        if *self == Group::Unknown {
            return None;
        }
        Some(match self {
            Group::Atmel => NameInCmsisSvdRepo::Atmel,
            Group::Freescale => NameInCmsisSvdRepo::Freescale,
            Group::Fujitsu => NameInCmsisSvdRepo::Fujitsu,
            Group::FujitsuMB9AF1 => NameInCmsisSvdRepo::Fujitsu,
            Group::FujitsuMB9AF3 => NameInCmsisSvdRepo::Fujitsu,
            Group::FujitsuMB9AF4 => NameInCmsisSvdRepo::Fujitsu,
            Group::FujitsuMB9AFA => NameInCmsisSvdRepo::Fujitsu,
            Group::FujitsuMB9AFB => NameInCmsisSvdRepo::Fujitsu,
            Group::FujitsuMB9BF1 => NameInCmsisSvdRepo::Fujitsu,
            Group::FujitsuMB9BF2 => NameInCmsisSvdRepo::Fujitsu,
            Group::FujitsuMB9BF3 => NameInCmsisSvdRepo::Fujitsu,
            Group::FujitsuMB9B => NameInCmsisSvdRepo::Fujitsu,
            Group::FujitsuMB9BF4 => NameInCmsisSvdRepo::Fujitsu,
            Group::FujitsuMB9BF5 => NameInCmsisSvdRepo::Fujitsu,
            Group::FujitsuMB9BF6 => NameInCmsisSvdRepo::Fujitsu,
            Group::FujitsuMB9BFD => NameInCmsisSvdRepo::Fujitsu,
            Group::Holtek => NameInCmsisSvdRepo::Holtek,
            Group::Microchip => NameInCmsisSvdRepo::Microchip,
            Group::Nordic => NameInCmsisSvdRepo::Nordic,
            Group::Nuvoton => NameInCmsisSvdRepo::Nuvoton,
            Group::NXP => NameInCmsisSvdRepo::NXP,
            Group::SiliconLabs => NameInCmsisSvdRepo::SiliconLabs,
            Group::Spansion => NameInCmsisSvdRepo::Spansion,
            Group::STMicroF0 => NameInCmsisSvdRepo::STMicro,
            Group::STMicroF1 => NameInCmsisSvdRepo::STMicro,
            Group::STMicroF2 => NameInCmsisSvdRepo::STMicro,
            Group::STMicroF3 => NameInCmsisSvdRepo::STMicro,
            Group::STMicroF4 => NameInCmsisSvdRepo::STMicro,
            Group::STMicro => NameInCmsisSvdRepo::STMicro,
            Group::Toshiba => NameInCmsisSvdRepo::Toshiba,
            Group::SiFive => NameInCmsisSvdRepo::Toshiba,
            Group::TexasInstruments => NameInCmsisSvdRepo::TexasInstruments,
            Group::Espressif => NameInCmsisSvdRepo::Espressif,
            Group::Unknown => panic!("unhandled group {:?}", self),
        })
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.serialize(f)
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
                format!("https://raw.githubusercontent.com/cmsis-svd/cmsis-svd-data/main/data/{vendor:?}/{chip}.svd",
                vendor = self.group.name_in_cmsis_svd_repo().expect(
                    &format!("could not find a vendor in CMSIS SVD repo for group {:?}", self.group)
                ),
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
