use crate::data::{Class, Medigun, TeamId, Weapon};
use serde::export::TryFrom;
use serde::Deserialize;
use std::collections::HashMap;
use steamid_ng::SteamID;

#[derive(Debug, Clone, Deserialize)]
pub struct RawLog {
    #[serde(default)]
    pub version: u8,
    #[serde(default)]
    pub length: u32,
    pub teams: Option<Teams>,
    pub players: HashMap<SteamID, Player>,
    pub names: HashMap<SteamID, String>,
    pub rounds: Option<Vec<Round>>,
    #[serde(rename = "healspread")]
    pub heal_spread: HashMap<SteamID, HashMap<SteamID, u32>>,
    #[serde(rename = "classkills")]
    pub class_kills: HashMap<SteamID, ClassNumbers>,
    #[serde(rename = "classdeaths")]
    pub class_deaths: HashMap<SteamID, ClassNumbers>,
    #[serde(rename = "classkillassists")]
    pub class_kill_assists: Option<HashMap<SteamID, ClassNumbers>>,
    pub chat: Vec<ChatMessage>,
    pub info: Info,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Teams {
    pub red: Team,
    pub blue: Team,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Team {
    pub score: u8,
    #[serde(default)]
    pub kills: u32,
    #[serde(default)]
    pub deaths: u32,
    #[serde(default)]
    pub dmg: u32,
    #[serde(default)]
    pub charges: u32,
    #[serde(default)]
    pub drops: u32,
    #[serde(default)]
    pub firstcaps: u32,
    #[serde(default)]
    pub caps: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Player {
    pub team: TeamId,
    pub kills: u16,
    pub deaths: u16,
    pub assists: u16,
    #[serde(default)]
    pub suicides: u16,
    #[serde(default)]
    pub dmg: u32,
    #[serde(default)]
    pub dmg_real: u32,
    #[serde(default)]
    pub dt: u32,
    #[serde(default)]
    pub dt_real: u32,
    #[serde(default)]
    pub hr: u16,
    #[serde(default)]
    pub lks: u16,
    pub ubers: u8,
    #[serde(default)]
    pub ubertypes: HashMap<Medigun, u8>,
    pub drops: u8,
    pub medkits: u8,
    #[serde(default)]
    pub medkits_hp: u16,
    pub backstabs: u8,
    pub headshots: u8,
    #[serde(default)]
    pub headshots_hit: u8,
    pub heal: u32,
    pub cpc: u8,
    pub ic: u8,
    pub medicstat: Option<MedicStats>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MedicStats {
    pub advantages_lost: u8,
    pub biggest_advantage_list: u16,
    pub deaths_within_20s_after_uber: u8,
    pub deaths_with_95_99_uber: u8,
    pub avg_time_before_healing: f32,
    pub avg_time_to_build: f32,
    pub avg_time_before_using: f32,
    pub avg_uber_length: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClassStat {
    #[serde(rename = "type")]
    pub class: Class,
    pub kills: u16,
    pub dmg: u32,
    pub total_time: u32,
    pub weapon: HashMap<Weapon, WeaponStat>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeaponStat {
    pub kills: u32,
    pub dmg: u32,
    pub avg_dmg: u32,
    pub shots: u32,
    pub hits: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Round {
    #[serde(default)]
    pub start_time: u64,
    pub winner: TeamId,
    #[serde(rename = "firstcap")]
    pub first_cap: Option<TeamId>,
    pub length: u32,
    pub team: Option<Teams>,
    #[serde(flatten)]
    pub flat_team: Option<Teams>,
    pub players: HashMap<SteamID, RoundPlayer>,
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RoundPlayer {
    pub kills: u32,
    pub dmg: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Charge {
        #[serde(default)]
        medigun: Medigun,
        time: u32,
        steamid: SteamID,
        team: TeamId,
    },
    #[serde(rename = "pointcap")]
    PointCap {
        time: u32,
        team: TeamId,
        point: u8,
    },
    MedicDeath {
        time: u32,
        team: TeamId,
        steamid: SteamID,
        killer: SteamID,
    },
    RoundWin {
        time: u32,
        team: TeamId,
    },
    Drop {
        time: u32,
        steamid: SteamID,
        team: TeamId,
    },
}

impl Event {
    pub fn time(&self) -> u32 {
        match self {
            Event::RoundWin { time, .. } => *time,
            Event::Charge { time, .. } => *time,
            Event::Drop { time, .. } => *time,
            Event::MedicDeath { time, .. } => *time,
            Event::PointCap { time, .. } => *time,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClassNumbers {
    #[serde(default)]
    pub scout: u8,
    #[serde(default)]
    pub soldier: u8,
    #[serde(default)]
    pub pyro: u8,
    #[serde(default)]
    pub demoman: u8,
    #[serde(default)]
    pub heavyweapons: u8,
    #[serde(default)]
    pub engineer: u8,
    #[serde(default)]
    pub medic: u8,
    #[serde(default)]
    pub sniper: u8,
    #[serde(default)]
    pub spy: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatMessage {
    pub steamid: ChatFrom,
    pub name: String,
    pub msg: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
#[serde(try_from = "String")]
pub enum ChatFrom {
    Player(SteamID),
    Console,
}

impl TryFrom<String> for ChatFrom {
    type Error = steamid_ng::SteamIDParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "Console" {
            Ok(ChatFrom::Console)
        } else {
            value.parse().map(ChatFrom::Player)
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    pub map: String,
    pub total_length: u32,
    #[serde(default)]
    pub supplemental: bool,
    #[serde(default)]
    pub has_real_damage: bool,
    #[serde(default)]
    pub has_weapon_damage: bool,
    #[serde(default)]
    pub has_accuracy: bool,
    #[serde(default)]
    pub has_hp: bool,
    #[serde(default)]
    pub has_hp_real: bool,
    #[serde(default)]
    pub has_hs: bool,
    #[serde(default)]
    pub has_hs_hit: bool,
    #[serde(default)]
    pub has_bs: bool,
    #[serde(default)]
    pub has_cp: bool,
    #[serde(default)]
    pub has_sb: bool,
    #[serde(default)]
    pub has_dt: bool,
    #[serde(default)]
    pub has_as: bool,
    #[serde(default)]
    pub has_hr: bool,
    #[serde(default)]
    pub has_intel: bool,
    #[serde(default)]
    #[serde(rename = "AD_scoring")]
    pub ad_scoring: bool,
    pub title: String,
    pub date: u64,
    pub uploader: Uploader,
    pub rounds: Option<Vec<Round>>,
    #[serde(flatten)]
    pub teams: Option<Teams>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Uploader {
    pub id: SteamID,
    pub name: String,
    pub info: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use test_case::test_case;

    #[test_case("1.json")]
    #[test_case("134389.json")]
    #[test_case("550237.json")]
    #[test_case("2522305.json")]
    fn test_parse(file: &str) {
        let content = fs::read_to_string(format!("tests/data/{}", file)).unwrap();
        let parsed: RawLog = serde_json::from_str(&content).unwrap();
        assert!(parsed.teams.is_some() || parsed.info.teams.is_some());
        assert!(parsed.rounds.is_some() || parsed.info.rounds.is_some());

        for round in parsed.rounds.or(parsed.info.rounds).unwrap() {
            assert!(round.flat_team.is_some() || round.team.is_some());
        }
    }
}
