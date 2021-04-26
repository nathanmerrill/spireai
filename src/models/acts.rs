use crate::models::core::*;
use crate::models::events;
use crate::models::monsters::*;

lazy_static! {
    static ref ACTS: Vec<Act> = vec![
        Act {
            num: 1,
            easy_count: 3,
            easy_fights: vec![
                (1, MonsterSet::Fixed(vec![CULTIST])),
                (1, MonsterSet::Fixed(vec![JAW_WORM])),
                (
                    1,
                    MonsterSet::ChooseN(2, vec![RED_LOUSE, RED_LOUSE, GREEN_LOUSE, GREEN_LOUSE])
                ),
                (
                    1,
                    MonsterSet::RandomSet(vec![
                        vec![ACID_SLIME_M, ACID_SLIME_S],
                        vec![ACID_SLIME_M, SPIKE_SLIME_S],
                        vec![SPIKE_SLIME_M, ACID_SLIME_S],
                        vec![SPIKE_SLIME_M, SPIKE_SLIME_S],
                    ])
                ),
            ],
            normal_fights: vec![
                (
                    2,
                    MonsterSet::ChooseN(
                        4,
                        vec![
                            MAD_GREMLIN,
                            MAD_GREMLIN,
                            SNEAKY_GREMLIN,
                            SNEAKY_GREMLIN,
                            FAT_GREMLIN,
                            FAT_GREMLIN,
                            GREMLIN_WIZARD,
                            SHIELD_GREMLIN
                        ]
                    )
                ),
                (4, MonsterSet::ChooseN(1, vec![ACID_SLIME_L, SPIKE_SLIME_L])),
                (
                    2,
                    MonsterSet::Fixed(vec![
                        SPIKE_SLIME_S,
                        SPIKE_SLIME_S,
                        SPIKE_SLIME_S,
                        ACID_SLIME_S,
                        ACID_SLIME_S,
                    ])
                ),
                (4, MonsterSet::Fixed(vec![BLUE_SLAVER])),
                (2, MonsterSet::Fixed(vec![RED_SLAVER])),
                (
                    4,
                    MonsterSet::ChooseN(
                        3,
                        vec![
                            RED_LOUSE,
                            RED_LOUSE,
                            RED_LOUSE,
                            GREEN_LOUSE,
                            GREEN_LOUSE,
                            GREEN_LOUSE,
                        ]
                    )
                ),
                (4, MonsterSet::Fixed(vec![FUNGI_BEAST, FUNGI_BEAST])),
                (
                    3,
                    MonsterSet::RandomSet(vec![
                        vec![RED_LOUSE, LOOTER],
                        vec![GREEN_LOUSE, LOOTER],
                        vec![ACID_SLIME_M, LOOTER],
                        vec![SPIKE_SLIME_M, LOOTER],
                        vec![RED_LOUSE, CULTIST],
                        vec![GREEN_LOUSE, CULTIST],
                        vec![ACID_SLIME_M, CULTIST],
                        vec![SPIKE_SLIME_M, CULTIST],
                        vec![RED_LOUSE, RED_SLAVER],
                        vec![GREEN_LOUSE, RED_SLAVER],
                        vec![ACID_SLIME_M, RED_SLAVER],
                        vec![SPIKE_SLIME_M, RED_SLAVER],
                        vec![RED_LOUSE, BLUE_SLAVER],
                        vec![GREEN_LOUSE, BLUE_SLAVER],
                        vec![ACID_SLIME_M, BLUE_SLAVER],
                        vec![SPIKE_SLIME_M, BLUE_SLAVER],
                    ])
                ),
                (
                    3,
                    MonsterSet::RandomSet(vec![
                        vec![FUNGI_BEAST, RED_LOUSE],
                        vec![JAW_WORM, RED_LOUSE],
                        vec![FUNGI_BEAST, GREEN_LOUSE],
                        vec![JAW_WORM, GREEN_LOUSE],
                        vec![FUNGI_BEAST, ACID_SLIME_M],
                        vec![JAW_WORM, ACID_SLIME_M],
                        vec![FUNGI_BEAST, SPIKE_SLIME_M],
                        vec![JAW_WORM, SPIKE_SLIME_M],
                    ])
                ),
                (4, MonsterSet::Fixed(vec![LOOTER])),
            ],
            elites: vec![
                MonsterSet::Fixed(vec![GREMLIN_NOB]),
                MonsterSet::Fixed(vec![LAGAVULIN]),
                MonsterSet::Fixed(vec![SENTRY, SENTRY, SENTRY]),
            ],
            bosses: vec![
                MonsterSet::Fixed(vec![SLIME_BOSS]),
                MonsterSet::Fixed(vec![THE_GUARDIAN]),
                MonsterSet::Fixed(vec![HEXAGHOST]),
            ],
            events: vec![
                events::BIG_FISH,
                events::THE_CLERIC,
                events::DEAD_ADVENTURER,
                events::GOLDEN_IDOL,
                events::LIVING_WALL,
                events::MUSHROOMS,
                events::SCRAP_OOZE,
                events::SHINING_LIGHT,
                events::THE_SSSSSERPENT,
                events::WING_STATUE,
                events::WORLD_OF_GOOP,
                events::BONFIRE_SPIRITS,
                events::THE_DIVINE_FOUNTAIN,
                events::FACE_TRADER,
                events::GOLDEN_SHRINE,
                events::LAB,
                events::MATCH_AND_KEEP,
                events::A_NOTE_FOR_YOURSELF,
                events::OMINOUS_FORGE,
                events::PURIFIER,
                events::TRANSMOGRIFIER,
                events::UPGRADE_SHRINE,
                events::WE_MEET_AGAIN,
                events::WHEEL_OF_CHANGE,
                events::THE_WOMAN_IN_BLUE,
            ],
        },
        Act {
            num: 2,
            easy_count: 2,
            easy_fights: vec![
                (1, MonsterSet::Fixed(vec![SPHERIC_GUARDIAN])),
                (1, MonsterSet::Fixed(vec![CHOSEN])),
                (1, MonsterSet::Fixed(vec![SHELLED_PARASITE])),
                (1, MonsterSet::Fixed(vec![BYRD, BYRD, BYRD])),
                (1, MonsterSet::Fixed(vec![LOOTER, MUGGER])),
            ],
            normal_fights: vec![
                (7, MonsterSet::Fixed(vec![CHOSEN, BYRD])),
                (10, MonsterSet::Fixed(vec![CULTIST, CHOSEN])),
                (7, MonsterSet::Fixed(vec![SENTRY, SPHERIC_GUARDIAN])),
                (21, MonsterSet::Fixed(vec![SNAKE_PLANT])),
                (14, MonsterSet::Fixed(vec![SNECKO])),
                (21, MonsterSet::Fixed(vec![CENTURION, MYSTIC])),
                (10, MonsterSet::Fixed(vec![CULTIST, CULTIST, CULTIST])),
                (10, MonsterSet::Fixed(vec![SHELLED_PARASITE, FUNGI_BEAST])),
            ],
            elites: vec![
                MonsterSet::Fixed(vec![BOOK_OF_STABBING]),
                MonsterSet::RandomSet(vec![
                    vec![GREMLIN_LEADER, FAT_GREMLIN, FAT_GREMLIN],
                    vec![GREMLIN_LEADER, FAT_GREMLIN, MAD_GREMLIN],
                    vec![GREMLIN_LEADER, FAT_GREMLIN, SHIELD_GREMLIN],
                    vec![GREMLIN_LEADER, FAT_GREMLIN, SNEAKY_GREMLIN],
                    vec![GREMLIN_LEADER, FAT_GREMLIN, GREMLIN_WIZARD],
                    vec![GREMLIN_LEADER, MAD_GREMLIN, FAT_GREMLIN],
                    vec![GREMLIN_LEADER, MAD_GREMLIN, MAD_GREMLIN],
                    vec![GREMLIN_LEADER, MAD_GREMLIN, SHIELD_GREMLIN],
                    vec![GREMLIN_LEADER, MAD_GREMLIN, SNEAKY_GREMLIN],
                    vec![GREMLIN_LEADER, MAD_GREMLIN, GREMLIN_WIZARD],
                    vec![GREMLIN_LEADER, SHIELD_GREMLIN, FAT_GREMLIN],
                    vec![GREMLIN_LEADER, SHIELD_GREMLIN, MAD_GREMLIN],
                    vec![GREMLIN_LEADER, SHIELD_GREMLIN, SHIELD_GREMLIN],
                    vec![GREMLIN_LEADER, SHIELD_GREMLIN, SNEAKY_GREMLIN],
                    vec![GREMLIN_LEADER, SHIELD_GREMLIN, GREMLIN_WIZARD],
                    vec![GREMLIN_LEADER, SNEAKY_GREMLIN, FAT_GREMLIN],
                    vec![GREMLIN_LEADER, SNEAKY_GREMLIN, MAD_GREMLIN],
                    vec![GREMLIN_LEADER, SNEAKY_GREMLIN, SHIELD_GREMLIN],
                    vec![GREMLIN_LEADER, SNEAKY_GREMLIN, SNEAKY_GREMLIN],
                    vec![GREMLIN_LEADER, SNEAKY_GREMLIN, GREMLIN_WIZARD],
                    vec![GREMLIN_LEADER, GREMLIN_WIZARD, FAT_GREMLIN],
                    vec![GREMLIN_LEADER, GREMLIN_WIZARD, MAD_GREMLIN],
                    vec![GREMLIN_LEADER, GREMLIN_WIZARD, SHIELD_GREMLIN],
                    vec![GREMLIN_LEADER, GREMLIN_WIZARD, SNEAKY_GREMLIN],
                    vec![GREMLIN_LEADER, GREMLIN_WIZARD, GREMLIN_WIZARD],
                ]),
                MonsterSet::Fixed(vec![BLUE_SLAVER, TASKMASTER, RED_SLAVER]),
            ],
            bosses: vec![
                MonsterSet::Fixed(vec![BRONZE_AUTOMATON]),
                MonsterSet::Fixed(vec![THE_CHAMP]),
                MonsterSet::Fixed(vec![THE_COLLECTOR]),
            ],
            events: vec![
                events::ANCIENT_WRITING,
                events::AUGMENTER,
                events::THE_COLOSSEUM,
                events::COUNCIL_OF_GHOSTS,
                events::CURSED_TOME,
                events::FORGOTTEN_ALTAR,
                events::THE_LIBRARY,
                events::MASKED_BANDITS,
                events::THE_MAUSOLEUM,
                events::THE_NEST,
                events::PLEADING_VAGRANT,
                events::OLD_BEGGAR,
                events::VAMPIRES,
                events::BONFIRE_SPIRITS,
                events::THE_DIVINE_FOUNTAIN,
                events::DESIGNER_INSPIRE,
                events::DUPLICATOR,
                events::FACE_TRADER,
                events::GOLDEN_SHRINE,
                events::THE_JOUST,
                events::KNOWING_SKULL,
                events::LAB,
                events::MATCH_AND_KEEP,
                events::A_NOTE_FOR_YOURSELF,
                events::NLOTH,
                events::OMINOUS_FORGE,
                events::PURIFIER,
                events::TRANSMOGRIFIER,
                events::UPGRADE_SHRINE,
                events::WE_MEET_AGAIN,
                events::WHEEL_OF_CHANGE,
                events::THE_WOMAN_IN_BLUE
            ],
        },
        Act {
            num: 3,
            easy_count: 2,
            easy_fights: vec![
                (1, MonsterSet::Fixed(vec![DARKLING, DARKLING, DARKLING])),
                (1, MonsterSet::Fixed(vec![ORB_WALKER])),
                (
                    1,
                    MonsterSet::ChooseN(
                        3,
                        vec![REPULSOR, REPULSOR, SPIKER, SPIKER, EXPLODER, EXPLODER]
                    )
                ),
            ],
            normal_fights: vec![
                (
                    1,
                    MonsterSet::ChooseN(
                        4,
                        vec![REPULSOR, REPULSOR, SPIKER, SPIKER, EXPLODER, EXPLODER]
                    )
                ),
                (1, MonsterSet::Fixed(vec![THE_MAW])),
                (
                    1,
                    MonsterSet::RandomSet(vec![
                        vec![SPHERIC_GUARDIAN, REPULSOR, REPULSOR],
                        vec![SPHERIC_GUARDIAN, REPULSOR, SPIKER],
                        vec![SPHERIC_GUARDIAN, REPULSOR, EXPLODER],
                        vec![SPHERIC_GUARDIAN, SPIKER, REPULSOR],
                        vec![SPHERIC_GUARDIAN, SPIKER, SPIKER],
                        vec![SPHERIC_GUARDIAN, SPIKER, EXPLODER],
                        vec![SPHERIC_GUARDIAN, EXPLODER, REPULSOR],
                        vec![SPHERIC_GUARDIAN, EXPLODER, SPIKER],
                        vec![SPHERIC_GUARDIAN, EXPLODER, EXPLODER],
                    ])
                ),
                (1, MonsterSet::Fixed(vec![DARKLING, DARKLING, DARKLING])),
                (1, MonsterSet::Fixed(vec![WRITHING_MASS])),
                (1, MonsterSet::Fixed(vec![JAW_WORM, JAW_WORM, JAW_WORM])),
                (1, MonsterSet::Fixed(vec![SPIRE_GROWTH])),
                (1, MonsterSet::Fixed(vec![TRANSIENT])),
            ],
            elites: vec![
                MonsterSet::Fixed(vec![GIANT_HEAD]),
                MonsterSet::Fixed(vec![NEMESIS]),
                MonsterSet::Fixed(vec![REPTOMANCER, DAGGER, DAGGER]),
            ],
            bosses: vec![
                MonsterSet::Fixed(vec![AWAKENED_ONE]),
                MonsterSet::Fixed(vec![TIME_EATER]),
                MonsterSet::Fixed(vec![DECA, DONU]),
            ],
            events: vec![
                events::FALLING,
                events::MIND_BLOOM,
                events::THE_MOAI_HEAD,
                events::MYSTERIOUS_SPHERE,
                events::SENSORY_STONE,
                events::TOMB_OF_LORD_RED_MASK,
                events::WINDING_HALLS,
                events::BONFIRE_SPIRITS,
                events::DESIGNER_INSPIRE,
                events::THE_DIVINE_FOUNTAIN,
                events::DUPLICATOR,
                events::GOLDEN_SHRINE,
                events::LAB,
                events::MATCH_AND_KEEP,
                events::A_NOTE_FOR_YOURSELF,
                events::OMINOUS_FORGE,
                events::PURIFIER,
                events::SECRET_PORTAL,
                events::TRANSMOGRIFIER,
                events::UPGRADE_SHRINE,
                events::WE_MEET_AGAIN,
                events::WHEEL_OF_CHANGE,
                events::THE_WOMAN_IN_BLUE
            ],
        },
        Act {
            num: 4,
            easy_count: 0,
            easy_fights: vec![],
            normal_fights: vec![],
            elites: vec![MonsterSet::Fixed(vec![SPIRE_SHIELD, SPIRE_SPEAR]),],
            bosses: vec![MonsterSet::Fixed(vec![CORRUPT_HEART])],
            events: vec![],
        }
    ];
}
