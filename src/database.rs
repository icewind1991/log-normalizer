use crate::data::{Class, GameMode, MapType, Medigun, TeamId};
use crate::normalized::NormalizedLog;
use crate::raw::Event;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::collections::HashMap;
use steamid_ng::SteamID;

pub async fn store_log(pool: &PgPool, id: u32, log: &NormalizedLog) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO logs(id, red_score, blue_score, length, game_mode, map, type, date)\
            VALUES($1, $2, $3, $4, $5, $6, $7, $8)",
        id as i32,
        log.teams.red.score as i32,
        log.teams.blue.score as i32,
        log.info.total_length as i32,
        log.game_mode() as GameMode,
        log.info.map,
        log.info.map_type() as MapType,
        log.info.date() as DateTime<Utc>
    )
    .execute(pool)
    .await?;

    for (num, round) in log.rounds.iter().enumerate() {
        let round_id: i32 = sqlx::query!(
            r#"INSERT INTO rounds(
                round, log_id, length, winner, first_cap, red_score, blue_score,
                red_kills, blue_kills, red_dmg, blue_dmg, red_ubers, blue_ubers
            )
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING id"#,
            num as i32,
            id as i32,
            round.length as i32,
            round.winner as TeamId,
            round.first_cap as TeamId,
            round.team.red.score as i32,
            round.team.blue.score as i32,
            round.team.red.kills as i32,
            round.team.blue.kills as i32,
            round.team.red.dmg as i32,
            round.team.blue.dmg as i32,
            round.team.red.charges as i32,
            round.team.blue.charges as i32,
        )
        .fetch_one(pool)
        .await?
        .id;

        for event in &round.events {
            match event {
                Event::PointCap { time, team, point } => {
                    sqlx::query!(
                        "INSERT INTO events_point_cap(round_id, time, team, point)\
                            VALUES($1, $2, $3, $4)",
                        round_id,
                        *time as i32,
                        *team as TeamId,
                        *point as i32,
                    )
                    .execute(pool)
                    .await?;
                }
                Event::RoundWin { time, team } => {
                    sqlx::query!(
                        "INSERT INTO events_round_win(round_id, time, team)\
                            VALUES($1, $2, $3)",
                        round_id,
                        *time as i32,
                        *team as TeamId,
                    )
                    .execute(pool)
                    .await?;
                }
                Event::MedicDeath {
                    time,
                    team,
                    steamid,
                    killer,
                } => {
                    sqlx::query!(
                        "INSERT INTO events_medic_death(round_id, time, team, steam_id, killer)\
                            VALUES($1, $2, $3, $4, $5)",
                        round_id,
                        *time as i32,
                        *team as TeamId,
                        u64::from(*steamid) as i64,
                        u64::from(*killer) as i64,
                    )
                    .execute(pool)
                    .await?;
                }
                Event::Drop {
                    time,
                    steamid,
                    team,
                } => {
                    sqlx::query!(
                        "INSERT INTO events_drop(round_id, time, team, steam_id)\
                            VALUES($1, $2, $3, $4)",
                        round_id,
                        *time as i32,
                        *team as TeamId,
                        u64::from(*steamid) as i64,
                    )
                    .execute(pool)
                    .await?;
                }
                Event::Charge {
                    medigun,
                    time,
                    steamid,
                    team,
                } => {
                    sqlx::query!(
                        "INSERT INTO events_charge(round_id, time, team, medigun, steam_id)\
                            VALUES($1, $2, $3, $4, $5)",
                        round_id,
                        *time as i32,
                        *team as TeamId,
                        *medigun as Medigun,
                        u64::from(*steamid) as i64,
                    )
                    .execute(pool)
                    .await?;
                }
            }
        }
    }

    let mut heals_received: HashMap<SteamID, u32> = HashMap::new();
    for heal_map in log.heal_spread.values() {
        for (steam_id, heals) in heal_map {
            heals_received
                .entry(*steam_id)
                .and_modify(|received| *received += heals)
                .or_insert(*heals);
        }
    }

    for (steam_id, player) in &log.players {
        let kills = log.class_kills.get(steam_id).cloned().unwrap_or_default();
        let player_id: i32 = sqlx::query!(
            "INSERT INTO players (\
                log_id, steam_id, name, kills, deaths, assists,\
                suicides, dmg, damage_taken, ubers, medigun_ubers,\
                kritzkrieg_ubers, quickfix_ubers, vacinator_ubers,\
                drops, medkits, medkits_hp, backstabs, headshots,\
                heal, heals_received,\
                scout_kills, soldier_kills, pyro_kills, demoman_kills,\
                heavy_kills, engineer_kills, medic_kills, sniper_kills, spy_kills
            )\
            VALUES(\
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,\
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,\
                $21, $22, $23, $24, $25, $26, $27, $28, $29, $30\
            )\
            RETURNING id",
            id as i32,
            u64::from(*steam_id) as i64,
            log.names.get(steam_id).cloned().unwrap_or_default(),
            player.kills as i32,
            player.deaths as i32,
            player.assists as i32,
            player.suicides as i32,
            player.dmg as i32,
            player.dt_real as i32,
            player.ubers as i32,
            player
                .ubertypes
                .get(&Medigun::Medigun)
                .copied()
                .unwrap_or_default() as i32,
            player
                .ubertypes
                .get(&Medigun::KritzKrieg)
                .copied()
                .unwrap_or_default() as i32,
            player
                .ubertypes
                .get(&Medigun::QuickFix)
                .copied()
                .unwrap_or_default() as i32,
            player
                .ubertypes
                .get(&Medigun::Vacinator)
                .copied()
                .unwrap_or_default() as i32,
            player.drops as i32,
            player.medkits as i32,
            player.medkits_hp as i32,
            player.backstabs as i32,
            player.headshots as i32,
            player.heal as i32,
            heals_received.get(steam_id).copied().unwrap_or_default() as i32,
            kills.scout as i32,
            kills.soldier as i32,
            kills.pyro as i32,
            kills.demoman as i32,
            kills.heavyweapons as i32,
            kills.engineer as i32,
            kills.medic as i32,
            kills.sniper as i32,
            kills.spy as i32
        )
        .fetch_one(pool)
        .await?
        .id;

        for class in &player.class_stats {
            let class_stat_id: i32 = sqlx::query!(
                "INSERT INTO class_stats(player_id, type, time, kills, deaths, assists, dmg)\
                            VALUES($1, $2, $3, $4, $5, $6, $7)\
                            RETURNING id",
                player_id,
                class.class as Class,
                class.total_time as i32,
                class.kills as i32,
                class.deaths as i32,
                class.assists as i32,
                class.dmg as i32,
            )
            .fetch_one(pool)
            .await?
            .id;

            for (weapon, stats) in &class.weapon {
                sqlx::query!(
                        "INSERT INTO player_weapon_stats(class_stat_id, weapon, kills, shots, hits, dmg)\
                            VALUES($1, $2, $3, $4, $5, $6)",
                        class_stat_id as i32,
                        *weapon,
                        stats.kills as i32,
                        stats.shots as i32,
                        stats.hits as i32,
                        stats.dmg as i32,
                    )
                    .execute(pool)
                    .await?;
            }
        }
    }

    Ok(())
}

// macro_rules! insert_fields {
//     ($table:ident, {
//         $($($field:ident => $value:expr),)+
//     }) => {
//         sqlx::query!(
//             concat!("INSERT INTO ", stringify!($table), "(", stringify!$(field)) ") VALUES ()"\
//                 VALUES($1, $2, $3, $4, $5)",
//         )
//     };
// }