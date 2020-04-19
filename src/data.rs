use serde::Deserialize;

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Eq, PartialEq)]
pub enum TeamId {
    Blue,
    Red,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Eq, PartialEq)]
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
}

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Eq, PartialEq)]
pub enum GameMode {
    UltiDuo,
    Fours,
    Sixes,
    Sevens,
    Highlander,
    Other,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Eq, PartialEq)]
pub enum EventType {
    Charge,
    PointCap,
    MedicDeath,
    RoundWin,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Medigun {
    Medigun,
    KritzKrieg,
    QuickFix,
    Vacinator,
}

impl Default for Medigun {
    fn default() -> Self {
        Medigun::Medigun
    }
}

#[derive(Debug, Clone, Copy, sqlx::Type, Deserialize, Hash, Eq, PartialEq)]
// #[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Weapon {
    Sniperrifle,
    TauntSniper,
    SydneySleeper,
    TheWinger,
    HotHand,
    DeflectRocket,
    ScoutSword,
    VoodooPin,
    Degreaser,
    Shortstop,
    RobotArm,
    TfPumpkinBomb,
    Kunai,
    Wrench,
    NecroSmasher,
    Headtaker,
    ProtoSyringe,
    EternalReward,
    EurekaEffect,
    DragonsFuryBonus,
    GrapplingHook,
    PepPistol,
    Guillotine,
    LavaAxe,
    WranglerKill,
    PanicAttack,
    LooseCannon,
    Thirddegree,
    TauntPyro,
    IronBomber,
    PersianPersuader,
    Amputator,
    TfProjectilePipe,
    AwperHand,
    Demokatana,
    LooseCannonReflect,
    ObjMinisentry,
    BackScratcher,
    Pomson,
    TheClassic,
    Sandman,
    Axtinguisher,
    ObjSentrygun2,
    Jar,
    Samrevolver,
    DeflectFlare,
    LochNLoad,
    TfProjectileRocket,
    Annihilator,
    RocketpackStomp,
    UllapoolCaberExplosion,
    Minigun,
    DeflectPromode,
    LibertyLauncher,
    Nessieclub,
    QuickiebombLauncher,
    CowMangler,
    WrapAssassin,
    Blackbox,
    CandyCane,
    RocketlauncherDirecthit,
    AiFlamethrower,
    SodaPopper,
    FrontierJustice,
    RescueRangerReflect,
    WarriorSpirit,
    Widowmaker,
    QuakeRl,
    TfProjectileArrow,
    Sledgehammer,
    DeflectArrow,
    MarketGardener,
    UniquePickaxeEscape,
    PistolScout,
    BostonBasher,
    SpellbookBats,
    Holymackerel,
    Club,
    Ball,
    Fireaxe,
    Backburner,
    EvictionNotice,
    HolidayPunch,
    TauntSoldier,
    ShotgunPrimary,
    NonnonviolentProtest,
    ForceANature,
    Paintrain,
    FreedomStaff,
    ShotgunHwg,
    LongHeatmaker,
    ShotgunSoldier,
    Knife,
    Batsaber,
    Mailbox,
    CompoundBow,
    UllapoolCaber,
    Ambassador,
    PlayerPenetration,
    Powerjack,
    CrusadersCrossbow,
    ScotlandShard,
    WrenchJag,
    Scattergun,
    Unknown,
    Shahanshah,
    Pistol,
    SpellbookBoss,
    Smg,
    DragonsFury,
    Revolver,
    Player,
    TheMaul,
    Skullbat,
    HamShank,
    SolemnVow,
    IronCurtain,
    Bonesaw,
    DumpsterDevice,
    Bushwacka,
    Builder,
    BreadBite,
    SouthernHospitality,
    Tribalkukri,
    TheCapper,
    Fists,
    DisciplinaryAction,
    TfProjectileFlare,
    BleedKill,
    BlackRose,
    Letranger,
    Tomislav,
    Atomizer,
    BackScatter,
    PepBrawlerblaster,
    TfProjectilePipeRemote,
    Battleaxe,
    DeflectFlareDetonator,
    TauntMedic,
    Telefrag,
    StickybombDefender,
    SplendidScreen,
    Claidheamohmor,
    Airstrike,
    RighteousBison,
    GlovesRunningUrgently,
    Sword,
    Mantreads,
    DeflectSticky,
    Enforcer,
    ScorchShot,
    ProRifle,
    SpyCicle,
    Bat,
    SharpDresser,
    SpellbookLightning,
    TideTurner,
    ShotgunPyro,
    LavaBat,
    TauntHeavy,
    Bottle,
    UniquePickaxe,
    Phlogistinator,
    CrossingGuard,
    GigerCounter,
    ChargedSmg,
    SyringegunMedic,
    Gloves,
    BazaarBargain,
    SpellbookMirv,
    BigEarner,
    Battleneedle,
    Warfan,
    ObjSentrygun,
    Manmelter,
    FamilyBusiness,
    ReserveShooter,
    ShortCircuit,
    Flaregun,
    SpellbookFireball,
    World,
    JarMilk,
    Flamethrower,
    ShootingStar,
    TriggerHurt,
    Blutsauger,
    TauntSpy,
    TfProjectileEnergyBall,
    JarGas,
    TfProjectileMechanicalarmorb,
    ObjSentrygun3,
    Diamondback,
    Shovel,
    BrassBeast,
    LooseCannonImpact,
    Demoshield,
    PrinnyMachete,
    Machina,
    RocketlauncherFireball,
    StickyResistance,
    Detonator,
    TfProjectileSentryrocket,
    UnarmedCombat,
    SpellbookSkeleton,
    Ubersaw,
    Maxgun,
    RobotArmComboKill,
    RescueRanger,
    Apocofists,
    Natascha,
    ProSmg,
    SteelFists,
    Fryingpan,
}
