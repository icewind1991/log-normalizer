use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Serialize, Eq, PartialEq, Default)]
#[sqlx(type_name = "team")]
#[sqlx(rename_all = "lowercase")]
pub enum TeamId {
    Blue,
    Red,
    #[serde(other)]
    #[default]
    Other,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "class_type")]
pub enum Class {
    Scout,
    Soldier,
    Pyro,
    Demoman,
    HeavyWeapons,
    Engineer,
    Medic,
    Sniper,
    Spy,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Eq, PartialEq)]
#[sqlx(type_name = "game_mode")]
pub enum GameMode {
    #[sqlx(rename = "ultiduo")]
    UltiDuo,
    #[sqlx(rename = "4v4")]
    Fours,
    #[sqlx(rename = "6v6")]
    Sixes,
    #[sqlx(rename = "7v7")]
    Sevens,
    #[sqlx(rename = "9v9")]
    Highlander,
    #[sqlx(rename = "other")]
    Other,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Serialize, Eq, PartialEq)]
pub enum EventType {
    Charge,
    PointCap,
    MedicDeath,
    RoundWin,
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Serialize, Hash, Eq, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum Medigun {
    KritzKrieg,
    QuickFix,
    Vaccinator,
    #[serde(other)]
    #[default]
    Medigun,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Eq, PartialEq, Default)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "map_type")]
pub enum MapType {
    Stopwatch,
    Cp,
    Koth,
    Ctf,
    UltiDuo,
    BBall,
    #[default]
    Other,
}
