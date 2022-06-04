use crate::data::{Class, GameMode, MapType, Medigun, TeamId};
use crate::normalized::NormalizedLog;
use crate::raw::Event;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::collections::HashMap;
use steamid_ng::SteamID;
use tracing::instrument;

#[instrument(skip(pool, log))]
pub async fn store_log(pool: &PgPool, id: i32, log: &NormalizedLog) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "INSERT INTO logs(id, red_score, blue_score, length, game_mode, map, type, date, version)\
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        id,
        log.teams.red.score as i32,
        log.teams.blue.score as i32,
        log.info.total_length as i32,
        log.game_mode() as GameMode,
        log.info.map,
        log.info.map_type() as MapType,
        log.info.date() as DateTime<Utc>,
        2
    )
    .execute(&mut tx)
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
            id,
            round.length as i32,
            round.winner.unwrap_or_default() as TeamId,
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
        .fetch_one(&mut tx)
        .await?
        .id;

        for event in &round.events {
            match event {
                Event::PointCap { time, team, point } => {
                    if let Some(team) = team {
                        sqlx::query!(
                            "INSERT INTO events_point_cap(round_id, time, team, point)\
                            VALUES($1, $2, $3, $4)",
                            round_id,
                            *time as i32,
                            *team as TeamId,
                            *point as i32,
                        )
                        .execute(&mut tx)
                        .await?;
                    }
                }
                Event::RoundWin { time, team } => {
                    if let Some(team) = team {
                        sqlx::query!(
                            "INSERT INTO events_round_win(round_id, time, team)\
                            VALUES($1, $2, $3)",
                            round_id,
                            *time as i32,
                            team.unwrap_or_default() as TeamId,
                        )
                        .execute(&mut tx)
                        .await?;
                    }
                }
                Event::MedicDeath {
                    time,
                    team,
                    steamid,
                    killer,
                } => {
                    if let Some(team) = team {
                        sqlx::query!(
                        "INSERT INTO events_medic_death(round_id, time, team, steam_id, killer)\
                            VALUES($1, $2, $3, $4, $5)",
                        round_id,
                        *time as i32,
                        *team as TeamId,
                        u64::from(*steamid) as i64,
                        u64::from(*killer) as i64,
                    )
                        .execute(&mut tx)
                        .await?;
                    }
                }
                Event::Drop {
                    time,
                    steamid,
                    team,
                } => {
                    if let Some(team) = team {
                        sqlx::query!(
                            "INSERT INTO events_drop(round_id, time, team, steam_id)\
                            VALUES($1, $2, $3, $4)",
                            round_id,
                            *time as i32,
                            *team as TeamId,
                            u64::from(*steamid) as i64,
                        )
                        .execute(&mut tx)
                        .await?;
                    }
                }
                Event::Charge {
                    medigun,
                    time,
                    steamid,
                    team,
                } => {
                    if let Some(team) = team {
                        sqlx::query!(
                            "INSERT INTO events_charge(round_id, time, team, medigun, steam_id)\
                            VALUES($1, $2, $3, $4, $5)",
                            round_id,
                            *time as i32,
                            *team as TeamId,
                            *medigun as Medigun,
                            u64::from(*steamid) as i64,
                        )
                        .execute(&mut tx)
                        .await?;
                    }
                }
                _ => {}
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
        if let Some(team) = player.team {
            let kills = log.class_kills.get(steam_id).cloned().unwrap_or_default();
            let deaths = log.class_deaths.get(steam_id).cloned().unwrap_or_default();
            let player_id: i64 = sqlx::query!(
                "INSERT INTO players (\
                log_id, steam_id, name, team, kills, deaths, assists,\
                suicides, dmg, damage_taken, ubers, medigun_ubers,\
                kritzkrieg_ubers, quickfix_ubers, vaccinator_ubers,\
                drops, medkits, medkits_hp, backstabs, headshots,\
                heal, heals_received,\
                scout_kills, soldier_kills, pyro_kills, demoman_kills,\
                heavy_kills, engineer_kills, medic_kills, sniper_kills, spy_kills,
                scout_deaths, soldier_deaths, pyro_deaths, demoman_deaths,\
                heavy_deaths, engineer_deaths, medic_deaths, sniper_deaths, spy_deaths
            )\
            VALUES(\
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,\
                $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,\
                $21, $22, $23, $24, $25, $26, $27, $28, $29, $30,\
                $31, $32, $33, $34, $35, $36, $37, $38, $39, $40\
            )\
            RETURNING id",
                id as i32,
                u64::from(*steam_id) as i64,
                log.names.get(steam_id).cloned().unwrap_or_default(),
                team as TeamId,
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
                    .get(&Medigun::Vaccinator)
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
                kills.spy as i32,
                deaths.scout as i32,
                deaths.soldier as i32,
                deaths.pyro as i32,
                deaths.demoman as i32,
                deaths.heavyweapons as i32,
                deaths.engineer as i32,
                deaths.medic as i32,
                deaths.sniper as i32,
                deaths.spy as i32,
            )
            .fetch_one(&mut tx)
            .await?
            .id;

            for class in &player.class_stats {
                if class.class != Class::Unknown {
                    let class_stat_id: i64 = sqlx::query!(
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
                    .fetch_one(&mut tx)
                    .await?
                    .id;

                    for (weapon, stats) in &class.weapon {
                        sqlx::query!(
                            "INSERT INTO player_weapon_stats(class_stat_id, weapon, kills, shots, hits, dmg)\
                                VALUES($1, $2, $3, $4, $5, $6)",
                            class_stat_id,
                            *weapon,
                            stats.kills as i32,
                            stats.shots as i32,
                            stats.hits as i32,
                            stats.dmg as i32,
                        )
                            .execute(&mut tx)
                            .await?;
                    }
                }
            }
        }
    }

    for kill_streak in &log.kill_streaks {
        sqlx::query!(
            "INSERT INTO kill_streaks(log_id, steam_id, time, streak)\
                VALUES($1, $2, $3, $4)",
            id,
            u64::from(kill_streak.steamid) as i64,
            kill_streak.time,
            kill_streak.streak,
        )
        .execute(&mut tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}

#[instrument(skip(pool, log))]
pub async fn upgrade(
    pool: &PgPool,
    id: i32,
    log: &NormalizedLog,
    from: i16,
    to: i16,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    if from <= 1 && to >= 2 {
        for kill_streak in &log.kill_streaks {
            sqlx::query!(
                "INSERT INTO kill_streaks(log_id, steam_id, time, streak)\
                VALUES($1, $2, $3, $4)",
                id,
                u64::from(kill_streak.steamid) as i64,
                kill_streak.time,
                kill_streak.streak,
            )
            .execute(&mut tx)
            .await?;
        }
    }

    sqlx::query!("UPDATE logs SET version = $1 WHERE id = $2", to, id)
        .execute(&mut tx)
        .await?;

    tx.commit().await?;

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
