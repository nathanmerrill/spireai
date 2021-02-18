use crate::models::effects::*;
use std::collections::HashMap;
use crate::models::{GameState};
use crate::spireai::calculator;
use calculator::{GamePossibilitySet};
use crate::models::cards::CardType; 
use crate::models::cards; 


pub struct Status {
    pub name: &'static str,
    pub is_additive: bool,
    pub is_buff: bool,
    pub multiply_effect: bool,
    pub reduce_at: StatusEvent,
    pub expire_at: StatusEvent,
    pub effect_at: StatusEvent,
    pub effect: Vec<Effect>,
}

pub enum StatusEvent {
    TurnStart,
    TurnEnd,
    OnAttackDamage,
    OnUnblockedDamage,
    OnHpLoss,
    Never,
    OnCard(CardType),
}

impl Default for Status {
    fn default() -> Self {
        Self {
            name: &"",
            is_additive: true,
            is_buff: true,
            multiply_effect: true,
            reduce_at: StatusEvent::Never,
            expire_at: StatusEvent::Never,
            effect_at: StatusEvent::Never,
            effect: Vec::new(),
        }
    }
}

pub const statuses: Vec<Status> = vec![
    Status {
        name: ARTIFACT,
        ..Status::default()
    },
    Status {
        name: BARRICADE,
        is_additive: false,
        ..Status::default()
    },
    Status {
        name: BUFFER,
        ..Status::default()
    },
    Status {
        name: DEXTERITY,
        ..Status::default()
    },
    Status {
        name: DRAW_CARD,
        effect_at: StatusEvent::TurnStart,
        expire_at: StatusEvent::TurnStart,
        effect: vec![Effect::Draw(1)],
        ..Status::default()
    },
    Status {
        name: ENERGIZED,
        effect_at: StatusEvent::TurnStart,
        expire_at: StatusEvent::TurnStart,
        effect: vec![Effect::AddEnergy(1)],
        ..Status::default()
    },
    Status {
        name: FOCUS,
        ..Status::default()
    },
    Status {
        name: INTANGIBLE,
        reduce_at: StatusEvent::TurnStart,
        ..Status::default()
    },
    Status {
        name: MANTRA,
        ..Status::default()
    },
    Status {
        name: METALLICIZE,
        effect_at: StatusEvent::TurnStart,
        effect: vec![Effect::Block(1, EffectTarget::_Self)],
        ..Status::default()
    },
    Status {
        name: NEXT_TURN_BLOCK,
        effect_at: StatusEvent::TurnStart,
        expire_at: StatusEvent::TurnStart,
        effect: vec![Effect::Block(1, EffectTarget::_Self)],
        ..Status::default()
    },
    Status {
        name: PLATED_ARMOR,
        effect_at: StatusEvent::TurnStart,
        reduce_at: StatusEvent::OnUnblockedDamage,
        effect: vec![Effect::Block(1, EffectTarget::_Self)],
        ..Status::default()
    },
    Status {
        name: RITUAL,
        effect_at: StatusEvent::TurnEnd,
        effect: vec![Effect::SetStatus(STRENGTH, 1, EffectTarget::_Self)],
        ..Status::default()
    },
    Status {
        name: STRENGTH,
        ..Status::default()
    },
    Status {
        name: THORNS,
        effect_at: StatusEvent::OnAttackDamage,
        effect: vec![Effect::Damage(1, EffectTarget::Attacker)],
        ..Status::default()
    },
    Status {
        name: VIGOR,
        ..Status::default()
    },
    Status {
        name: ACCURACY,
        ..Status::default()
    },
    Status {
        name: AFTER_IMAGE,
        effect_at: StatusEvent::OnCard(CardType::All),
        effect: vec![Effect::Block(1, EffectTarget::_Self)],
        ..Status::default()
    },
    Status {
        name: AMPLIFY,
        expire_at: StatusEvent::TurnEnd,
        ..Status::default()
    },
    Status {
        name: BATTLE_HYMN,
        effect_at: StatusEvent::TurnStart,
        effect: vec![Effect::AddCard{
            card: CardReference::ByName(cards::SMITE),
            destination: CardLocation::PlayerHand(RelativePosition::Top), 
            copies: 1,
            modifier: CardModifier::None,
        }],
        ..Status::default()
    },
    Status {
        name: BERSERK,
        effect_at: StatusEvent::TurnStart,
        effect: vec![Effect::AddEnergy(1)),
        ..Status::default()
    },
    Status {
        name: BLASPHEMER,
        is_additive: false,
        effect_at: StatusEvent::TurnStart,
        expire_at: StatusEvent::TurnStart,
        effect: vec![Effect::Damage(9999, EffectTarget::_Self)],
        ..Status::default()
    },
    Status {
        name: BLUR,
        reduce_at: StatusEvent::TurnStart,
        ..Status::default()
    },
    Status {
        name: BRUTALITY,
        effect_at: StatusEvent::TurnStart,
        effect: vec![
            Effect::LoseHp(1, EffectTarget::_Self),
            Effect::Draw(1),
        ],
        ..Status::default()
    },
    Status {
        name: COLLECT,
        effect_at: StatusEvent::TurnStart,
        effect: vec![Effect::AddCard{
            card: CardReference::ByName(cards::COLLECT),
            destination: CardLocation::PlayerHand(RelativePosition::Top), 
            copies: 1,
            modifier: CardModifier::Upgraded,
        }],
        ..Status::default()
    },
    Status {
        name: COMBUST,
        effect_at: StatusEvent::TurnStart,
        effect: vec![Effect::AddCard{
            card: CardReference::ByName(cards::COLLECT),
            destination: CardLocation::PlayerHand(RelativePosition::Top), 
            copies: 1,
            modifier: CardModifier::Upgraded,
        }],
        ..Status::default()
    },
];

pub const ACCURACY: &str = "Accuracy";
pub const AFTER_IMAGE: &str = "After Image";
pub const AMPLIFY: &str = "Amplify";
pub const ARTIFACT: &str = "Artifact";
pub const BARRICADE: &str = "Barricade";
pub const BATTLE_HYMN: &str = "Battle Hymn";
pub const BIAS: &str = "Bias";
pub const BERSERK: &str = "Berserk";
pub const BLASPHEMER: &str = "Blasphemer";
pub const BLOCK_RETURN: &str = "Block Return";
pub const BLUR: &str = "Blur";
pub const BRUTALITY: &str = "Brutality";
pub const BUFFER: &str = "Buffer";
pub const BURST: &str = "Burst";
pub const CHOKED: &str = "Choked";
pub const COLLECT: &str = "Collect";
pub const CONFUSED: &str = "Confused";
pub const CONSTRICTED: &str = "Constricted";
pub const COMBUST: &str = "Combust";
pub const CORRUPTION: &str = "Corruption";
pub const CORPSE_EXPLOSION: &str = "Corpse Explosion";
pub const CREATIVE_AI: &str = "Creative AI";
pub const DARK_EMBRANCE: &str = "Dark Embrace";
pub const DEMON_FORM: &str = "Demon Form";
pub const DEVA: &str = "Deva";
pub const DEVOTION: &str = "Devotion";
pub const DEXTERITY: &str = "Dexterity";
pub const DEXTERITY_DOWN: &str = "Dexterity Down";
pub const DOUBLE_DAMAGE: &str = "Double Damage";
pub const DOUBLE_TAP: &str = "Double Tap";
pub const DRAW_CARD: &str = "Draw Card";
pub const DRAW_REDUCTION: &str = "Draw Reduction";
pub const DUPLICATION: &str = "Duplication";
pub const ECHO_FORM: &str = "Echo Form";
pub const ENERGIZED: &str = "Energized";
pub const ELECTRO: &str = "Electro";
pub const ENTANGLED: &str = "Entangled";
pub const ENVENOM: &str = "Envenom";
pub const EQUILIBRIUM: &str = "Equilibrium";
pub const ESTABLISHMENT: &str = "Establishment";
pub const EVOLVE: &str = "Evolve";
pub const FASTING: &str = "Fasting";
pub const FEEL_NO_PAIN: &str = "Feel No Pain";
pub const FIRE_BREATHING: &str = "Fire Breathing";
pub const FLAME_BARRIER: &str = "Flame Barrier";
pub const FOCUS: &str = "Focus";
pub const FORESIGHT: &str = "Foresight";
pub const FRAIL: &str = "Frail";
pub const FREE_ATTACK_POWER: &str = "Free Attack Power";
pub const HEATSINK: &str = "Heatsink";
pub const HELLO: &str = "Hello";
pub const HEX: &str = "Hex";
pub const INFINITE_BLADES: &str = "Infinite Blades";
pub const INTANGIBLE: &str = "Intangible";
pub const JUGGERNAUT: &str = "Juggernaut";
pub const LIKE_WATER: &str = "Like Water";
pub const LOCK_ON: &str = "Lock-On";
pub const LOOP: &str = "Loop";
pub const MACHINE_LEARNING: &str = "Machine Learning";
pub const MAGNETISM: &str = "Magnetism";
pub const MANTRA: &str = "Mantra";
pub const MARK: &str = "Mark";
pub const MASTER_REALITY: &str = "Master Reality";
pub const MAYHEM: &str = "Mayhem";
pub const MENTAL_FORTRESS: &str = "Mental Fortress";
pub const METALLICIZE: &str = "Metallicize";
pub const NEXT_TURN_BLOCK: &str = "Next Turn Block";
pub const NIGHTMARE: &str = "Nightmare";
pub const NIRVANA: &str = "Nirvana";
pub const NO_BLOCK: &str = "No Block";
pub const NO_DRAW: &str = "No Draw";
pub const NOXIOUS_FUMES: &str = "Noxious Fumes";
pub const OMEGA: &str = "Omega";
pub const PANACE: &str = "Panache";
pub const PEN_NIB: &str = "Pen Nib";
pub const PLATED_ARMOR: &str = "Plated Armor";
pub const PHANTASMAL: &str = "Phantasmal";
pub const POISON: &str = "Poison";
pub const RAGE: &str = "Rage";
pub const REBOUND: &str = "Rebound";
pub const REGEN: &str = "Regen";
pub const REPAIR: &str = "Repair";
pub const RITUAL: &str = "Ritual";
pub const RUSHDOWN: &str = "Rushdown";
pub const RUPTURE: &str = "Rupture";
pub const SADISTIC: &str = "Sadistic";
pub const SHACKLED: &str = "Shackled";
pub const SIMMERING_RAGE: &str = "Simmering Rage";
pub const SLOW: &str = "Slow";
pub const STATIC_DISCHARGE: &str = "Static Discharge";
pub const STORM: &str = "Storm";
pub const STRENGTH: &str = "Strength";
pub const STRENGTH_DOWN: &str = "Strength Down";
pub const STUDY: &str = "Study";
pub const SURROUNDED: &str = "Surrounded";
pub const THE_BOMB: &str = "The Bomb";
pub const THORNS: &str = "Thorns";
pub const THOUSAND_CUTS: &str = "Thousand Cuts";
pub const TOOLS_OF_THE_TRADE: &str = "Tools of the Trade";
pub const VIGOR: &str = "Vigor";
pub const VULNERABLE: &str = "Vulnerable";
pub const WAVE_OF_THE_HAND: &str = "Wave of the Hand";
pub const WEAK: &str = "Weak";
pub const WELL_LAID_PLANS: &str = "Well-Laid Plans";
pub const WRAITH_FORM: &str = "Wraith Form";