use crate::models::cards; 
use crate::models::core::*;
use Amount::*;

impl Status {
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
    
    pub fn by_name(name: &str) -> Status {
        match name {
            ARTIFACT => Status { 
                name: ARTIFACT,
                ..Status::default()
            },
            BARRICADE => Status { 
                name: BARRICADE,
                is_additive: false,
                ..Status::default()
            },
            BUFFER => Status { 
                name: BUFFER,
                ..Status::default()
            },
            DEXTERITY => Status { 
                name: DEXTERITY,
                ..Status::default()
            },
            DRAW_CARD => Status { 
                name: DRAW_CARD,
                effect_at: Event::TurnStart,
                expire_at: Event::TurnStart,
                effect: Effect::Draw(X),
                ..Status::default()
            },
            ENERGIZED => Status { 
                name: ENERGIZED,
                effect_at: Event::TurnStart,
                expire_at: Event::TurnStart,
                effect: Effect::AddEnergy(X),
                ..Status::default()
            },
            FOCUS => Status { 
                name: FOCUS,
                ..Status::default()
            },
            INTANGIBLE => Status { 
                name: INTANGIBLE,
                reduce_at: Event::TurnStart,
                ..Status::default()
            },
            MANTRA => Status { 
                name: MANTRA,
                ..Status::default()
            },
            METALLICIZE => Status { 
                name: METALLICIZE,
                effect_at: Event::TurnStart,
                effect: Effect::Block(X, EffectTarget::_Self),
                ..Status::default()
            },
            NEXT_TURN_BLOCK => Status { 
                name: NEXT_TURN_BLOCK,
                effect_at: Event::TurnStart,
                expire_at: Event::TurnStart,
                effect: Effect::Block(X, EffectTarget::_Self),
                ..Status::default()
            },
            PLATED_ARMOR => Status { 
                name: PLATED_ARMOR,
                effect_at: Event::TurnStart,
                reduce_at: Event::OnUnblockedDamage,
                effect: Effect::Block(X, EffectTarget::_Self),
                ..Status::default()
            },
            RITUAL => Status { 
                name: RITUAL,
                effect_at: Event::TurnEnd,
                effect: Effect::SetStatus(STRENGTH, X, EffectTarget::_Self),
                ..Status::default()
            },
            STRENGTH => Status { 
                name: STRENGTH,
                ..Status::default()
            },
            THORNS => Status { 
                name: THORNS,
                effect_at: Event::OnAttackDamage,
                effect: Effect::Damage(X, EffectTarget::Attacker),
                ..Status::default()
            },
            VIGOR => Status { 
                name: VIGOR,
                expire_at: Event::OnCard(CardType::Attack),
                ..Status::default()
            },
            ACCURACY => Status { 
                name: ACCURACY,
                ..Status::default()
            },
            AFTER_IMAGE => Status { 
                name: AFTER_IMAGE,
                effect_at: Event::OnCard(CardType::All),
                effect: Effect::Block(X, EffectTarget::_Self),
                ..Status::default()
            },
            AMPLIFY => Status { 
                name: AMPLIFY,
                expire_at: Event::TurnEnd,
                ..Status::default()
            },
            BATTLE_HYMN => Status { 
                name: BATTLE_HYMN,
                effect_at: Event::TurnStart,
                effect: Effect::AddCard{
                    card: CardReference::ByName(cards::SMITE),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..Status::default()
            },
            BERSERK => Status { 
                name: BERSERK,
                effect_at: Event::TurnStart,
                effect: Effect::AddEnergy(X),
                ..Status::default()
            },
            BLASPHEMER => Status { 
                name: BLASPHEMER,
                is_additive: false,
                effect_at: Event::TurnStart,
                expire_at: Event::TurnStart,
                effect: Effect::Damage(Fixed(9999), EffectTarget::_Self),
                ..Status::default()
            },
            BLUR => Status { 
                name: BLUR,
                reduce_at: Event::TurnStart,
                ..Status::default()
            },
            BRUTALITY => Status { 
                name: BRUTALITY,
                effect_at: Event::TurnStart,
                effect: Effect::Multiple(
                    vec![
                        Effect::LoseHp(X, EffectTarget::_Self),
                        Effect::Draw(X),
                    ]),
                ..Status::default()
            },
            BURST => Status { 
                name: BURST,
                reduce_at: Event::OnCard(CardType::Skill),
                ..Status::default()
            },
            COLLECT => Status { 
                name: COLLECT,
                effect_at: Event::TurnStart,
                reduce_at: Event::TurnStart,
                effect: Effect::AddCard{
                    card: CardReference::ByName(cards::COLLECT),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: Fixed(1),
                    modifier: CardModifier::Upgraded,
                },
                ..Status::default()
            },
            COMBUST => Status { 
                name: COMBUST,
                ..Status::default()
            },
            CORRUPTION => Status { 
                name: CORRUPTION,
                is_additive: false,
                ..Status::default()
            },
            CREATIVE_AI => Status { 
                name: CREATIVE_AI,
                effect_at: Event::TurnStart,
                effect: Effect::AddCard{
                    card: CardReference::RandomType(CardType::Power),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..Status::default()
            },
            DARK_EMBRACE => Status { 
                name: DARK_EMBRACE,
                effect_at: Event::OnExhaust,
                effect: Effect::Draw(X),
                ..Status::default()
            },
            DEMON_FORM => Status { 
                name: DEMON_FORM,
                effect_at: Event::TurnStart,
                effect: Effect::SetStatus(STRENGTH, X, EffectTarget::_Self),
                ..Status::default()
            },
            DEVA => Status { 
                name: DEVA,
                effect_at: Event::TurnStart,
                effect: Effect::Multiple(vec![
                    Effect::AddEnergy(N),
                    Effect::IncreaseStatusN(DEVA, X, EffectTarget::_Self),
                ]),
                ..Status::default()
            },
            DEVOTION => Status { 
                name: DEVOTION,
                effect_at: Event::TurnStart,
                effect: Effect::AddMantra(X),
                ..Status::default()
            },
            DOUBLE_DAMAGE => Status { 
                name: DOUBLE_DAMAGE,
                reduce_at: Event::TurnEnd,
                ..Status::default()
            },
            DOUBLE_TAP => Status { 
                name: DOUBLE_TAP,
                reduce_at: Event::OnCard(CardType::Attack),
                ..Status::default()
            },
            DUPLICATION => Status { 
                name: DUPLICATION,
                reduce_at: Event::OnCard(CardType::All),
                ..Status::default()
            },
            ECHO_FORM => Status { 
                name: ECHO_FORM,
                ..Status::default()
            },
            ELECTRO => Status { 
                name: ELECTRO,
                is_additive: false,
                ..Status::default()
            },
            ENVENOM => Status { 
                name: ENVENOM,
                effect_at: Event::OnTargetUnblockedDamage,
                effect: Effect::SetStatus(POISON, X, EffectTarget::TargetEnemy),
                ..Status::default()
            },
            EQUILIBRIUM => Status { 
                name: EQUILIBRIUM,
                reduce_at: Event::TurnStart,
                ..Status::default()
            },
            ESTABLISHMENT => Status { 
                name: ESTABLISHMENT,
                ..Status::default()
            },
            EVOLVE => Status { 
                name: EVOLVE,
                effect_at: Event::OnDraw(CardType::Status),
                effect: Effect::Draw(X),
                ..Status::default()
            },
            FEEL_NO_PAIN => Status { 
                name: FEEL_NO_PAIN,
                effect_at: Event::OnExhaust,
                effect: Effect::Block(X, EffectTarget::_Self),
                ..Status::default()
            },
            FIRE_BREATHING => Status { 
                name: FIRE_BREATHING,
                effect_at: Event::Multiple(vec![Event::OnDraw(CardType::Status), Event::OnDraw(CardType::Status)]),
                effect: Effect::Damage(X, EffectTarget::AllEnemies),
                ..Status::default()
            },
            FORESIGHT => Status { 
                name: FORESIGHT,
                effect_at: Event::TurnStart,
                effect: Effect::Scry(X),
                ..Status::default()
            },
            FREE_ATTACK_POWER => Status { 
                name: FREE_ATTACK_POWER,
                reduce_at: Event::OnCard(CardType::Attack),
                ..Status::default()
            },
            HEATSINK => Status { 
                name: HEATSINK,
                effect_at: Event::OnCard(CardType::Power),
                effect: Effect::Draw(X),
                ..Status::default()
            },
            HELLO => Status { 
                name: HELLO,
                effect_at: Event::TurnStart,
                effect: Effect::AddCard{
                    card: CardReference::RandomRarity(Rarity::Common),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..Status::default()
            },
            INFINITE_BLADES => Status { 
                name: INFINITE_BLADES,
                effect_at: Event::TurnStart,
                effect: Effect::AddCard{
                    card: CardReference::ByName(cards::SHIV),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..Status::default()
            },
            JUGGERNAUT => Status { 
                name: JUGGERNAUT,
                effect_at: Event::OnBlock,
                effect: Effect::Damage(X, EffectTarget::RandomEnemy),
                ..Status::default()
            },
            LIKE_WATER => Status { 
                name: LIKE_WATER,
                effect_at: Event::TurnEnd,
                effect: Effect::IfStance(Stance::Calm, vec![Effect::Block(X, EffectTarget::_Self)]),
                ..Status::default()
            },
            LOOP => Status { 
                name: LOOP,
                ..Status::default()
            },
            MACHINE_LEARNING => Status { 
                name: MACHINE_LEARNING,
                effect_at: Event::TurnStart,
                effect: Effect::Draw(X),
                ..Status::default()
            },
            MAGNETISM => Status { 
                name: MAGNETISM,
                effect_at: Event::TurnStart,
                effect: Effect::AddCard{
                    card: CardReference::RandomClass(Class::None),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..Status::default()
            },
            MASTER_REALITY => Status { 
                name: MASTER_REALITY,
                is_additive: false,
                ..Status::default()
            },
            MAYHEM => Status { 
                name: MAYHEM,
                effect_at: Event::TurnStart,
                effect: Effect::AutoPlayCard(CardLocation::DrawPile(RelativePosition::Top)),
                ..Status::default()
            },
            MENTAL_FORTRESS => Status { 
                name: MENTAL_FORTRESS,
                effect_at: Event::OnStanceChange(Stance::All, Stance::All),
                effect: Effect::Block(X, EffectTarget::_Self),
                ..Status::default()
            },
            NIGHTMARE => Status { 
                name: NIGHTMARE,
                is_additive: false,
                stacks: false,
                ..Status::default()
            },
            NIRVANA => Status { 
                name: NIRVANA,
                effect_at: Event::OnScry,
                effect: Effect::Block(X, EffectTarget::_Self),
                ..Status::default()
            },
            NOXIOUS_FUMES => Status { 
                name: NOXIOUS_FUMES,
                effect_at: Event::TurnStart,
                effect: Effect::SetStatus(POISON, X, EffectTarget::AllEnemies),
                ..Status::default()
            },
            OMEGA => Status { 
                name: OMEGA,
                effect_at: Event::TurnEnd,
                effect: Effect::Damage(X, EffectTarget::AllEnemies),
                ..Status::default()
            },
            PANACHE => Status { 
                name: PANACHE,
                starting_n: 5,
                effect_at: Event::OnCard(CardType::All),
                effect: Effect::Multiple(vec![
                    Effect::IncreaseStatusN(PANACHE, Fixed(-1), EffectTarget::_Self),
                    Effect::IfStatusN(EffectTarget::_Self, PANACHE, Fixed(0), vec![
                        Effect::IncreaseStatusN(PANACHE, Fixed(5), EffectTarget::_Self),
                        Effect::Damage(X, EffectTarget::AllEnemies),
                    ]),
                ]),

                ..Status::default()
            },
            PEN_NIB => Status { 
                name: PEN_NIB,
                is_additive: false,
                ..Status::default()
            },
            PHANTASMAL => Status { 
                name: PHANTASMAL,
                reduce_at: Event::TurnStart,
                effect_at: Event::TurnStart,
                effect: Effect::SetStatus(DOUBLE_DAMAGE, Fixed(1), EffectTarget::_Self),
                ..Status::default()
            },
            RAGE => Status { 
                name: RAGE,
                effect_at: Event::OnCard(CardType::Attack),
                effect: Effect::Block(X, EffectTarget::_Self),
                ..Status::default()
            },
            REBOUND => Status { 
                name: REBOUND,
                reduce_at: Event::OnCard(CardType::All),
                ..Status::default()
            },
            REGEN => Status { 
                name: REGEN,
                reduce_at: Event::TurnEnd,
                effect_at: Event::TurnEnd,
                effect: Effect::Heal(X),
                ..Status::default()
            },
            RUSHDOWN => Status { 
                name: RUSHDOWN,
                effect_at: Event::OnStanceChange(Stance::All, Stance::Wrath),
                effect: Effect::Draw(X),
                ..Status::default()
            },
            REPAIR => Status { 
                name: REPAIR,
                effect_at: Event::OnCombatEnd,
                effect: Effect::Heal(X),
                ..Status::default()
            },
            RUPTURE => Status { 
                name: RUPTURE,
                ..Status::default()
            },
            SADISTIC => Status { 
                name: SADISTIC,
                ..Status::default()
            },
            SIMMERING_RAGE => Status { 
                name: SIMMERING_RAGE,
                is_additive: false,
                expire_at: Event::TurnStart,
                effect_at: Event::TurnStart,
                effect: Effect::SetStance(Stance::Wrath),
                ..Status::default()
            },
            STATIC_DISCHARGE => Status { 
                name: STATIC_DISCHARGE,
                effect_at: Event::OnUnblockedDamage,
                effect: Effect::ChannelOrb(Orb::Lightning),
                ..Status::default()
            },
            STORM => Status { 
                name: STORM,
                effect_at: Event::OnCard(CardType::Power),
                effect: Effect::ChannelOrb(Orb::Lightning),
                ..Status::default()
            },
            STUDY => Status { 
                name: STUDY,
                effect_at: Event::TurnEnd,
                effect: Effect::AddCard{
                    card: CardReference::ByName(cards::INSIGHT),
                    destination: CardLocation::DrawPile(RelativePosition::Random), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..Status::default()
            },
            SURROUNDED => Status { 
                name: SURROUNDED,
                is_additive: false,
                ..Status::default()
            },
            THE_BOMB => Status { 
                name: THE_BOMB,
                is_additive: false,
                effect_at: Event::TurnEnd,
                effect: Effect::Custom,
                stacks: false,
                ..Status::default()
            },
            THOUSAND_CUTS => Status { 
                name: THOUSAND_CUTS,
                effect_at: Event::OnCard(CardType::All),
                effect: Effect::Damage(X, EffectTarget::AllEnemies),
                ..Status::default()
            },
            TOOLS_OF_THE_TRADE => Status { 
                name: TOOLS_OF_THE_TRADE,
                effect_at: Event::TurnStart,
                effect: Effect::Multiple(
                    vec![
                        Effect::Draw(X),
                        Effect::DiscardCard(CardLocation::PlayerHand(RelativePosition::PlayerChoice(1)))
                    ]
                ),
                ..Status::default()
            },
            WAVE_OF_THE_HAND => Status { 
                name: WAVE_OF_THE_HAND,
                effect_at: Event::OnBlock,
                effect: Effect::SetStatus(WEAK, X, EffectTarget::AllEnemies),
                ..Status::default()
            },
            WELL_LAID_PLANS => Status { 
                name: WELL_LAID_PLANS,
                ..Status::default()
            },
            
            CONFUSED => Status { 
                name: CONFUSED,
                is_additive: false,
                is_buff: false,
                ..Status::default()
            },
            DEXTERITY_DOWN => Status { 
                name: DEXTERITY_DOWN,
                is_buff: false,
                effect_at: Event::TurnEnd,
                effect: Effect::SetStatus(DEXTERITY, NegX, EffectTarget::_Self),
                ..Status::default()
            },
            FRAIL => Status { 
                name: FRAIL,
                is_buff: false,
                ..Status::default()
            },
            NO_DRAW => Status { 
                name: NO_DRAW,
                is_buff: false,
                is_additive: false,
                ..Status::default()
            },
            POISON => Status { 
                name: POISON,
                is_buff: false,
                effect_at: Event::TurnEnd,
                effect: Effect::LoseHp(X, EffectTarget::_Self),
                ..Status::default()
            },
            SHACKLED => Status { 
                name: SHACKLED,
                is_buff: false,
                effect_at: Event::TurnEnd,
                effect: Effect::SetStatus(STRENGTH, X, EffectTarget::_Self),
                ..Status::default()
            },
            SLOW => Status { 
                name: SLOW,
                is_buff: false,
                ..Status::default()
            },
            STRENGTH_DOWN => Status { 
                name: STRENGTH_DOWN,
                is_buff: false,
                effect_at: Event::TurnEnd,
                effect: Effect::SetStatus(STRENGTH, NegX, EffectTarget::_Self),
                ..Status::default()
            },
            VULNERABLE => Status { 
                name: VULNERABLE,
                is_buff: false,
                ..Status::default()
            },
            WEAK => Status { 
                name: WEAK,
                is_buff: false,
                ..Status::default()
            },
            BIAS => Status { 
                name: BIAS,
                is_buff: false,
                effect_at: Event::TurnStart,
                effect: Effect::SetStatus(FOCUS, NegX, EffectTarget::_Self),
                ..Status::default()
            },
            BLOCK_RETURN => Status { 
                name: BLOCK_RETURN,
                is_buff: false,
                effect_at: Event::OnAttackDamage,
                effect: Effect::Block(X, EffectTarget::Attacker),
                ..Status::default()
            },
            CHOKED => Status { 
                name: CHOKED,
                is_buff: false,
                effect_at: Event::OnCard(CardType::All),
                effect: Effect::LoseHp(X, EffectTarget::_Self),
                ..Status::default()
            },
            CONSTRICTED => Status { 
                name: CONSTRICTED,
                is_buff: false,
                effect_at: Event::TurnEnd,
                effect: Effect::Damage(X, EffectTarget::_Self),
                ..Status::default()
            },
            CORPSE_EXPLOSION => Status { 
                name: CORPSE_EXPLOSION,
                is_buff: false,
                ..Status::default()
            },
            DRAW_REDUCTION => Status { 
                name: DRAW_REDUCTION,
                is_buff: false,
                expire_at: Event::TurnEnd,
                ..Status::default()
            },
            ENTANGLED => Status { 
                name: ENTANGLED,
                is_buff: false,
                is_additive: false,
                expire_at: Event::TurnEnd,
                ..Status::default()
            },
            FASTING => Status { 
                name: FASTING,
                is_buff: false,
                effect_at: Event::TurnStart,
                effect: Effect::AddEnergy(NegX),
                ..Status::default()
            },
            HEX => Status { 
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
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..Status::default()
            },
            LOCK_ON => Status { 
                name: LOCK_ON,
                is_buff: false,
                reduce_at: Event::TurnStart,
                ..Status::default()
            },
            MARK => Status { 
                name: MARK,
                is_buff: false,
                ..Status::default()
            },
            NO_BLOCK => Status { 
                name: NO_BLOCK,
                is_buff: false,
                reduce_at: Event::TurnEnd,
                ..Status::default()
            },
            WRAITH_FORM => Status { 
                name: WRAITH_FORM,
                is_buff: false,
                effect_at: Event::TurnStart,
                effect: Effect::SetStatus(DEXTERITY, NegX, EffectTarget::_Self),
                ..Status::default()
            },
            _ => panic!("Unrecognized status name"),
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