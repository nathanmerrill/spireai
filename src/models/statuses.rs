use crate::models::effects::*;
use crate::models::cards; 
use cards::{CardType, CardRarity}; 


pub struct Status {
    pub name: &'static str,
    pub stacks: bool,
    pub is_additive: bool,
    pub is_buff: bool,
    pub multiply_effect: bool,
    pub reduce_at: Event,
    pub expire_at: Event,
    pub effect_at: Event,
    pub effect: Effect,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            name: &"",
            is_additive: true,
            stacks: true,
            is_buff: true,
            multiply_effect: true,
            reduce_at: Event::Never,
            expire_at: Event::Never,
            effect_at: Event::Never,
            effect: Effect::None,
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
        effect_at: Event::TurnStart,
        expire_at: Event::TurnStart,
        effect: Effect::Draw(1),
        ..Status::default()
    },
    Status {
        name: ENERGIZED,
        effect_at: Event::TurnStart,
        expire_at: Event::TurnStart,
        effect: Effect::AddEnergy(1),
        ..Status::default()
    },
    Status {
        name: FOCUS,
        ..Status::default()
    },
    Status {
        name: INTANGIBLE,
        reduce_at: Event::TurnStart,
        ..Status::default()
    },
    Status {
        name: MANTRA,
        ..Status::default()
    },
    Status {
        name: METALLICIZE,
        effect_at: Event::TurnStart,
        effect: Effect::Block(1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: NEXT_TURN_BLOCK,
        effect_at: Event::TurnStart,
        expire_at: Event::TurnStart,
        effect: Effect::Block(1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: PLATED_ARMOR,
        effect_at: Event::TurnStart,
        reduce_at: Event::OnUnblockedDamage,
        effect: Effect::Block(1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: RITUAL,
        effect_at: Event::TurnEnd,
        effect: Effect::SetStatus(STRENGTH, 1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: STRENGTH,
        ..Status::default()
    },
    Status {
        name: THORNS,
        effect_at: Event::OnAttackDamage,
        effect: Effect::Damage(1, EffectTarget::Attacker),
        ..Status::default()
    },
    Status {
        name: VIGOR,
        expire_at: Event::OnCard(CardType::Attack),
        ..Status::default()
    },
    Status {
        name: ACCURACY,
        ..Status::default()
    },
    Status {
        name: AFTER_IMAGE,
        effect_at: Event::OnCard(CardType::All),
        effect: Effect::Block(1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: AMPLIFY,
        expire_at: Event::TurnEnd,
        ..Status::default()
    },
    Status {
        name: BATTLE_HYMN,
        effect_at: Event::TurnStart,
        effect: Effect::AddCard{
            card: CardReference::ByName(cards::SMITE),
            destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
            copies: 1,
            modifier: CardModifier::None,
        },
        ..Status::default()
    },
    Status {
        name: BERSERK,
        effect_at: Event::TurnStart,
        effect: Effect::AddEnergy(1),
        ..Status::default()
    },
    Status {
        name: BLASPHEMER,
        is_additive: false,
        effect_at: Event::TurnStart,
        expire_at: Event::TurnStart,
        effect: Effect::Damage(9999, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: BLUR,
        reduce_at: Event::TurnStart,
        ..Status::default()
    },
    Status {
        name: BRUTALITY,
        effect_at: Event::TurnStart,
        effect: Effect::Multiple(
            vec![
                Effect::LoseHp(1, EffectTarget::_Self),
                Effect::Draw(1),
            ]),
        ..Status::default()
    },
    Status {
        name: BURST,
        reduce_at: Event::OnCard(CardType::Skill),
        ..Status::default()
    },
    Status {
        name: COLLECT,
        effect_at: Event::TurnStart,
        reduce_at: Event::TurnStart,
        effect: Effect::AddCard{
            card: CardReference::ByName(cards::COLLECT),
            destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
            copies: 1,
            modifier: CardModifier::Upgraded,
        },
        ..Status::default()
    },
    Status {
        name: COMBUST,
        ..Status::default()
    },
    Status {
        name: CORRUPTION,
        is_additive: false,
        ..Status::default()
    },
    Status {
        name: CREATIVE_AI,
        effect_at: Event::TurnStart,
        effect: Effect::AddCard{
            card: CardReference::RandomType(CardType::Power),
            destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
            copies: 1,
            modifier: CardModifier::None,
        },
        ..Status::default()
    },
    Status {
        name: DARK_EMBRACE,
        effect_at: Event::OnExhaust,
        effect: Effect::Draw(1),
        ..Status::default()
    },
    Status {
        name: DEMON_FORM,
        effect_at: Event::TurnStart,
        effect: Effect::SetStatus(STRENGTH, 1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: DEVA,
        ..Status::default()
    },
    Status {
        name: DEVOTION,
        effect_at: Event::TurnStart,
        effect: Effect::AddMantra(1),
        ..Status::default()
    },
    Status {
        name: DOUBLE_DAMAGE,
        reduce_at: Event::TurnEnd,
        ..Status::default()
    },
    Status {
        name: DOUBLE_TAP,
        reduce_at: Event::OnCard(CardType::Attack),
        ..Status::default()
    },
    Status {
        name: DUPLICATION,
        reduce_at: Event::OnCard(CardType::All),
        ..Status::default()
    },
    Status {
        name: ECHO_FORM,
        ..Status::default()
    },
    Status {
        name: ELECTRO,
        is_additive: false,
        ..Status::default()
    },
    Status {
        name: ENVENOM,
        effect_at: Event::OnTargetUnblockedDamage,
        effect: Effect::SetStatus(POISON, 1, EffectTarget::TargetEnemy),
        ..Status::default()
    },
    Status {
        name: EQUILIBRIUM,
        reduce_at: Event::TurnStart,
        ..Status::default()
    },
    Status {
        name: ESTABLISHMENT,
        ..Status::default()
    },
    Status {
        name: EVOLVE,
        effect_at: Event::OnDraw(CardType::Status),
        effect: Effect::Draw(1),
        ..Status::default()
    },
    Status {
        name: FEEL_NO_PAIN,
        effect_at: Event::OnExhaust,
        effect: Effect::Block(1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: FIRE_BREATHING,
        effect_at: Event::Multiple(vec![Event::OnDraw(CardType::Status), Event::OnDraw(CardType::Status)]),
        effect: Effect::Damage(1, EffectTarget::AllEnemies),
        ..Status::default()
    },
    Status {
        name: FORESIGHT,
        effect_at: Event::TurnStart,
        effect: Effect::Scry(1),
        ..Status::default()
    },
    Status {
        name: FREE_ATTACK_POWER,
        reduce_at: Event::OnCard(CardType::Attack),
        ..Status::default()
    },
    Status {
        name: HEATSINK,
        effect_at: Event::OnCard(CardType::Power),
        effect: Effect::Draw(1),
        ..Status::default()
    },
    Status {
        name: HELLO,
        effect_at: Event::TurnStart,
        effect: Effect::AddCard{
            card: CardReference::RandomRarity(CardRarity::Common),
            destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
            copies: 1,
            modifier: CardModifier::None,
        },
        ..Status::default()
    },
    Status {
        name: INFINITE_BLADES,
        effect_at: Event::TurnStart,
        effect: Effect::AddCard{
            card: CardReference::ByName(cards::SHIV),
            destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
            copies: 1,
            modifier: CardModifier::None,
        },
        ..Status::default()
    },
    Status {
        name: INFINITE_BLADES,
        effect_at: Event::TurnStart,
        effect: Effect::AddCard{
            card: CardReference::ByName(cards::SHIV),
            destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
            copies: 1,
            modifier: CardModifier::None,
        },
        ..Status::default()
    },
    Status {
        name: JUGGERNAUT,
        effect_at: Event::OnBlock,
        effect: Effect::Damage(1, EffectTarget::RandomEnemy),
        ..Status::default()
    },
    Status {
        name: LIKE_WATER,
        effect_at: Event::TurnEnd,
        effect: Effect::IfStance(Stance::Calm, vec![Effect::Block(1, EffectTarget::_Self)]),
        ..Status::default()
    },
    Status {
        name: LOOP,
        ..Status::default()
    },
    Status {
        name: MACHINE_LEARNING,
        effect_at: Event::TurnStart,
        effect: Effect::Draw(1),
        ..Status::default()
    },
    Status {
        name: MAGNETISM,
        effect_at: Event::TurnStart,
        effect: Effect::AddCard{
            card: CardReference::RandomClass(cards::CardClass::Neutral),
            destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
            copies: 1,
            modifier: CardModifier::None,
        },
        ..Status::default()
    },
    Status {
        name: MASTER_REALITY,
        is_additive: false,
        ..Status::default()
    },
    Status {
        name: MAYHEM,
        effect_at: Event::TurnStart,
        effect: Effect::AutoPlayCard(CardLocation::DrawPile(RelativePosition::Top)),
        ..Status::default()
    },
    Status {
        name: MENTAL_FORTRESS,
        effect_at: Event::OnStanceChange(Stance::All, Stance::All),
        effect: Effect::Block(1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: NIGHTMARE,
        is_additive: false,
        stacks: false,
        ..Status::default()
    },
    Status {
        name: NIRVANA,
        effect_at: Event::OnScry,
        effect: Effect::Block(1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: NOXIOUS_FUMES,
        effect_at: Event::TurnStart,
        effect: Effect::SetStatus(POISON, 1, EffectTarget::AllEnemies),
        ..Status::default()
    },
    Status {
        name: OMEGA,
        effect_at: Event::TurnEnd,
        effect: Effect::Damage(1, EffectTarget::AllEnemies),
        ..Status::default()
    },
    Status {
        name: PANACHE,
        ..Status::default()
    },
    Status {
        name: PEN_NIB,
        is_additive: false,
        ..Status::default()
    },
    Status {
        name: PHANTASMAL,
        reduce_at: Event::TurnStart,
        effect_at: Event::TurnStart,
        multiply_effect: false,
        effect: Effect::SetStatus(DOUBLE_DAMAGE, 1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: RAGE,
        effect_at: Event::OnCard(CardType::Attack),
        effect: Effect::Block(1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: REBOUND,
        reduce_at: Event::OnCard(CardType::All),
        ..Status::default()
    },
    Status {
        name: REGEN,
        reduce_at: Event::TurnEnd,
        effect_at: Event::TurnEnd,
        effect: Effect::Heal(1),
        ..Status::default()
    },
    Status {
        name: RUSHDOWN,
        effect_at: Event::OnStanceChange(Stance::All, Stance::Wrath),
        effect: Effect::Draw(1),
        ..Status::default()
    },
    Status {
        name: REPAIR,
        effect_at: Event::OnCombatEnd,
        effect: Effect::Heal(1),
        ..Status::default()
    },
    Status {
        name: RUPTURE,
        ..Status::default()
    },
    Status {
        name: SADISTIC,
        ..Status::default()
    },
    Status {
        name: SIMMERING_RAGE,
        is_additive: false,
        expire_at: Event::TurnStart,
        effect_at: Event::TurnStart,
        effect: Effect::SetStance(Stance::Wrath),
        ..Status::default()
    },
    Status {
        name: SIMMERING_RAGE,
        is_additive: false,
        expire_at: Event::TurnStart,
        effect_at: Event::TurnStart,
        effect: Effect::SetStance(Stance::Wrath),
        ..Status::default()
    },
    Status {
        name: STATIC_DISCHARGE,
        effect_at: Event::OnUnblockedDamage,
        effect: Effect::ChannelOrb(Orb::Lightning),
        ..Status::default()
    },
    Status {
        name: STORM,
        effect_at: Event::OnCard(CardType::Power),
        effect: Effect::ChannelOrb(Orb::Lightning),
        ..Status::default()
    },
    Status {
        name: STUDY,
        effect_at: Event::TurnEnd,
        effect: Effect::AddCard{
            card: CardReference::ByName(cards::INSIGHT),
            destination: CardLocation::DrawPile(RelativePosition::Random), 
            copies: 1,
            modifier: CardModifier::None,
        },
        ..Status::default()
    },
    Status {
        name: SURROUNDED,
        is_additive: false,
        ..Status::default()
    },
    Status {
        name: THE_BOMB,
        is_additive: false,
        stacks: false,
        ..Status::default()
    },
    Status {
        name: THOUSAND_CUTS,
        effect_at: Event::OnCard(CardType::All),
        effect: Effect::Damage(1, EffectTarget::AllEnemies),
        ..Status::default()
    },
    Status {
        name: TOOLS_OF_THE_TRADE,
        effect_at: Event::TurnStart,
        effect: Effect::Multiple(
            vec![
                Effect::Draw(1),
                Effect::DiscardCard(CardLocation::PlayerHand(RelativePosition::PlayerChoice(1)))
            ]
        ),
        ..Status::default()
    },
    Status {
        name: WAVE_OF_THE_HAND,
        effect_at: Event::OnBlock,
        effect: Effect::SetStatus(WEAK, 1, EffectTarget::AllEnemies),
        ..Status::default()
    },
    Status {
        name: WELL_LAID_PLANS,
        ..Status::default()
    },
    
    Status {
        name: CONFUSED,
        is_additive: false,
        is_buff: false,
        ..Status::default()
    },
    Status {
        name: DEXTERITY_DOWN,
        is_buff: false,
        effect_at: Event::TurnEnd,
        effect: Effect::SetStatus(DEXTERITY, -1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: FRAIL,
        is_buff: false,
        ..Status::default()
    },
    Status {
        name: NO_DRAW,
        is_buff: false,
        is_additive: false,
        ..Status::default()
    },
    Status {
        name: POISON,
        is_buff: false,
        effect_at: Event::TurnEnd,
        effect: Effect::LoseHp(1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: SHACKLED,
        is_buff: false,
        effect_at: Event::TurnEnd,
        effect: Effect::SetStatus(STRENGTH, 1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: SLOW,
        is_buff: false,
        ..Status::default()
    },
    Status {
        name: STRENGTH_DOWN,
        is_buff: false,
        effect_at: Event::TurnEnd,
        effect: Effect::SetStatus(STRENGTH, -1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: VULNERABLE,
        is_buff: false,
        ..Status::default()
    },
    Status {
        name: WEAK,
        is_buff: false,
        ..Status::default()
    },
    Status {
        name: BIAS,
        is_buff: false,
        effect_at: Event::TurnStart,
        effect: Effect::SetStatus(FOCUS, -1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: BLOCK_RETURN,
        is_buff: false,
        effect_at: Event::OnAttackDamage,
        effect: Effect::Block(1, EffectTarget::Attacker),
        ..Status::default()
    },
    Status {
        name: CHOKED,
        is_buff: false,
        effect_at: Event::OnCard(CardType::All),
        effect: Effect::LoseHp(1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: CONSTRICTED,
        is_buff: false,
        effect_at: Event::TurnEnd,
        effect: Effect::Damage(1, EffectTarget::_Self),
        ..Status::default()
    },
    Status {
        name: CORPSE_EXPLOSION,
        is_buff: false,
        ..Status::default()
    },
    Status {
        name: DRAW_REDUCTION,
        is_buff: false,
        expire_at: Event::TurnEnd,
        ..Status::default()
    },
    Status {
        name: ENTANGLED,
        is_buff: false,
        is_additive: false,
        expire_at: Event::TurnEnd,
        ..Status::default()
    },
    Status {
        name: FASTING,
        is_buff: false,
        effect_at: Event::TurnStart,
        effect: Effect::AddEnergy(-1),
        ..Status::default()
    },
    Status {
        name: HEX,
        is_buff: false,
        effect_at: Event::Multiple(vec![
            Event::OnCard(CardType::Curse),
            Event::OnCard(CardType::Power),
            Event::OnCard(CardType::Skill),
            Event::OnCard(CardType::Status),
        ]),
        effect: Effect::AddCard{
            card: CardReference::ByName(cards::DAZED),
            destination: CardLocation::DrawPile(RelativePosition::Random), 
            copies: 1,
            modifier: CardModifier::None,
        },
        ..Status::default()
    },
    Status {
        name: LOCK_ON,
        is_buff: false,
        reduce_at: Event::TurnStart,
        ..Status::default()
    },
    Status {
        name: MARK,
        is_buff: false,
        ..Status::default()
    },
    Status {
        name: NO_BLOCK,
        is_buff: false,
        reduce_at: Event::TurnEnd,
        ..Status::default()
    },
    Status {
        name: WRAITH_FORM,
        is_buff: false,
        effect_at: Event::TurnStart,
        effect: Effect::SetStatus(DEXTERITY, -1, EffectTarget::_Self),
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
pub const DARK_EMBRACE: &str = "Dark Embrace";
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
pub const PANACHE: &str = "Panache";
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