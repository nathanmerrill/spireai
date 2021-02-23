use crate::models::cards; 
use crate::models::core::*;
use Amount::*;

impl BaseBuff {
    fn default() -> Self {
        Self {
            name: &"",
            is_additive: true,
            stacks: true,
            is_buff: true,
            starting_n: 0,
            reduce_at: Event::Never,
            expire_at: Event::Never,
            effect_at: Event::Never,
            effect: Effect::None,
        }
    }
    
    pub fn by_name(name: &str) -> BaseBuff {
        match name {
            ARTIFACT => BaseBuff { 
                name: ARTIFACT,
                ..BaseBuff::default()
            },
            BARRICADE => BaseBuff { 
                name: BARRICADE,
                is_additive: false,
                ..BaseBuff::default()
            },
            BUFFER => BaseBuff { 
                name: BUFFER,
                ..BaseBuff::default()
            },
            DEXTERITY => BaseBuff { 
                name: DEXTERITY,
                ..BaseBuff::default()
            },
            DRAW_CARD => BaseBuff { 
                name: DRAW_CARD,
                effect_at: Event::TurnStart,
                expire_at: Event::TurnStart,
                effect: Effect::Draw(X),
                ..BaseBuff::default()
            },
            ENERGIZED => BaseBuff { 
                name: ENERGIZED,
                effect_at: Event::TurnStart,
                expire_at: Event::TurnStart,
                effect: Effect::AddEnergy(X),
                ..BaseBuff::default()
            },
            FOCUS => BaseBuff { 
                name: FOCUS,
                ..BaseBuff::default()
            },
            INTANGIBLE => BaseBuff { 
                name: INTANGIBLE,
                reduce_at: Event::TurnStart,
                ..BaseBuff::default()
            },
            MANTRA => BaseBuff { 
                name: MANTRA,
                ..BaseBuff::default()
            },
            METALLICIZE => BaseBuff { 
                name: METALLICIZE,
                effect_at: Event::TurnStart,
                effect: Effect::Block(X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            NEXT_TURN_BLOCK => BaseBuff { 
                name: NEXT_TURN_BLOCK,
                effect_at: Event::TurnStart,
                expire_at: Event::TurnStart,
                effect: Effect::Block(X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            PLATED_ARMOR => BaseBuff { 
                name: PLATED_ARMOR,
                effect_at: Event::TurnStart,
                reduce_at: Event::UnblockedDamage(EffectTarget::_Self),
                effect: Effect::Block(X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            RITUAL => BaseBuff { 
                name: RITUAL,
                effect_at: Event::TurnEnd,
                effect: Effect::AddBuff(STRENGTH, X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            STRENGTH => BaseBuff { 
                name: STRENGTH,
                ..BaseBuff::default()
            },
            THORNS => BaseBuff { 
                name: THORNS,
                effect_at: Event::AttackDamage(EffectTarget::_Self),
                effect: Effect::Damage(X, EffectTarget::Attacker),
                ..BaseBuff::default()
            },
            VIGOR => BaseBuff { 
                name: VIGOR,
                expire_at: Event::PlayCard(CardType::Attack),
                ..BaseBuff::default()
            },
            ACCURACY => BaseBuff { 
                name: ACCURACY,
                ..BaseBuff::default()
            },
            AFTER_IMAGE => BaseBuff { 
                name: AFTER_IMAGE,
                effect_at: Event::PlayCard(CardType::All),
                effect: Effect::Block(X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            AMPLIFY => BaseBuff { 
                name: AMPLIFY,
                expire_at: Event::TurnEnd,
                ..BaseBuff::default()
            },
            BATTLE_HYMN => BaseBuff { 
                name: BATTLE_HYMN,
                effect_at: Event::TurnStart,
                effect: Effect::AddCard{
                    card: CardReference::ByName(cards::SMITE),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..BaseBuff::default()
            },
            BERSERK => BaseBuff { 
                name: BERSERK,
                effect_at: Event::TurnStart,
                effect: Effect::AddEnergy(X),
                ..BaseBuff::default()
            },
            BLASPHEMER => BaseBuff { 
                name: BLASPHEMER,
                is_additive: false,
                effect_at: Event::TurnStart,
                expire_at: Event::TurnStart,
                effect: Effect::Damage(Fixed(9999), EffectTarget::_Self),
                ..BaseBuff::default()
            },
            BLUR => BaseBuff { 
                name: BLUR,
                reduce_at: Event::TurnStart,
                ..BaseBuff::default()
            },
            BRUTALITY => BaseBuff { 
                name: BRUTALITY,
                effect_at: Event::TurnStart,
                effect: Effect::Multiple(
                    vec![
                        Effect::LoseHp(X, EffectTarget::_Self),
                        Effect::Draw(X),
                    ]),
                ..BaseBuff::default()
            },
            BURST => BaseBuff { 
                name: BURST,
                reduce_at: Event::PlayCard(CardType::Skill),
                ..BaseBuff::default()
            },
            COLLECT => BaseBuff { 
                name: COLLECT,
                effect_at: Event::TurnStart,
                reduce_at: Event::TurnStart,
                effect: Effect::AddCard{
                    card: CardReference::ByName(cards::COLLECT),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: Fixed(1),
                    modifier: CardModifier::Upgraded,
                },
                ..BaseBuff::default()
            },
            COMBUST => BaseBuff { 
                name: COMBUST,
                ..BaseBuff::default()
            },
            CORRUPTION => BaseBuff { 
                name: CORRUPTION,
                is_additive: false,
                ..BaseBuff::default()
            },
            CREATIVE_AI => BaseBuff { 
                name: CREATIVE_AI,
                effect_at: Event::TurnStart,
                effect: Effect::AddCard{
                    card: CardReference::RandomType(CardType::Power),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..BaseBuff::default()
            },
            DARK_EMBRACE => BaseBuff { 
                name: DARK_EMBRACE,
                effect_at: Event::Exhaust,
                effect: Effect::Draw(X),
                ..BaseBuff::default()
            },
            DEMON_FORM => BaseBuff { 
                name: DEMON_FORM,
                effect_at: Event::TurnStart,
                effect: Effect::AddBuff(STRENGTH, X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            DEVA => BaseBuff { 
                name: DEVA,
                effect_at: Event::TurnStart,
                effect: Effect::Multiple(vec![
                    Effect::AddEnergy(N),
                    Effect::AddBuffN(DEVA, X, EffectTarget::_Self),
                ]),
                ..BaseBuff::default()
            },
            DEVOTION => BaseBuff { 
                name: DEVOTION,
                effect_at: Event::TurnStart,
                effect: Effect::AddBuff(MANTRA, X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            DOUBLE_DAMAGE => BaseBuff { 
                name: DOUBLE_DAMAGE,
                reduce_at: Event::TurnEnd,
                ..BaseBuff::default()
            },
            DOUBLE_TAP => BaseBuff { 
                name: DOUBLE_TAP,
                reduce_at: Event::PlayCard(CardType::Attack),
                ..BaseBuff::default()
            },
            DUPLICATION => BaseBuff { 
                name: DUPLICATION,
                reduce_at: Event::PlayCard(CardType::All),
                ..BaseBuff::default()
            },
            ECHO_FORM => BaseBuff { 
                name: ECHO_FORM,
                ..BaseBuff::default()
            },
            ELECTRO => BaseBuff { 
                name: ELECTRO,
                is_additive: false,
                ..BaseBuff::default()
            },
            ENVENOM => BaseBuff { 
                name: ENVENOM,
                effect_at: Event::UnblockedDamage(EffectTarget::TargetEnemy),
                effect: Effect::AddBuff(POISON, X, EffectTarget::TargetEnemy),
                ..BaseBuff::default()
            },
            EQUILIBRIUM => BaseBuff { 
                name: EQUILIBRIUM,
                reduce_at: Event::TurnStart,
                ..BaseBuff::default()
            },
            ESTABLISHMENT => BaseBuff { 
                name: ESTABLISHMENT,
                ..BaseBuff::default()
            },
            EVOLVE => BaseBuff { 
                name: EVOLVE,
                effect_at: Event::DrawCard(CardType::Status),
                effect: Effect::Draw(X),
                ..BaseBuff::default()
            },
            FEEL_NO_PAIN => BaseBuff { 
                name: FEEL_NO_PAIN,
                effect_at: Event::Exhaust,
                effect: Effect::Block(X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            FIRE_BREATHING => BaseBuff { 
                name: FIRE_BREATHING,
                effect_at: Event::Multiple(vec![Event::DrawCard(CardType::Status), Event::DrawCard(CardType::Status)]),
                effect: Effect::Damage(X, EffectTarget::AllEnemies),
                ..BaseBuff::default()
            },
            FORESIGHT => BaseBuff { 
                name: FORESIGHT,
                effect_at: Event::TurnStart,
                effect: Effect::Scry(X),
                ..BaseBuff::default()
            },
            FREE_ATTACK_POWER => BaseBuff { 
                name: FREE_ATTACK_POWER,
                reduce_at: Event::PlayCard(CardType::Attack),
                ..BaseBuff::default()
            },
            HEATSINK => BaseBuff { 
                name: HEATSINK,
                effect_at: Event::PlayCard(CardType::Power),
                effect: Effect::Draw(X),
                ..BaseBuff::default()
            },
            HELLO => BaseBuff { 
                name: HELLO,
                effect_at: Event::TurnStart,
                effect: Effect::AddCard{
                    card: CardReference::RandomRarity(Rarity::Common),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..BaseBuff::default()
            },
            INFINITE_BLADES => BaseBuff { 
                name: INFINITE_BLADES,
                effect_at: Event::TurnStart,
                effect: Effect::AddCard{
                    card: CardReference::ByName(cards::SHIV),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..BaseBuff::default()
            },
            JUGGERNAUT => BaseBuff { 
                name: JUGGERNAUT,
                effect_at: Event::Block(EffectTarget::_Self),
                effect: Effect::Damage(X, EffectTarget::RandomEnemy),
                ..BaseBuff::default()
            },
            LIKE_WATER => BaseBuff { 
                name: LIKE_WATER,
                effect_at: Event::TurnEnd,
                effect: Effect::IfStance(Stance::Calm, vec![Effect::Block(X, EffectTarget::_Self)]),
                ..BaseBuff::default()
            },
            LOOP => BaseBuff { 
                name: LOOP,
                ..BaseBuff::default()
            },
            MACHINE_LEARNING => BaseBuff { 
                name: MACHINE_LEARNING,
                effect_at: Event::TurnStart,
                effect: Effect::Draw(X),
                ..BaseBuff::default()
            },
            MAGNETISM => BaseBuff { 
                name: MAGNETISM,
                effect_at: Event::TurnStart,
                effect: Effect::AddCard{
                    card: CardReference::RandomClass(Class::None),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..BaseBuff::default()
            },
            MASTER_REALITY => BaseBuff { 
                name: MASTER_REALITY,
                is_additive: false,
                ..BaseBuff::default()
            },
            MAYHEM => BaseBuff { 
                name: MAYHEM,
                effect_at: Event::TurnStart,
                effect: Effect::AutoPlayCard(CardLocation::DrawPile(RelativePosition::Top)),
                ..BaseBuff::default()
            },
            MENTAL_FORTRESS => BaseBuff { 
                name: MENTAL_FORTRESS,
                effect_at: Event::StanceChange(Stance::All, Stance::All),
                effect: Effect::Block(X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            NIGHTMARE => BaseBuff { 
                name: NIGHTMARE,
                is_additive: false,
                stacks: false,
                ..BaseBuff::default()
            },
            NIRVANA => BaseBuff { 
                name: NIRVANA,
                effect_at: Event::Scry,
                effect: Effect::Block(X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            NOXIOUS_FUMES => BaseBuff { 
                name: NOXIOUS_FUMES,
                effect_at: Event::TurnStart,
                effect: Effect::AddBuff(POISON, X, EffectTarget::AllEnemies),
                ..BaseBuff::default()
            },
            OMEGA => BaseBuff { 
                name: OMEGA,
                effect_at: Event::TurnEnd,
                effect: Effect::Damage(X, EffectTarget::AllEnemies),
                ..BaseBuff::default()
            },
            PANACHE => BaseBuff { 
                name: PANACHE,
                starting_n: 5,
                effect_at: Event::PlayCard(CardType::All),
                effect: Effect::Multiple(vec![
                    Effect::AddBuffN(PANACHE, Fixed(-1), EffectTarget::_Self),
                    Effect::IfBuffN(EffectTarget::_Self, PANACHE, Fixed(0), vec![
                        Effect::AddBuffN(PANACHE, Fixed(5), EffectTarget::_Self),
                        Effect::Damage(X, EffectTarget::AllEnemies),
                    ]),
                ]),

                ..BaseBuff::default()
            },
            PEN_NIB => BaseBuff { 
                name: PEN_NIB,
                is_additive: false,
                ..BaseBuff::default()
            },
            PHANTASMAL => BaseBuff { 
                name: PHANTASMAL,
                reduce_at: Event::TurnStart,
                effect_at: Event::TurnStart,
                effect: Effect::AddBuff(DOUBLE_DAMAGE, Fixed(1), EffectTarget::_Self),
                ..BaseBuff::default()
            },
            RAGE => BaseBuff { 
                name: RAGE,
                effect_at: Event::PlayCard(CardType::Attack),
                effect: Effect::Block(X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            REBOUND => BaseBuff { 
                name: REBOUND,
                reduce_at: Event::PlayCard(CardType::All),
                ..BaseBuff::default()
            },
            REGEN => BaseBuff { 
                name: REGEN,
                reduce_at: Event::TurnEnd,
                effect_at: Event::TurnEnd,
                effect: Effect::Heal(X),
                ..BaseBuff::default()
            },
            RUSHDOWN => BaseBuff { 
                name: RUSHDOWN,
                effect_at: Event::StanceChange(Stance::All, Stance::Wrath),
                effect: Effect::Draw(X),
                ..BaseBuff::default()
            },
            REPAIR => BaseBuff { 
                name: REPAIR,
                effect_at: Event::CombatEnd,
                effect: Effect::Heal(X),
                ..BaseBuff::default()
            },
            RUPTURE => BaseBuff { 
                name: RUPTURE,
                ..BaseBuff::default()
            },
            SADISTIC => BaseBuff { 
                name: SADISTIC,
                ..BaseBuff::default()
            },
            SIMMERING_RAGE => BaseBuff { 
                name: SIMMERING_RAGE,
                is_additive: false,
                expire_at: Event::TurnStart,
                effect_at: Event::TurnStart,
                effect: Effect::SetStance(Stance::Wrath),
                ..BaseBuff::default()
            },
            STATIC_DISCHARGE => BaseBuff { 
                name: STATIC_DISCHARGE,
                effect_at: Event::UnblockedDamage(EffectTarget::_Self),
                effect: Effect::ChannelOrb(Orb::Lightning),
                ..BaseBuff::default()
            },
            STORM => BaseBuff { 
                name: STORM,
                effect_at: Event::PlayCard(CardType::Power),
                effect: Effect::ChannelOrb(Orb::Lightning),
                ..BaseBuff::default()
            },
            STUDY => BaseBuff { 
                name: STUDY,
                effect_at: Event::TurnEnd,
                effect: Effect::AddCard{
                    card: CardReference::ByName(cards::INSIGHT),
                    destination: CardLocation::DrawPile(RelativePosition::Random), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..BaseBuff::default()
            },
            SURROUNDED => BaseBuff { 
                name: SURROUNDED,
                is_additive: false,
                ..BaseBuff::default()
            },
            THE_BOMB => BaseBuff { 
                name: THE_BOMB,
                is_additive: false,
                effect_at: Event::TurnEnd,
                effect: Effect::Custom,
                stacks: false,
                ..BaseBuff::default()
            },
            THOUSAND_CUTS => BaseBuff { 
                name: THOUSAND_CUTS,
                effect_at: Event::PlayCard(CardType::All),
                effect: Effect::Damage(X, EffectTarget::AllEnemies),
                ..BaseBuff::default()
            },
            TOOLS_OF_THE_TRADE => BaseBuff { 
                name: TOOLS_OF_THE_TRADE,
                effect_at: Event::TurnStart,
                effect: Effect::Multiple(
                    vec![
                        Effect::Draw(X),
                        Effect::DiscardCard(CardLocation::PlayerHand(RelativePosition::PlayerChoice(1)))
                    ]
                ),
                ..BaseBuff::default()
            },
            WAVE_OF_THE_HAND => BaseBuff { 
                name: WAVE_OF_THE_HAND,
                effect_at: Event::Block(EffectTarget::_Self),
                effect: Effect::AddBuff(WEAK, X, EffectTarget::AllEnemies),
                ..BaseBuff::default()
            },
            WELL_LAID_PLANS => BaseBuff { 
                name: WELL_LAID_PLANS,
                ..BaseBuff::default()
            },
            
            CONFUSED => BaseBuff { 
                name: CONFUSED,
                is_additive: false,
                is_buff: false,
                ..BaseBuff::default()
            },
            DEXTERITY_DOWN => BaseBuff { 
                name: DEXTERITY_DOWN,
                is_buff: false,
                effect_at: Event::TurnEnd,
                effect: Effect::AddBuff(DEXTERITY, NegX, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            FRAIL => BaseBuff { 
                name: FRAIL,
                is_buff: false,
                ..BaseBuff::default()
            },
            NO_DRAW => BaseBuff { 
                name: NO_DRAW,
                is_buff: false,
                is_additive: false,
                ..BaseBuff::default()
            },
            POISON => BaseBuff { 
                name: POISON,
                is_buff: false,
                effect_at: Event::TurnEnd,
                effect: Effect::LoseHp(X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            SHACKLED => BaseBuff { 
                name: SHACKLED,
                is_buff: false,
                effect_at: Event::TurnEnd,
                effect: Effect::AddBuff(STRENGTH, X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            SLOW => BaseBuff { 
                name: SLOW,
                is_buff: false,
                ..BaseBuff::default()
            },
            STRENGTH_DOWN => BaseBuff { 
                name: STRENGTH_DOWN,
                is_buff: false,
                effect_at: Event::TurnEnd,
                effect: Effect::AddBuff(STRENGTH, NegX, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            VULNERABLE => BaseBuff { 
                name: VULNERABLE,
                is_buff: false,
                ..BaseBuff::default()
            },
            WEAK => BaseBuff { 
                name: WEAK,
                is_buff: false,
                ..BaseBuff::default()
            },
            BIAS => BaseBuff { 
                name: BIAS,
                is_buff: false,
                effect_at: Event::TurnStart,
                effect: Effect::AddBuff(FOCUS, NegX, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            BLOCK_RETURN => BaseBuff { 
                name: BLOCK_RETURN,
                is_buff: false,
                effect_at: Event::AttackDamage(EffectTarget::_Self),
                effect: Effect::Block(X, EffectTarget::Attacker),
                ..BaseBuff::default()
            },
            CHOKED => BaseBuff { 
                name: CHOKED,
                is_buff: false,
                effect_at: Event::PlayCard(CardType::All),
                effect: Effect::LoseHp(X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            CONSTRICTED => BaseBuff { 
                name: CONSTRICTED,
                is_buff: false,
                effect_at: Event::TurnEnd,
                effect: Effect::Damage(X, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            CORPSE_EXPLOSION => BaseBuff { 
                name: CORPSE_EXPLOSION,
                is_buff: false,
                ..BaseBuff::default()
            },
            DRAW_REDUCTION => BaseBuff { 
                name: DRAW_REDUCTION,
                is_buff: false,
                expire_at: Event::TurnEnd,
                ..BaseBuff::default()
            },
            ENTANGLED => BaseBuff { 
                name: ENTANGLED,
                is_buff: false,
                is_additive: false,
                expire_at: Event::TurnEnd,
                ..BaseBuff::default()
            },
            FASTING => BaseBuff { 
                name: FASTING,
                is_buff: false,
                effect_at: Event::TurnStart,
                effect: Effect::AddEnergy(NegX),
                ..BaseBuff::default()
            },
            HEX => BaseBuff { 
                name: HEX,
                is_buff: false,
                effect_at: Event::Multiple(vec![
                    Event::PlayCard(CardType::Curse),
                    Event::PlayCard(CardType::Power),
                    Event::PlayCard(CardType::Skill),
                    Event::PlayCard(CardType::Status),
                ]),
                effect: Effect::AddCard{
                    card: CardReference::ByName(cards::DAZED),
                    destination: CardLocation::DrawPile(RelativePosition::Random), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..BaseBuff::default()
            },
            LOCK_ON => BaseBuff { 
                name: LOCK_ON,
                is_buff: false,
                reduce_at: Event::TurnStart,
                ..BaseBuff::default()
            },
            MARK => BaseBuff { 
                name: MARK,
                is_buff: false,
                ..BaseBuff::default()
            },
            NO_BLOCK => BaseBuff { 
                name: NO_BLOCK,
                is_buff: false,
                reduce_at: Event::TurnEnd,
                ..BaseBuff::default()
            },
            WRAITH_FORM => BaseBuff { 
                name: WRAITH_FORM,
                is_buff: false,
                effect_at: Event::TurnStart,
                effect: Effect::AddBuff(DEXTERITY, NegX, EffectTarget::_Self),
                ..BaseBuff::default()
            },
            _ => panic!("Unrecognized BaseBuff name"),
        }
    }
}

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