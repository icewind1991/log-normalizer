pub use crate::data::TeamId;
use crate::data::{GameMode, MapType};
use crate::raw::RawLog;
pub use crate::raw::{
    ChatMessage, ClassNumbers, Event, KillStreak, Player, RoundPlayer, Team, Teams, Uploader,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::HashMap;
use steamid_ng::SteamID;

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "crate::raw::RawLog")]
pub struct NormalizedLog {
    pub version: u8,
    pub length: u32,
    pub teams: Teams,
    pub players: HashMap<SteamID, Player>,
    pub names: HashMap<SteamID, String>,
    pub rounds: Vec<Round>,
    pub heal_spread: HashMap<SteamID, HashMap<SteamID, u32>>,
    pub class_kills: HashMap<SteamID, ClassNumbers>,
    pub class_deaths: HashMap<SteamID, ClassNumbers>,
    pub class_kill_assists: HashMap<SteamID, ClassNumbers>,
    pub chat: Vec<ChatMessage>,
    pub info: Info,
    pub kill_streaks: Vec<KillStreak>,
}

impl NormalizedLog {
    pub fn game_mode(&self) -> GameMode {
        if self.info.map_type() == MapType::UltiDuo {
            return GameMode::UltiDuo;
        }

        match self.players.len() {
            7..=9 => GameMode::Fours,
            11..=13 => GameMode::Sixes,
            14 => GameMode::Sevens,
            17..=19 => GameMode::Highlander,
            _ => GameMode::Other,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Info {
    pub map: String,
    pub total_length: u32,
    pub supplemental: bool,
    pub has_real_damage: bool,
    pub has_weapon_damage: bool,
    pub has_accuracy: bool,
    pub has_hp: bool,
    pub has_hp_real: bool,
    pub has_hs: bool,
    pub has_hs_hit: bool,
    pub has_bs: bool,
    pub has_cp: bool,
    pub has_sb: bool,
    pub has_dt: bool,
    pub has_as: bool,
    pub has_hr: bool,
    pub has_intel: bool,
    pub ad_scoring: bool,
    pub title: String,
    pub date: u64,
    pub uploader: Uploader,
}

impl Info {
    pub fn map_type(&self) -> MapType {
        if map_is_stopwatch(&self.map) {
            MapType::Stopwatch
        } else if self.map.starts_with("cp") {
            MapType::Cp
        } else if self.map.starts_with("koth") {
            MapType::Koth
        } else if self.map.starts_with("ctf") {
            MapType::Ctf
        } else if self.map.starts_with("ultiduo") {
            MapType::UltiDuo
        } else if self.map.starts_with("bball") {
            MapType::BBall
        } else {
            MapType::Other
        }
    }

    pub fn date(&self) -> DateTime<Utc> {
        NaiveDateTime::from_timestamp_opt(self.date as i64, 0)
            .unwrap()
            .and_utc()
    }
}

#[derive(Debug, Clone)]
pub struct Round {
    pub start_time: u64,
    pub winner: Option<TeamId>,
    pub first_cap: TeamId,
    pub length: u32,
    pub team: Teams,
    pub players: HashMap<SteamID, RoundPlayer>,
    pub events: Vec<Event>,
}

impl From<RawLog> for NormalizedLog {
    fn from(raw: RawLog) -> Self {
        let info = Info {
            map: raw.info.map,
            total_length: raw.info.total_length,
            supplemental: raw.info.supplemental,
            has_real_damage: raw.info.has_real_damage,
            has_weapon_damage: raw.info.has_weapon_damage,
            has_accuracy: raw.info.has_accuracy,
            has_hp: raw.info.has_hp,
            has_hp_real: raw.info.has_hp_real,
            has_hs: raw.info.has_hs,
            has_hs_hit: raw.info.has_hs_hit,
            has_bs: raw.info.has_bs,
            has_cp: raw.info.has_cp,
            has_sb: raw.info.has_sb,
            has_dt: raw.info.has_dt,
            has_as: raw.info.has_as,
            has_hr: raw.info.has_hr,
            has_intel: raw.info.has_intel,
            ad_scoring: raw.info.ad_scoring,
            title: raw.info.title,
            date: raw.info.date,
            uploader: raw.info.uploader,
        };
        let rounds: Vec<Round> = raw
            .rounds
            .or(raw.info.rounds)
            .unwrap_or_default()
            .into_iter()
            .map(Round::from)
            .collect();
        let teams = raw.teams.or(raw.info.teams).unwrap_or_default();

        let mut normalized = NormalizedLog {
            version: raw.version,
            length: raw.length,
            teams,
            players: raw.players,
            names: raw.names,
            rounds,
            heal_spread: raw.heal_spread,
            class_kills: raw.class_kills,
            class_deaths: raw.class_deaths,
            class_kill_assists: raw.class_kill_assists.unwrap_or_default(),
            chat: raw.chat,
            info,
            kill_streaks: raw.kill_streaks.unwrap_or_default(),
        };

        normalize_stopwatch_events(&mut normalized);
        filter_double_wins(&mut normalized);
        normalize_event_times(&mut normalized);
        normalize_stopwatch_score(&mut normalized);

        normalized
    }
}

impl From<crate::raw::Round> for Round {
    fn from(raw: crate::raw::Round) -> Self {
        let first_cap = raw
            .first_cap
            .or_else(|| {
                raw.events.iter().find_map(|event| match event {
                    Event::PointCap { team, .. } => *team,
                    _ => None,
                })
            })
            .unwrap_or(TeamId::Blue);
        let team = raw.team.or(raw.flat_team).unwrap_or_default();

        Round {
            start_time: raw.start_time,
            winner: raw.winner,
            first_cap,
            length: raw.length,
            team,
            players: raw.players,
            events: raw.events,
        }
    }
}

pub fn map_is_stopwatch(map: &str) -> bool {
    map.starts_with("pl_")
        || map.starts_with("cp_steel")
        || map.starts_with("cp_gravelpit")
        || map.starts_with("cp_dustbowl")
        || map.starts_with("cp_egypt")
        || map.starts_with("cp_degrootkeep")
        || map.starts_with("cp_gorge")
        || map.starts_with("cp_junction")
        || map.starts_with("cp_mossrock")
        || map.starts_with("cp_manor")
        || map.starts_with("cp_snowplow")
        || map.starts_with("cp_alloy")
}

/// Add missing round wins for 2nd round blue win
fn normalize_stopwatch_events(log: &mut NormalizedLog) {
    if map_is_stopwatch(&log.info.map)
        && log.rounds.len() >= 2
        && log.rounds[1].winner == Some(TeamId::Blue)
    {
        let first_half_rounds = get_round_point_capped(&log.rounds[0]);
        let second_half_rounds = get_round_point_capped(&log.rounds[1]);
        let second_half_end_time = get_round_end_time(&log.rounds[1]);

        // attackers won 2nd round so they have to have at least the same number of point caps
        // however some old demos dont properly include the last cap so we add them
        if second_half_rounds < first_half_rounds {
            let last_event = log.rounds[1].events.pop();
            log.rounds[1].events.push(Event::PointCap {
                time: second_half_end_time,
                team: Some(TeamId::Blue),
                point: first_half_rounds,
            });
            if let Some(last_event) = last_event {
                log.rounds[1].events.push(last_event);
            }
        }
    }
}

fn filter_double_wins(log: &mut NormalizedLog) {
    for round in log.rounds.iter_mut() {
        if let Some(index) = round
            .events
            .iter()
            .enumerate()
            .find(|(_index, event)| matches!(event, Event::RoundWin { .. }))
            .map(|(index, _event)| index)
        {
            round.events.truncate(index + 1);
        }
    }
}

fn get_round_end_time(round: &Round) -> u32 {
    round
        .events
        .iter()
        .filter_map(|event| match event {
            Event::RoundWin { time, .. } => Some(*time),
            _ => None,
        })
        .last()
        .unwrap_or_default()
}

fn get_first_event_time(round: &Round) -> u32 {
    round
        .events
        .iter()
        .map(|event| event.time())
        .last()
        .unwrap_or_default()
}

fn get_round_point_capped(round: &Round) -> u8 {
    round
        .events
        .iter()
        .filter_map(|event| match event {
            Event::PointCap { point, .. } => Some(*point),
            _ => None,
        })
        .last()
        .unwrap_or_default()
}

/// Old logs have event times reset each round, newer ones keep counting
fn normalize_event_times(log: &mut NormalizedLog) {
    let mut prev_round_end_time = 0;
    for round in log.rounds.iter_mut() {
        if get_first_event_time(round) < prev_round_end_time {
            round.events.iter_mut().for_each(|event| match event {
                Event::PointCap { time, .. } => *time += prev_round_end_time,
                Event::Charge { time, .. } => *time += prev_round_end_time,
                Event::Drop { time, .. } => *time += prev_round_end_time,
                Event::MedicDeath { time, .. } => *time += prev_round_end_time,
                Event::RoundWin { time, .. } => *time += prev_round_end_time,
                Event::Other => {}
            });
        }
        prev_round_end_time = get_round_end_time(round);
    }
}

fn get_last_cap_time(round: &Round) -> u32 {
    round
        .events
        .iter()
        .filter_map(|event| match event {
            Event::PointCap { time, .. } => Some(*time),
            _ => None,
        })
        .last()
        .unwrap_or_default()
}

/// Apply modern ad scoring to old demos
fn normalize_stopwatch_score(log: &mut NormalizedLog) {
    if !log.info.ad_scoring && map_is_stopwatch(&log.info.map) && log.rounds.len() == 2 {
        let first_half_capped = get_round_point_capped(&log.rounds[0]);
        let second_half_capped = get_round_point_capped(&log.rounds[1]);

        // "blue" is the team that attacked first
        match first_half_capped.cmp(&second_half_capped) {
            Ordering::Greater => {
                log.teams.blue.score = 1;
                log.teams.red.score = 0;
            }
            Ordering::Less => {
                log.teams.blue.score = 0;
                log.teams.red.score = 1;
            }
            Ordering::Equal => {
                let first_half_cap_time = get_last_cap_time(&log.rounds[0]);
                let second_half_cap_time = get_last_cap_time(&log.rounds[1])
                    .saturating_sub(get_round_end_time(&log.rounds[0]));

                if first_half_cap_time < second_half_cap_time {
                    log.teams.blue.score = 1;
                    log.teams.red.score = 0;
                } else {
                    log.teams.blue.score = 0;
                    log.teams.red.score = 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use test_case::test_case;

    fn parse(file: &str) -> NormalizedLog {
        let content = fs::read_to_string(format!("tests/data/{}", file)).unwrap();
        serde_json::from_str(&content).unwrap()
    }

    #[test_case("134389.json", 0, 1)]
    #[test_case("550237.json", 1, 0)]
    fn test_normalize_stopwatch_score(file: &str, blue: u32, red: u32) {
        let parsed = parse(file);

        assert_eq!(parsed.teams.blue.score, blue);
        assert_eq!(parsed.teams.red.score, red);
    }

    #[test_case("1.json")]
    #[test_case("134389.json")]
    #[test_case("550237.json")]
    #[test_case("2522305.json")]
    fn test_normalize_event_time(file: &str) {
        let parsed = parse(file);

        let mut last_event_time = 0;

        for event in parsed.rounds.iter().flat_map(|round| round.events.iter()) {
            assert!(event.time() >= last_event_time);
            last_event_time = event.time();
        }
    }

    #[test_case("1.json")]
    #[test_case("114840.json")]
    #[test_case("134389.json")]
    #[test_case("550237.json")]
    #[test_case("2522305.json")]
    fn test_round_win(file: &str) {
        let parsed = parse(file);

        for round in parsed.rounds {
            assert_eq!(
                1,
                round
                    .events
                    .into_iter()
                    .filter(|event| matches!(event, Event::RoundWin { .. }))
                    .count()
            );
        }
    }
}
