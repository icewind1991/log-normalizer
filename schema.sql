CREATE EXTENSION IF NOT EXISTS pg_trgm WITH SCHEMA public;

CREATE TYPE team AS ENUM ('blue', 'red', 'other');

CREATE TYPE class_type AS ENUM ('scout', 'soldier', 'pyro', 'demoman', 'heavyweapons', 'engineer', 'medic', 'sniper', 'spy', 'unknown');

CREATE TYPE game_mode AS ENUM ('ultiduo', '4v4', '6v6', '7v7', '9v9', 'other');

CREATE TYPE map_type AS ENUM ('stopwatch', 'cp', 'koth', 'ctf', 'ultiduo', 'bball', 'other');

CREATE TYPE event_type AS ENUM ('charge', 'pointcap', 'medic_death', 'round_win');

CREATE TYPE medigun AS ENUM ('medigun', 'kritzkrieg', 'quickfix', 'vaccinator');

CREATE FUNCTION clean_map_name(map TEXT) RETURNS TEXT AS $$
    SELECT regexp_replace(map, '(_(a|b|beta|u|r|v|rc|final|comptf|ugc)?[0-9]*[a-z]?$)|([0-9]+[a-z]?$)', '', 'g');
$$ LANGUAGE SQL IMMUTABLE;

CREATE TABLE logs (
    id              INTEGER                     PRIMARY KEY,
    red_score       INTEGER                     NOT NULL,
    blue_score      INTEGER                     NOT NULL,
    length          INTEGER                     NOT NULL,
    game_mode       game_mode                   NOT NULL,
    map             TEXT                        NOT NULL,
    clean_map       TEXT GENERATED ALWAYS AS (clean_map_name(map)) STORED,
    type            map_type                    NOT NULL,
    date            TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    winner          team GENERATED ALWAYS AS (CASE WHEN red_score > blue_score THEN 'red'::team WHEN blue_score > red_score THEN 'blue'::team ELSE 'other'::team END) STORED
);

CREATE INDEX logs_map_idx
    ON logs USING BTREE (map);

CREATE INDEX logs_clean_map_idx
    ON logs USING BTREE (clean_map);

CREATE INDEX logs_mode_idx
    ON logs USING BTREE (game_mode);

CREATE INDEX logs_winner_idx
    ON logs USING BTREE (winner);

CREATE INDEX logs_date_idx
    ON logs USING BTREE (date);

CREATE TABLE rounds (
    id              SERIAL                      PRIMARY KEY,
    round           INTEGER                     NOT NULL,
    log_id          INTEGER                     NOT NULL REFERENCES logs(id),
    length          INTEGER                     NOT NULL,
    winner          team                        NOT NULL,
    first_cap       team                        NOT NULL,
    red_score       INTEGER                     NOT NULL,
    blue_score      INTEGER                     NOT NULL,
    red_kills       INTEGER                     NOT NULL,
    blue_kills      INTEGER                     NOT NULL,
    red_dmg         INTEGER                     NOT NULL,
    blue_dmg        INTEGER                     NOT NULL,
    red_ubers       INTEGER                     NOT NULL,
    blue_ubers      INTEGER                     NOT NULL
);

CREATE INDEX rounds_log_id_idx
    ON rounds USING BTREE (log_id);

CREATE UNIQUE INDEX rounds_round_log_id_idx
    ON rounds USING BTREE (round, log_id);

CREATE INDEX rounds_winner_idx
    ON rounds USING BTREE (winner);

CREATE INDEX rounds_first_cap_idx
    ON rounds USING BTREE (first_cap);

CREATE TABLE events_charge (
    id              BIGSERIAL                   PRIMARY KEY,
    round_id        INTEGER                     NOT NULL REFERENCES rounds(id),
    medigun         medigun                     NOT NULL,
    time            INTEGER                     NOT NULL,
    team            team                        NOT NULL,
    steam_id        BIGINT                      NOT NULL
);

CREATE INDEX events_charge_round_id_idx
    ON events_charge USING BTREE (round_id);

CREATE INDEX events_charge_steam_id_idx
    ON events_charge USING BTREE (steam_id);

CREATE TABLE events_point_cap (
    id              BIGSERIAL                   PRIMARY KEY,
    round_id        INTEGER                     NOT NULL REFERENCES rounds(id),
    time            INTEGER                     NOT NULL,
    team            team                        NOT NULL,
    point           INTEGER                     NOT NULL
);

CREATE INDEX events_point_cap_round_id_idx
    ON events_point_cap USING BTREE (round_id);

CREATE TABLE events_medic_death (
    id              BIGSERIAL                   PRIMARY KEY,
    round_id        INTEGER                     NOT NULL REFERENCES rounds(id),
    time            INTEGER                     NOT NULL,
    team            team                        NOT NULL,
    steam_id        BIGINT                      NOT NULL,
    killer          BIGINT                      NOT NULL
);

CREATE INDEX events_medic_death_round_id_idx
    ON events_medic_death USING BTREE (round_id);

CREATE INDEX events_medic_death_steam_id_idx
    ON events_medic_death USING BTREE (steam_id);

CREATE TABLE events_drop (
    id              BIGSERIAL                   PRIMARY KEY,
    round_id        INTEGER                     NOT NULL REFERENCES rounds(id),
    time            INTEGER                     NOT NULL,
    team            team                        NOT NULL,
    steam_id        BIGINT                      NOT NULL
);

CREATE INDEX events_drop_round_id_idx
    ON events_drop USING BTREE (round_id);

CREATE INDEX events_drop_steam_id_idx
    ON events_drop USING BTREE (steam_id);

CREATE TABLE events_round_win (
    id              BIGSERIAL                   PRIMARY KEY,
    round_id        INTEGER                     NOT NULL REFERENCES rounds(id),
    time            INTEGER                     NOT NULL,
    team            team                        NOT NULL
);

CREATE UNIQUE INDEX events_round_win_round_id_idx
    ON events_round_win USING BTREE (round_id);

CREATE FUNCTION team_is_winner(log_id INTEGER, team team) RETURNS BOOL AS $$
DECLARE
    is_winner BOOLEAN;
BEGIN
    SELECT team = winner into is_winner FROM logs WHERE id = log_id;
    RETURN is_winner;
END; $$
    LANGUAGE PLPGSQL IMMUTABLE;

CREATE FUNCTION get_game_mode(log_id INTEGER) RETURNS game_mode AS $$
DECLARE
    result game_mode;
BEGIN
    SELECT game_mode into result FROM logs WHERE id = log_id;
    RETURN result;
END; $$
    LANGUAGE PLPGSQL IMMUTABLE;

CREATE FUNCTION get_clean_map(log_id INTEGER) RETURNS TEXT AS $$
DECLARE
    result TEXT;
BEGIN
    SELECT clean_map into result FROM logs WHERE id = log_id;
    RETURN result;
END; $$
    LANGUAGE PLPGSQL IMMUTABLE;

CREATE FUNCTION get_date(log_id INTEGER) RETURNS TIMESTAMP WITHOUT TIME ZONE AS $$
DECLARE
    result TIMESTAMP WITHOUT TIME ZONE;
BEGIN
    SELECT date into result FROM logs WHERE id = log_id;
    RETURN result;
END; $$
    LANGUAGE PLPGSQL IMMUTABLE;

CREATE TABLE players (
    id              BIGSERIAL                   PRIMARY KEY,
    log_id          INTEGER                     NOT NULL REFERENCES logs(id),
    steam_id        BIGINT                      NOT NULL,
    name            TEXT                        NOT NULL,
    team            team                        NOT NULL,
    kills           INTEGER                     NOT NULL,
    deaths          INTEGER                     NOT NULL,
    assists         INTEGER                     NOT NULL,
    suicides        INTEGER                     NOT NULL,
    dmg             INTEGER                     NOT NULL,
    damage_taken    INTEGER                     NOT NULL,
    ubers           INTEGER                     NOT NULL,
    medigun_ubers   INTEGER                     NOT NULL,
    kritzkrieg_ubers INTEGER                    NOT NULL,
    quickfix_ubers  INTEGER                     NOT NULL,
    vaccinator_ubers INTEGER                     NOT NULL,
    drops           INTEGER                     NOT NULL,
    medkits         INTEGER                     NOT NULL,
    medkits_hp      INTEGER                     NOT NULL,
    backstabs       INTEGER                     NOT NULL,
    headshots       INTEGER                     NOT NULL,
    heal            INTEGER                     NOT NULL,
    heals_received  INTEGER                     NOT NULL,
    scout_kills     INTEGER                     NOT NULL,
    soldier_kills   INTEGER                     NOT NULL,
    pyro_kills      INTEGER                     NOT NULL,
    demoman_kills   INTEGER                     NOT NULL,
    heavy_kills     INTEGER                     NOT NULL,
    engineer_kills  INTEGER                     NOT NULL,
    medic_kills     INTEGER                     NOT NULL,
    sniper_kills    INTEGER                     NOT NULL,
    spy_kills       INTEGER                     NOT NULL,
    is_winner       BOOL GENERATED ALWAYS AS (team_is_winner(log_id, team)) STORED,
    game_mode       game_mode GENERATED ALWAYS AS (get_game_mode(log_id)) STORED,
    clean_map       TEXT GENERATED ALWAYS AS (get_clean_map(log_id)) STORED,
    date            TIMESTAMP WITHOUT TIME ZONE GENERATED ALWAYS AS (get_date(log_id)) STORED
);

CREATE INDEX players_log_id_idx
    ON players USING BTREE (log_id);

CREATE UNIQUE INDEX players_log_steam_id_idx
    ON players USING BTREE (log_id, steam_id);

CREATE INDEX players_steam_id_idx
    ON players USING BTREE (steam_id);

CREATE INDEX players_team_idx
    ON players USING BTREE (team);

CREATE INDEX players_is_winner_idx
    ON players USING BTREE (is_winner);

CREATE INDEX players_game_mode_idx
    ON players USING BTREE (game_mode);

CREATE INDEX players_clean_map_idx
    ON players USING BTREE (clean_map);

CREATE INDEX players_date_idx
    ON players USING BTREE (date);

CREATE TABLE class_stats (
    id              BIGSERIAL                   PRIMARY KEY,
    player_id       BIGINT                     NOT NULL REFERENCES players(id),
    type            class_type                  NOT NULL,
    time            INTEGER                     NOT NULL,
    kills           INTEGER                     NOT NULL,
    deaths          INTEGER                     NOT NULL,
    assists         INTEGER                     NOT NULL,
    dmg             INTEGER                     NOT NULL,
);

CREATE INDEX class_stats_player_id_idx
    ON class_stats USING BTREE (player_id);

CREATE UNIQUE INDEX class_stats_player_id_type_idx
    ON class_stats USING BTREE (player_id, type);

CREATE TABLE player_weapon_stats (
    id              BIGSERIAL                   PRIMARY KEY,
    class_stat_id   BIGINT                     NOT NULL REFERENCES class_stats(id),
    weapon          TEXT                        NOT NULL,
    kills           INTEGER                     NOT NULL,
    shots           INTEGER                     NOT NULL,
    hits            INTEGER                     NOT NULL,
    dmg             INTEGER                     NOT NULL
);

CREATE INDEX player_weapon_stats_class_stat_id_idx
    ON player_weapon_stats USING BTREE (class_stat_id);

CREATE UNIQUE INDEX player_weapon_stats_class_stat_id_weapon_idx
    ON player_weapon_stats USING BTREE (class_stat_id, weapon);

CREATE MATERIALIZED VIEW player_stats AS
    SELECT
        game_mode, clean_map, extract(year from date) as year, type as class,
            sum(class_stats.dmg) as damage,
            sum(class_stats.kills) as kills,
            sum(class_stats.deaths) as deaths,
            count(*) as count,
            sum(is_winner::INTEGER) as wins,
            steam_id
        FROM players
        INNER JOIN class_stats ON players.id = class_stats.player_id
        GROUP BY game_mode, clean_map, extract(year from date), type, steam_id;

CREATE INDEX player_stats_steam_id_idx
    ON player_stats USING BTREE (steam_id);

CREATE INDEX player_stats_game_mode_idx
    ON player_stats USING BTREE (game_mode);

CREATE INDEX player_stats_class_idx
    ON player_stats USING BTREE (class);

CREATE INDEX player_stats_year_idx
    ON player_stats USING BTREE (year);

CREATE UNIQUE INDEX player_stats_unique_idx
    ON player_stats USING BTREE (game_mode, clean_map, year, steam_id, class);

CREATE MATERIALIZED VIEW player_names AS
    SELECT
        steam_id, name, sum(length) as TIME, count(*) AS count
    FROM players
    INNER JOIN logs on players.log_id = logs.id
    GROUP BY steam_id, name;

CREATE INDEX player_names_steam_id_idx
    ON player_names USING BTREE (steam_id);

CREATE UNIQUE INDEX player_names_steam_id_name_idx
    ON player_names USING BTREE (steam_id, name);

CREATE INDEX player_names_search_idx
    ON player_names USING GIN (name gin_trgm_ops);

CREATE MATERIALIZED VIEW user_names AS
    WITH names AS
             (
                 select name, count, steam_id,
                        rank() over (partition by steam_id order by steam_id, count desc) rn
                 from player_names
             )
    SELECT steam_id, MAX(name) as name
    FROM names
    WHERE rn = 1
    GROUP BY steam_id;

CREATE UNIQUE INDEX user_names_steam_id_idx
    ON user_names USING BTREE (steam_id);