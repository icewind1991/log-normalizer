use crate::data::{Class, Medigun, TeamId};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::{serde_as, DefaultOnNull};
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use steamid_ng::SteamID;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawLog {
    #[serde(default)]
    pub version: u8,
    #[serde(default)]
    pub length: u32,
    pub teams: Option<Teams>,
    #[serde(deserialize_with = "deserialize_steam_id_bot_map")]
    pub players: HashMap<SteamID, Player>,
    #[serde(deserialize_with = "deserialize_steam_id_bot_map")]
    pub names: HashMap<SteamID, String>,
    pub rounds: Option<Vec<Round>>,
    #[serde(rename = "healspread")]
    #[serde(deserialize_with = "deserialize_steam_id_bot_map")]
    pub heal_spread: HashMap<SteamID, HashMap<SteamID, u32>>,
    #[serde(rename = "classkills")]
    #[serde(deserialize_with = "deserialize_steam_id_bot_map")]
    pub class_kills: HashMap<SteamID, ClassNumbers>,
    #[serde(rename = "classdeaths")]
    #[serde(deserialize_with = "deserialize_steam_id_bot_map")]
    pub class_deaths: HashMap<SteamID, ClassNumbers>,
    #[serde(rename = "classkillassists")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_steam_id_bot_map_opt")]
    pub class_kill_assists: Option<HashMap<SteamID, ClassNumbers>>,
    pub chat: Vec<ChatMessage>,
    pub info: Info,
    #[serde(rename = "killstreaks")]
    pub kill_streaks: Option<Vec<KillStreak>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Teams {
    pub red: Team,
    pub blue: Team,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Team {
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub score: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub kills: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub deaths: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub dmg: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub charges: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub drops: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub firstcaps: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub caps: u32,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Player {
    pub class_stats: Vec<ClassStat>,
    pub team: Option<TeamId>,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub kills: u16,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub deaths: u16,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub assists: u16,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub suicides: u16,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub dmg: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub dmg_real: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub dt: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub dt_real: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub hr: u16,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub lks: u16,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub ubers: u32,
    #[serde(default)]
    pub ubertypes: HashMap<Medigun, u32>,
    pub drops: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub medkits: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub medkits_hp: u16,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub backstabs: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub headshots: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub headshots_hit: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub heal: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub cpc: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub ic: u32,
    pub medicstat: Option<MedicStats>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MedicStats {
    pub advantages_lost: u32,
    pub biggest_advantage_list: u16,
    pub deaths_within_20s_after_uber: u32,
    pub deaths_with_95_99_uber: u32,
    pub avg_time_before_healing: f32,
    pub avg_time_to_build: f32,
    pub avg_time_before_using: f32,
    pub avg_uber_length: f32,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassStat {
    #[serde(rename = "type")]
    pub class: Class,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub kills: u16,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub assists: u16,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub deaths: u16,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub dmg: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub total_time: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(default)]
    pub weapon: HashMap<String, WeaponStat>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RawWeaponStats {
    Kills(u32),
    Stats {
        #[serde_as(deserialize_as = "DefaultOnNull")]
        kills: u32,
        #[serde_as(deserialize_as = "DefaultOnNull")]
        dmg: i64,
        #[serde_as(deserialize_as = "DefaultOnNull")]
        #[serde(default)]
        avg_dmg: f32,
        #[serde_as(deserialize_as = "DefaultOnNull")]
        shots: u32,
        #[serde_as(deserialize_as = "DefaultOnNull")]
        hits: u32,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(from = "RawWeaponStats")]
pub struct WeaponStat {
    pub kills: u32,
    pub dmg: u32,
    pub avg_dmg: f32,
    pub shots: u32,
    pub hits: u32,
}

impl From<RawWeaponStats> for WeaponStat {
    fn from(raw: RawWeaponStats) -> Self {
        match raw {
            RawWeaponStats::Kills(kills) => WeaponStat {
                kills,
                dmg: 0,
                avg_dmg: 0.0,
                shots: 0,
                hits: 0,
            },
            RawWeaponStats::Stats {
                kills,
                dmg,
                avg_dmg,
                shots,
                hits,
            } => WeaponStat {
                kills,
                dmg: if dmg > 0 && dmg < 100_000 {
                    dmg as u32
                } else {
                    0
                },
                avg_dmg: if dmg > 0 && dmg < 100_000 {
                    avg_dmg
                } else {
                    0.0
                },
                shots,
                hits,
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Round {
    #[serde(default)]
    pub start_time: u64,
    pub winner: Option<TeamId>,
    #[serde(rename = "firstcap")]
    pub first_cap: Option<TeamId>,
    pub length: u32,
    pub team: Option<Teams>,
    #[serde(flatten)]
    pub flat_team: Option<Teams>,
    #[serde(deserialize_with = "deserialize_steam_id_bot_map")]
    pub players: HashMap<SteamID, RoundPlayer>,
    pub events: Vec<Event>,
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoundPlayer {
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub kills: u32,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub dmg: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Charge {
        #[serde(default)]
        medigun: Medigun,
        time: u32,
        steamid: SteamID,
        team: Option<TeamId>,
    },
    #[serde(rename = "pointcap")]
    PointCap {
        time: u32,
        team: Option<TeamId>,
        point: u8,
    },
    MedicDeath {
        time: u32,
        team: Option<TeamId>,
        steamid: SteamID,
        killer: SteamID,
    },
    RoundWin {
        time: u32,
        team: Option<TeamId>,
    },
    Drop {
        time: u32,
        steamid: SteamID,
        team: Option<TeamId>,
    },
    #[serde(other)]
    Other,
}

impl Event {
    pub fn time(&self) -> u32 {
        match self {
            Event::RoundWin { time, .. } => *time,
            Event::Charge { time, .. } => *time,
            Event::Drop { time, .. } => *time,
            Event::MedicDeath { time, .. } => *time,
            Event::PointCap { time, .. } => *time,
            Event::Other => 0,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ClassNumbers {
    #[serde(default)]
    pub scout: u32,
    #[serde(default)]
    pub soldier: u32,
    #[serde(default)]
    pub pyro: u32,
    #[serde(default)]
    pub demoman: u32,
    #[serde(default)]
    pub heavyweapons: u32,
    #[serde(default)]
    pub engineer: u32,
    #[serde(default)]
    pub medic: u32,
    #[serde(default)]
    pub sniper: u32,
    #[serde(default)]
    pub spy: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatMessage {
    pub steamid: ChatFrom,
    pub name: String,
    pub msg: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
#[serde(try_from = "String")]
pub enum ChatFrom {
    Player(SteamID),
    Console,
}

impl TryFrom<String> for ChatFrom {
    type Error = steamid_ng::SteamIDError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "Console" {
            Ok(ChatFrom::Console)
        } else {
            value.as_str().try_into().map(ChatFrom::Player)
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Uploader {
    pub id: SteamID,
    pub name: String,
    pub info: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KillStreak {
    pub steamid: SteamID,
    pub streak: i32,
    pub time: i32,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum MaybeSteamId {
    Bot,
    SteamId(SteamID),
}

impl MaybeSteamId {
    fn steam_id(&self) -> Option<SteamID> {
        match self {
            MaybeSteamId::Bot => None,
            MaybeSteamId::SteamId(id) => Some(*id),
        }
    }
}

fn deserialize_steam_id_bot_map<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<HashMap<SteamID, T>, D::Error> {
    let map = <HashMap<MaybeSteamId, T>>::deserialize(deserializer)?;
    Ok(map
        .into_iter()
        .filter_map(|(k, v)| Some((k.steam_id()?, v)))
        .collect())
}

fn deserialize_steam_id_bot_map_opt<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<HashMap<SteamID, T>>, D::Error> {
    let opt = <Option<HashMap<MaybeSteamId, T>>>::deserialize(deserializer)?;
    Ok(opt.map(|map| {
        map.into_iter()
            .filter_map(|(k, v)| Some((k.steam_id()?, v)))
            .collect()
    }))
}

impl<'de> Deserialize<'de> for MaybeSteamId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = <Cow<str> as Deserialize>::deserialize(deserializer)?;
        match raw.as_ref() {
            "BOT" => Ok(MaybeSteamId::Bot),
            raw => SteamID::try_from(raw)
                .map_err(D::Error::custom)
                .map(MaybeSteamId::SteamId),
        }
    }
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
    #[test_case("3578739.json")]
    #[test_case("3579548.json")]
    fn test_parse(file: &str) {
        let content = fs::read_to_string(format!("tests/data/{}", file)).unwrap();
        let parsed: RawLog = serde_json::from_str(&content).unwrap();
        assert!(parsed.teams.is_some() || parsed.info.teams.is_some());
        assert!(parsed.rounds.is_some() || parsed.info.rounds.is_some());

        for round in parsed
            .rounds
            .as_ref()
            .or(parsed.info.rounds.as_ref())
            .unwrap()
        {
            assert!(round.flat_team.is_some() || round.team.is_some());
        }

        insta::with_settings!({sort_maps => true, snapshot_path => "../tests/data/snapshots"}, {
            insta::assert_ron_snapshot!(file, parsed);
        });
    }
}
