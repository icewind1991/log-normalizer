CREATE EXTENSION IF NOT EXISTS pg_trgm WITH SCHEMA public;

CREATE TYPE team AS ENUM ('Blue', 'Red');

CREATE TYPE class_type AS ENUM ('scout', 'soldier', 'pyro', 'demoman', 'heavyweapons', 'engineer', 'medic', 'sniper', 'spy');

CREATE TYPE game_mode AS ENUM ('ultiduo', '4v4', '6v6', '7v7', '9v9', 'other');

CREATE TYPE event_type AS ENUM ('charge', 'pointcap', 'medic_death', 'round_win');

CREATE TYPE medigun AS ENUM ('medigun', 'kritzkrieg', 'quickfix', 'vacinator');

CREATE TYPE weapon_id AS ENUM (
    'sniperrifle',
    'taunt_sniper',
    'sydney_sleeper',
    'the_winger',
    'hot_hand',
    'deflect_rocket',
    'scout_sword',
    'voodoo_pin',
    'degreaser',
    'shortstop',
    'robot_arm',
    'tf_pumpkin_bomb',
    'kunai',
    'wrench',
    'necro_smasher',
    'headtaker',
    'proto_syringe',
    'eternal_reward',
    'eureka_effect',
    'dragons_fury_bonus',
    'grappling_hook',
    'pep_pistol',
    'guillotine',
    'lava_axe',
    'wrangler_kill',
    'panic_attack',
    'loose_cannon',
    'thirddegree',
    'taunt_pyro',
    'iron_bomber',
    'persian_persuader',
    'amputator',
    'tf_projectile_pipe',
    'awper_hand',
    'demokatana',
    'loose_cannon_reflect',
    'obj_minisentry',
    'back_scratcher',
    'pomson',
    'the_classic',
    'sandman',
    'axtinguisher',
    'obj_sentrygun2',
    'jar',
    'samrevolver',
    'deflect_flare',
    'loch_n_load',
    'tf_projectile_rocket',
    'annihilator',
    'rocketpack_stomp',
    'ullapool_caber_explosion',
    'minigun',
    'deflect_promode',
    'liberty_launcher',
    'nessieclub',
    'quickiebomb_launcher',
    'cow_mangler',
    'wrap_assassin',
    'blackbox',
    'candy_cane',
    'rocketlauncher_directhit',
    'ai_flamethrower',
    'soda_popper',
    'frontier_justice',
    'rescue_ranger_reflect',
    'warrior_spirit',
    'widowmaker',
    'quake_rl',
    'tf_projectile_arrow',
    'sledgehammer',
    'deflect_arrow',
    'market_gardener',
    'unique_pickaxe_escape',
    'pistol_scout',
    'boston_basher',
    'spellbook_bats',
    'holymackerel',
    'club',
    'ball',
    'fireaxe',
    'backburner',
    'eviction_notice',
    'holiday_punch',
    'taunt_soldier',
    'shotgun_primary',
    'nonnonviolent_protest',
    'force_a_nature',
    'paintrain',
    'freedom_staff',
    'shotgun_hwg',
    'long_heatmaker',
    'shotgun_soldier',
    'knife',
    'batsaber',
    'mailbox',
    'compound_bow',
    'ullapool_caber',
    'ambassador',
    'player_penetration',
    'powerjack',
    'crusaders_crossbow',
    'scotland_shard',
    'wrench_jag',
    'scattergun',
    'unknown',
    'shahanshah',
    'pistol',
    'spellbook_boss',
    'smg',
    'dragons_fury',
    'revolver',
    'player',
    'the_maul',
    'skullbat',
    'ham_shank',
    'solemn_vow',
    'iron_curtain',
    'bonesaw',
    'dumpster_device',
    'bushwacka',
    'builder',
    'bread_bite',
    'southern_hospitality',
    'tribalkukri',
    'the_capper',
    'fists',
    'disciplinary_action',
    'tf_projectile_flare',
    'bleed_kill',
    'black_rose',
    'letranger',
    'tomislav',
    'atomizer',
    'back_scatter',
    'pep_brawlerblaster',
    'tf_projectile_pipe_remote',
    'battleaxe',
    'deflect_flare_detonator',
    'taunt_medic',
    'telefrag',
    'stickybomb_defender',
    'splendid_screen',
    'claidheamohmor',
    'airstrike',
    'righteous_bison',
    'gloves_running_urgently',
    'sword',
    'mantreads',
    'deflect_sticky',
    'enforcer',
    'scorch_shot',
    'pro_rifle',
    'spy_cicle',
    'bat',
    'sharp_dresser',
    'spellbook_lightning',
    'tide_turner',
    'shotgun_pyro',
    'lava_bat',
    'taunt_heavy',
    'bottle',
    'unique_pickaxe',
    'phlogistinator',
    'crossing_guard',
    'giger_counter',
    'charged_smg',
    'syringegun_medic',
    'gloves',
    'bazaar_bargain',
    'spellbook_mirv',
    'big_earner',
    'battleneedle',
    'warfan',
    'obj_sentrygun',
    'manmelter',
    'family_business',
    'reserve_shooter',
    'short_circuit',
    'flaregun',
    'spellbook_fireball',
    'world',
    'jar_milk',
    'flamethrower',
    'shooting_star',
    'trigger_hurt',
    'blutsauger',
    'taunt_spy',
    'tf_projectile_energy_ball',
    'jar_gas',
    'tf_projectile_mechanicalarmorb',
    'obj_sentrygun3',
    'diamondback',
    'shovel',
    'brass_beast',
    'loose_cannon_impact',
    'demoshield',
    'prinny_machete',
    'machina',
    'rocketlauncher_fireball',
    'sticky_resistance',
    'detonator',
    'tf_projectile_sentryrocket',
    'unarmed_combat',
    'spellbook_skeleton',
    'ubersaw',
    'maxgun',
    'robot_arm_combo_kill',
    'rescue_ranger',
    'apocofists',
    'natascha',
    'pro_smg',
    'steel_fists',
    'fryingpan',
);

CREATE TABLE logs (
    id              INTEGER                     PRIMARY KEY,
    red_score       INTEGER                     NOT NULL,
    blue_score      INTEGER                     NOT NULL,
    length          INTEGER                     NOT NULL,
    game_mode       game_mode                   NOT NULL,
    map             TEXT                        NOT NULL,
    date            TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE INDEX logs_map_idx
    ON logs USING BTREE (map);

CREATE INDEX logs_mode_idx
    ON logs USING BTREE (game_mode);

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
    id              SERIAL                      PRIMARY KEY,
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
    id              SERIAL                      PRIMARY KEY,
    round_id        INTEGER                     NOT NULL REFERENCES rounds(id),
    time            INTEGER                     NOT NULL,
    team            team                        NOT NULL,
    point           INTEGER                     NOT NULL
);

CREATE INDEX events_point_cap_round_id_idx
    ON events_point_cap USING BTREE (round_id);

CREATE TABLE events_medic_death (
    id              SERIAL                      PRIMARY KEY,
    round_id        INTEGER                     NOT NULL REFERENCES rounds(id),
    time            INTEGER                     NOT NULL,
    team            team                        NOT NULL,
    steam_id        BIGINT                      NOT NULL,
    killer          BIGINT                      NOT NULL
);

CREATE INDEX events_medic_death_round_id_idx
    ON events_medic_death USING BTREE (round_id);

CREATE TABLE events_round_win (
    id              SERIAL                      PRIMARY KEY,
    round_id        INTEGER                     NOT NULL REFERENCES rounds(id),
    time            INTEGER                     NOT NULL,
    team            team                        NOT NULL
);

CREATE UNIQUE INDEX events_round_win_round_id_idx
    ON events_round_win USING BTREE (round_id);

CREATE TABLE players (
    id              SERIAL                      PRIMARY KEY,
    log_id          INTEGER                     NOT NULL REFERENCES logs(id),
    steam_id        BIGINT                      NOT NULL,
    name            TEXT                        NOT NULL,
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
    vacinator_ubers INTEGER                     NOT NULL,
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
    spy_kills       INTEGER                     NOT NULL
);

CREATE INDEX players_log_id_idx
    ON players USING BTREE (log_id);

CREATE UNIQUE INDEX players_log_steam_id_idx
    ON players USING BTREE (log_id, steam_id);

CREATE INDEX players_steam_id_idx
    ON players USING BTREE (steam_id);

CREATE TABLE class_stats (
    id              SERIAL                      PRIMARY KEY,
    player_id       INTEGER                     NOT NULL REFERENCES players(id),
    type            class_type                  NOT NULL,
    time            INTEGER                     NOT NULL,
    kills           INTEGER                     NOT NULL,
    deaths          INTEGER                     NOT NULL,
    assists         INTEGER                     NOT NULL,
    dmg             INTEGER                     NOT NULL
);

CREATE INDEX class_stats_player_id_idx
    ON class_stats USING BTREE (player_id);

CREATE UNIQUE INDEX class_stats_player_id_type_idx
    ON class_stats USING BTREE (player_id, type);

CREATE TABLE player_weapon_stats (
    id              SERIAL                      PRIMARY KEY,
    class_stat_id   INTEGER                     NOT NULL REFERENCES class_stats(id),
    weapon          weapon_id                   NOT NULL,
    kills           INTEGER                     NOT NULL,
    shots           INTEGER                     NOT NULL,
    hits            INTEGER                     NOT NULL,
    dmg             INTEGER                     NOT NULL
);

CREATE INDEX player_weapon_stats_class_stat_id_idx
    ON player_weapon_stats USING BTREE (class_stat_id);

CREATE UNIQUE INDEX player_weapon_stats_class_stat_id_weapon_idx
    ON player_weapon_stats USING BTREE (class_stat_id, weapon);