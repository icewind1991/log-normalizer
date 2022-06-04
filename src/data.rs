use serde::Deserialize;

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Eq, PartialEq)]
#[sqlx(type_name = "team")]
#[sqlx(rename_all = "lowercase")]
pub enum TeamId {
    Blue,
    Red,
    #[serde(other)]
    Other,
}

impl Default for TeamId {
    fn default() -> Self {
        TeamId::Other
    }
}

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Eq, PartialEq)]
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

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Eq, PartialEq)]
pub enum EventType {
    Charge,
    PointCap,
    MedicDeath,
    RoundWin,
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum Medigun {
    KritzKrieg,
    QuickFix,
    Vaccinator,
    #[serde(other)]
    Medigun,
}

impl Default for Medigun {
    fn default() -> Self {
        Medigun::Medigun
    }
}

#[derive(Debug, Clone, Copy, sqlx::Type, Eq, PartialEq)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "map_type")]
pub enum MapType {
    Stopwatch,
    Cp,
    KOTH,
    CTF,
    UltiDuo,
    BBall,
    Other,
}

impl Default for MapType {
    fn default() -> Self {
        MapType::Other
    }
}
