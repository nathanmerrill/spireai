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
            on_add: Effect::None,
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
                effect_at: Event::Custom,
                effect: Effect::Custom,
                ..BaseBuff::default()
            },
            ANGRY => BaseBuff {
                name: ANGRY,
                effect_at: Event::AttackDamage(Target::_Self),
                effect: Effect::AddBuff(STRENGTH, X, Target::_Self),
                ..BaseBuff::default()
            },
            BARRICADE => BaseBuff { 
                name: BARRICADE,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::Custom,
                is_additive: false,
                ..BaseBuff::default()
            },
            BUFFER => BaseBuff { 
                name: BUFFER,
                effect_at: Event::HpLoss(Target::_Self),
                effect: Effect::Custom,
                ..BaseBuff::default()
            },
            DEXTERITY => BaseBuff { 
                name: DEXTERITY,
                ..BaseBuff::default()
            },
            DRAW_CARD => BaseBuff { 
                name: DRAW_CARD,
                effect_at: Event::BeforeHandDraw,
                expire_at: Event::BeforeHandDraw,
                effect: Effect::Draw(X),
                ..BaseBuff::default()
            },
            ENERGIZED => BaseBuff { 
                name: ENERGIZED,
                effect_at: Event::BeforeHandDraw,
                expire_at: Event::BeforeHandDraw,
                effect: Effect::AddEnergy(X),
                ..BaseBuff::default()
            },
            FOCUS => BaseBuff { 
                name: FOCUS,
                ..BaseBuff::default()
            },
            INTANGIBLE => BaseBuff { 
                name: INTANGIBLE,
                effect_at: Event::Damage(Target::_Self),
                effect: Effect::Custom,
                reduce_at: Event::BeforeHandDraw,
                ..BaseBuff::default()
            },
            MANTRA => BaseBuff { 
                name: MANTRA,
                ..BaseBuff::default()
            },
            METALLICIZE => BaseBuff { 
                name: METALLICIZE,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::Block(X, Target::_Self),
                ..BaseBuff::default()
            },
            NEXT_TURN_BLOCK => BaseBuff { 
                name: NEXT_TURN_BLOCK,
                effect_at: Event::BeforeHandDraw,
                expire_at: Event::BeforeHandDraw,
                effect: Effect::Block(X, Target::_Self),
                ..BaseBuff::default()
            },
            PAINFUL_STABS => BaseBuff {
                name: PAINFUL_STABS, 
                effect_at: Event::UnblockedDamage(Target::TargetEnemy),
                effect: Effect::AddCard {
                    card: CardReference::ByName(cards::WOUND), 
                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..BaseBuff::default()
            },
            PLATED_ARMOR => BaseBuff { 
                name: PLATED_ARMOR,
                effect_at: Event::BeforeHandDraw,
                reduce_at: Event::UnblockedDamage(Target::_Self),
                effect: Effect::Block(X, Target::_Self),
                ..BaseBuff::default()
            },
            RITUAL => BaseBuff { 
                name: RITUAL,
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::AddBuff(STRENGTH, X, Target::_Self),
                ..BaseBuff::default()
            },
            SPLIT => BaseBuff {
                name: SPLIT, 
                is_additive: false,
                ..BaseBuff::default()
            },
            STRENGTH => BaseBuff { 
                name: STRENGTH,
                ..BaseBuff::default()
            },
            STRENGTH_UP => BaseBuff { 
                name: STRENGTH_UP,
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::AddBuff(STRENGTH, X, Target::_Self),
                ..BaseBuff::default()
            },
            THORNS => BaseBuff { 
                name: THORNS,
                effect_at: Event::AttackDamage(Target::_Self),
                effect: Effect::Damage(X, Target::Attacker),
                ..BaseBuff::default()
            },
            VIGOR => BaseBuff { 
                name: VIGOR,
                effect_at: Event::PlayCard(CardType::Attack),
                effect: Effect::Custom,
                expire_at: Event::PlayCard(CardType::Attack),
                ..BaseBuff::default()
            },
            ACCURACY => BaseBuff { 
                name: ACCURACY,
                effect_at: Event::PlayCard(CardType::Attack),
                effect: Effect::Custom,
                ..BaseBuff::default()
            },
            AFTER_IMAGE => BaseBuff { 
                name: AFTER_IMAGE,
                effect_at: Event::PlayCard(CardType::All),
                effect: Effect::Block(X, Target::_Self),
                ..BaseBuff::default()
            },
            AMPLIFY => BaseBuff { 
                name: AMPLIFY,
                effect_at: Event::PlayCard(CardType::Power),
                effect: Effect::Custom,
                expire_at: Event::BeforeEnemyMove,
                reduce_at: Event::PlayCard(CardType::Power),
                ..BaseBuff::default()
            },
            ASLEEP => BaseBuff {
                name: ASLEEP,
                expire_at: Event::UnblockedDamage(Target::_Self),
                ..BaseBuff::default()
            },
            BATTLE_HYMN => BaseBuff { 
                name: BATTLE_HYMN,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::AddCard{
                    card: CardReference::ByName(cards::SMITE),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..BaseBuff::default()
            },
            BEAT_OF_DEATH => BaseBuff{
                name: BEAT_OF_DEATH,
                effect_at: Event::PlayCard(CardType::All),
                effect: Effect::Damage(X, Target::AllEnemies),
                ..BaseBuff::default()
            },
            BERSERK => BaseBuff { 
                name: BERSERK,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::AddEnergy(X),
                ..BaseBuff::default()
            },
            BLASPHEMER => BaseBuff { 
                name: BLASPHEMER,
                is_additive: false,
                effect_at: Event::BeforeHandDraw,
                expire_at: Event::BeforeHandDraw,
                effect: Effect::Damage(Fixed(9999), Target::_Self),
                ..BaseBuff::default()
            },
            BLUR => BaseBuff { 
                name: BLUR,
                reduce_at: Event::BeforeHandDraw,
                ..BaseBuff::default()
            },
            BRUTALITY => BaseBuff { 
                name: BRUTALITY,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::Multiple(
                    vec![
                        Effect::LoseHp(X, Target::_Self),
                        Effect::Draw(X),
                    ]),
                ..BaseBuff::default()
            },
            BURST => BaseBuff { 
                name: BURST,
                reduce_at: Event::PlayCard(CardType::Skill),
                ..BaseBuff::default()
            },
            CURL_UP => BaseBuff {
                name: CURL_UP,
                effect_at: Event::AttackDamage(Target::_Self),
                expire_at: Event::AttackDamage(Target::_Self),
                effect: Effect::Block(X, Target::_Self),
                ..BaseBuff::default()
            },
            CURIOSITY => BaseBuff {
                name: CURIOSITY,
                effect_at: Event::PlayCard(CardType::Power),
                effect: Effect::AddBuff(STRENGTH, X, Target::_Self),
                ..BaseBuff::default()
            },
            COLLECT => BaseBuff { 
                name: COLLECT,
                effect_at: Event::BeforeHandDraw,
                reduce_at: Event::BeforeHandDraw,
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
                on_add: Effect::AddN(Fixed(1)),
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::Multiple(vec![
                    Effect::LoseHp(Amount::N, Target::_Self),
                    Effect::Damage(Amount::X, Target::AllEnemies),
                ]),
                ..BaseBuff::default()
            },
            CORRUPTION => BaseBuff { 
                name: CORRUPTION,
                is_additive: false,
                ..BaseBuff::default()
            },
            CREATIVE_AI => BaseBuff { 
                name: CREATIVE_AI,
                effect_at: Event::BeforeHandDraw,
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
                effect_at: Event::BeforeHandDraw,
                effect: Effect::AddBuff(STRENGTH, X, Target::_Self),
                ..BaseBuff::default()
            },
            DEVA => BaseBuff { 
                name: DEVA,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::Multiple(vec![
                    Effect::AddN(X),
                    Effect::AddEnergy(N),
                ]),
                ..BaseBuff::default()
            },
            DEVOTION => BaseBuff { 
                name: DEVOTION,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::AddBuff(MANTRA, X, Target::_Self),
                ..BaseBuff::default()
            },
            DOUBLE_DAMAGE => BaseBuff { 
                name: DOUBLE_DAMAGE,
                reduce_at: Event::BeforeEnemyMove,
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
            ENRAGE => BaseBuff {
                name: ENRAGE,
                effect_at: Event::PlayCard(CardType::Attack),
                effect: Effect::AddBuff(STRENGTH, X, Target::_Self),
                ..BaseBuff::default()
            },
            ENVENOM => BaseBuff { 
                name: ENVENOM,
                effect_at: Event::UnblockedDamage(Target::TargetEnemy),
                effect: Effect::AddBuff(POISON, X, Target::TargetEnemy),
                ..BaseBuff::default()
            },
            EQUILIBRIUM => BaseBuff { 
                name: EQUILIBRIUM,
                reduce_at: Event::BeforeHandDraw,
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
            EXPLODE => BaseBuff {
                name: EXPLODE,
                reduce_at: Event::AttackDamage(Target::TargetEnemy),
                ..BaseBuff::default()
            },
            FADING => BaseBuff { 
                name: FADING,
                reduce_at: Event::AfterEnemyMove,
                effect_at: Event::UnBuff(FADING, Target::_Self),
                effect: Effect::Die(Target::_Self),
                ..BaseBuff::default()
            },
            FEEL_NO_PAIN => BaseBuff { 
                name: FEEL_NO_PAIN,
                effect_at: Event::Exhaust,
                effect: Effect::Block(X, Target::_Self),
                ..BaseBuff::default()
            },
            FLAME_BARRIER => BaseBuff {
                name: FLAME_BARRIER,
                effect_at: Event::AttackDamage(Target::_Self),
                effect: Effect::Damage(X, Target::Attacker),
                expire_at: Event::BeforeHandDraw,
                ..BaseBuff::default()
            },
            FLYING => BaseBuff {
                name: FLYING,
                effect_at: Event::AttackDamage(Target::_Self),
                effect: Effect::Custom,
                ..BaseBuff::default()
            },
            FIRE_BREATHING => BaseBuff { 
                name: FIRE_BREATHING,
                effect_at: Event::Multiple(vec![Event::DrawCard(CardType::Status), Event::DrawCard(CardType::Status)]),
                effect: Effect::Damage(X, Target::AllEnemies),
                ..BaseBuff::default()
            },
            FORESIGHT => BaseBuff { 
                name: FORESIGHT,
                effect_at: Event::BeforeHandDraw,
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
                effect_at: Event::BeforeHandDraw,
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
                effect_at: Event::BeforeHandDraw,
                effect: Effect::AddCard{
                    card: CardReference::ByName(cards::SHIV),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..BaseBuff::default()
            },
            INNATE_THIEVERY => BaseBuff { 
                name: INNATE_THIEVERY,
                effect_at: Event::Damage(Target::TargetEnemy),
                effect: Effect::Custom,
                ..BaseBuff::default()
            },
            INVINCIBLE => BaseBuff {
                name: INVINCIBLE,
                effect_at: Event::HpLoss(Target::_Self),
                effect: Effect::Custom,
                ..BaseBuff::default()
            },
            JUGGERNAUT => BaseBuff { 
                name: JUGGERNAUT,
                effect_at: Event::Block(Target::_Self),
                effect: Effect::Damage(X, Target::RandomEnemy),
                ..BaseBuff::default()
            },
            LIFE_LINK => BaseBuff {
                name: LIFE_LINK,
                is_additive: false,
                effect_at: Event::Die(Target::_Self),
                effect: Effect::Custom,
                ..BaseBuff::default()
            },
            LIKE_WATER => BaseBuff { 
                name: LIKE_WATER,
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::If(Condition::Stance(Stance::Calm), vec![Effect::Block(X, Target::_Self)], vec![]),
                ..BaseBuff::default()
            },
            LOOP => BaseBuff { 
                name: LOOP,
                ..BaseBuff::default()
            },
            MACHINE_LEARNING => BaseBuff { 
                name: MACHINE_LEARNING,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::Draw(X),
                ..BaseBuff::default()
            },
            MALLEABLE => BaseBuff {
                name: MALLEABLE,
                effect_at: Event::Custom,
                effect: Effect::Custom,
                ..BaseBuff::default()
            },
            MAGNETISM => BaseBuff { 
                name: MAGNETISM,
                effect_at: Event::BeforeHandDraw,
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
                effect_at: Event::BeforeHandDraw,
                effect: Effect::AutoPlayCard(CardLocation::DrawPile(RelativePosition::Top)),
                ..BaseBuff::default()
            },
            MENTAL_FORTRESS => BaseBuff { 
                name: MENTAL_FORTRESS,
                effect_at: Event::StanceChange(Stance::All, Stance::All),
                effect: Effect::Block(X, Target::_Self),
                ..BaseBuff::default()
            },
            MODE_SHIFT => BaseBuff { 
                name: MODE_SHIFT,
                effect_at: Event::UnblockedDamage(Target::_Self),
                effect: Effect::Custom,
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
                effect: Effect::Block(X, Target::_Self),
                ..BaseBuff::default()
            },
            NOXIOUS_FUMES => BaseBuff { 
                name: NOXIOUS_FUMES,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::AddBuff(POISON, X, Target::AllEnemies),
                ..BaseBuff::default()
            },
            OMEGA => BaseBuff { 
                name: OMEGA,
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::Damage(X, Target::AllEnemies),
                ..BaseBuff::default()
            },
            PANACHE => BaseBuff { 
                name: PANACHE,
                on_add: Effect::SetN(Fixed(5)),
                effect_at: Event::PlayCard(CardType::All),
                effect: Effect::Multiple(vec![
                    Effect::AddN(Fixed(-1)),
                    Effect::If(Condition::NEquals(Fixed(0)), vec![
                        Effect::ResetN,
                        Effect::Damage(X, Target::AllEnemies),
                    ], vec![]),
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
                reduce_at: Event::BeforeHandDraw,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::AddBuff(DOUBLE_DAMAGE, Fixed(1), Target::_Self),
                ..BaseBuff::default()
            },
            RAGE => BaseBuff { 
                name: RAGE,
                effect_at: Event::PlayCard(CardType::Attack),
                effect: Effect::Block(X, Target::_Self),
                ..BaseBuff::default()
            },
            REACTIVE => BaseBuff {
                name: REACTIVE,
                effect_at: Event::UnblockedDamage(Target::_Self),
                effect: Effect::Custom,
                ..BaseBuff::default()
            },
            REBOUND => BaseBuff { 
                name: REBOUND,
                reduce_at: Event::PlayCard(CardType::All),
                ..BaseBuff::default()
            },
            REGENERATION => BaseBuff { 
                name: REGENERATION,
                reduce_at: Event::BeforeEnemyMove,
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::Heal(X, Target::_Self),
                ..BaseBuff::default()
            },
            REGENERATE => BaseBuff { 
                name: REGENERATE,
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::Heal(X, Target::_Self),
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
                effect: Effect::Heal(X, Target::_Self),
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
            SHARP_HIDE => BaseBuff {
                name: SHARP_HIDE,
                effect_at: Event::Custom,
                effect: Effect::Custom,
                ..BaseBuff::default()
            },
            SHIFTING => BaseBuff {
                name: SHIFTING,
                effect_at: Event::PlayCard(CardType::Attack),
                effect: Effect::Damage(X, Target::Attacker),
                ..BaseBuff::default()
            },
            SIMMERING_RAGE => BaseBuff { 
                name: SIMMERING_RAGE,
                is_additive: false,
                expire_at: Event::BeforeHandDraw,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::SetStance(Stance::Wrath),
                ..BaseBuff::default()
            },
            STASIS => BaseBuff {
                name: STASIS,
                is_additive: false,
                effect_at: Event::Die(Target::_Self),
                effect: Effect::Custom,
                ..BaseBuff::default()
            },
            STATIC_DISCHARGE => BaseBuff { 
                name: STATIC_DISCHARGE,
                effect_at: Event::UnblockedDamage(Target::_Self),
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
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::AddCard{
                    card: CardReference::ByName(cards::INSIGHT),
                    destination: CardLocation::DrawPile(RelativePosition::Random), 
                    copies: X,
                    modifier: CardModifier::None,
                },
                ..BaseBuff::default()
            },
            SPORE_CLOUD => BaseBuff {
                name: SPORE_CLOUD,
                effect_at: Event::Die(Target::_Self),
                expire_at: Event::Die(Target::_Self),
                effect: Effect::AddBuff(VULNERABLE, X, Target::AllEnemies),
                ..BaseBuff::default()
            },
            SURROUNDED => BaseBuff { 
                name: SURROUNDED,
                is_additive: false,
                ..BaseBuff::default()
            },
            TIME_WARP => BaseBuff { 
                name: TIME_WARP,
                is_additive: false,
                reduce_at: Event::PlayCard(CardType::All),
                effect_at: Event::UnBuff(TIME_WARP, Target::_Self),
                effect: Effect::Custom,
                stacks: false,
                ..BaseBuff::default()
            },
            THE_BOMB => BaseBuff { 
                name: THE_BOMB,
                is_additive: false,
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::Custom,
                stacks: false,
                ..BaseBuff::default()
            },
            THOUSAND_CUTS => BaseBuff { 
                name: THOUSAND_CUTS,
                effect_at: Event::PlayCard(CardType::All),
                effect: Effect::Damage(X, Target::AllEnemies),
                ..BaseBuff::default()
            },
            TOOLS_OF_THE_TRADE => BaseBuff { 
                name: TOOLS_OF_THE_TRADE,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::Multiple(
                    vec![
                        Effect::Draw(X),
                        Effect::DiscardCard(CardLocation::PlayerHand(RelativePosition::PlayerChoice(Fixed(1))))
                    ]
                ),
                ..BaseBuff::default()
            },
            WAVE_OF_THE_HAND => BaseBuff { 
                name: WAVE_OF_THE_HAND,
                effect_at: Event::Block(Target::_Self),
                effect: Effect::AddBuff(WEAK, X, Target::AllEnemies),
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
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::AddBuff(DEXTERITY, NegX, Target::_Self),
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
                expire_at: Event::BeforeEnemyMove,
                ..BaseBuff::default()
            },
            POISON => BaseBuff { 
                name: POISON,
                is_buff: false,
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::LoseHp(X, Target::_Self),
                ..BaseBuff::default()
            },
            SHACKLED => BaseBuff { 
                name: SHACKLED,
                is_buff: false,
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::AddBuff(STRENGTH, X, Target::_Self),
                ..BaseBuff::default()
            },
            SLOW => BaseBuff { 
                name: SLOW,
                is_buff: false,
                effect_at: Event::PlayCard(CardType::All),
                effect: Effect::Custom,
                ..BaseBuff::default()
            },
            STRENGTH_DOWN => BaseBuff { 
                name: STRENGTH_DOWN,
                is_buff: false,
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::AddBuff(STRENGTH, NegX, Target::_Self),
                ..BaseBuff::default()
            },
            VULNERABLE => BaseBuff { 
                name: VULNERABLE,
                is_buff: false,
                reduce_at: Event::BeforeHandDraw,
                ..BaseBuff::default()
            },
            WEAK => BaseBuff { 
                name: WEAK,
                is_buff: false,
                reduce_at: Event::BeforeHandDraw,
                ..BaseBuff::default()
            },
            BIAS => BaseBuff { 
                name: BIAS,
                is_buff: false,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::AddBuff(FOCUS, NegX, Target::_Self),
                ..BaseBuff::default()
            },
            BLOCK_RETURN => BaseBuff { 
                name: BLOCK_RETURN,
                is_buff: false,
                effect_at: Event::AttackDamage(Target::_Self),
                effect: Effect::Block(X, Target::Attacker),
                ..BaseBuff::default()
            },
            CHOKED => BaseBuff { 
                name: CHOKED,
                is_buff: false,
                expire_at: Event::BeforeHandDraw,
                effect_at: Event::PlayCard(CardType::All),
                effect: Effect::LoseHp(X, Target::_Self),
                ..BaseBuff::default()
            },
            CONSTRICTED => BaseBuff { 
                name: CONSTRICTED,
                is_buff: false,
                effect_at: Event::BeforeEnemyMove,
                effect: Effect::Damage(X, Target::_Self),
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
                expire_at: Event::BeforeEnemyMove,
                ..BaseBuff::default()
            },
            ENTANGLED => BaseBuff { 
                name: ENTANGLED,
                is_buff: false,
                is_additive: false,
                expire_at: Event::BeforeEnemyMove,
                ..BaseBuff::default()
            },
            FASTING => BaseBuff { 
                name: FASTING,
                is_buff: false,
                effect_at: Event::BeforeHandDraw,
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
                reduce_at: Event::BeforeHandDraw,
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
                reduce_at: Event::BeforeEnemyMove,
                ..BaseBuff::default()
            },
            WRAITH_FORM => BaseBuff { 
                name: WRAITH_FORM,
                is_buff: false,
                effect_at: Event::BeforeHandDraw,
                effect: Effect::AddBuff(DEXTERITY, NegX, Target::_Self),
                ..BaseBuff::default()
            },
            _ => panic!("Unrecognized BaseBuff name"),
        }
    }
}

pub const ACCURACY: &str = "Accuracy";
pub const AFTER_IMAGE: &str = "After Image";
pub const AMPLIFY: &str = "Amplify";
pub const ANGRY: &str = "Angry";
pub const ARTIFACT: &str = "Artifact";
pub const ASLEEP: &str = "Asleep";
pub const BARRICADE: &str = "Barricade";
pub const BATTLE_HYMN: &str = "Battle Hymn";
pub const BEAT_OF_DEATH: &str = "Beat Of Death";
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
pub const CURIOSITY: &str = "Curiosity";
pub const CURL_UP: &str = "Curl Up";
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
pub const ENRAGE: &str = "Enraged";
pub const ELECTRO: &str = "Electro";
pub const ENTANGLED: &str = "Entangled";
pub const ENVENOM: &str = "Envenom";
pub const EQUILIBRIUM: &str = "Equilibrium";
pub const ESTABLISHMENT: &str = "Establishment";
pub const EVOLVE: &str = "Evolve";
pub const EXPLODE: &str = "Explode";
pub const FADING: &str = "Fading";
pub const FASTING: &str = "Fasting";
pub const FEEL_NO_PAIN: &str = "Feel No Pain";
pub const FIRE_BREATHING: &str = "Fire Breathing";
pub const FLAME_BARRIER: &str = "Flame Barrier";
pub const FLYING: &str = "Flying";
pub const FOCUS: &str = "Focus";
pub const FORESIGHT: &str = "Foresight";
pub const FRAIL: &str = "Frail";
pub const FREE_ATTACK_POWER: &str = "Free Attack Power";
pub const HEATSINK: &str = "Heatsink";
pub const HELLO: &str = "Hello";
pub const HEX: &str = "Hex";
pub const INFINITE_BLADES: &str = "Infinite Blades";
pub const INNATE_THIEVERY: &str = "Innate Thievery";
pub const INTANGIBLE: &str = "Intangible";
pub const INVINCIBLE: &str = "Invincible";
pub const JUGGERNAUT: &str = "Juggernaut";
pub const LIFE_LINK: &str = "Life Link";
pub const LIKE_WATER: &str = "Like Water";
pub const LOCK_ON: &str = "Lock-On";
pub const LOOP: &str = "Loop";
pub const MACHINE_LEARNING: &str = "Machine Learning";
pub const MAGNETISM: &str = "Magnetism";
pub const MALLEABLE: &str = "Malleable";
pub const MANTRA: &str = "Mantra";
pub const MARK: &str = "Mark";
pub const MASTER_REALITY: &str = "Master Reality";
pub const MAYHEM: &str = "Mayhem";
pub const MENTAL_FORTRESS: &str = "Mental Fortress";
pub const METALLICIZE: &str = "Metallicize";
pub const MODE_SHIFT: &str = "Metallicize";
pub const NEXT_TURN_BLOCK: &str = "Next Turn Block";
pub const NIGHTMARE: &str = "Nightmare";
pub const NIRVANA: &str = "Nirvana";
pub const NO_BLOCK: &str = "No Block";
pub const NO_DRAW: &str = "No Draw";
pub const NOXIOUS_FUMES: &str = "Noxious Fumes";
pub const OMEGA: &str = "Omega";
pub const PAINFUL_STABS: &str = "Painful Stabs";
pub const PANACHE: &str = "Panache";
pub const PEN_NIB: &str = "Pen Nib";
pub const PLATED_ARMOR: &str = "Plated Armor";
pub const PHANTASMAL: &str = "Phantasmal";
pub const POISON: &str = "Poison";
pub const RAGE: &str = "Rage";
pub const REACTIVE: &str = "Reactive";
pub const REBOUND: &str = "Rebound";
pub const REGENERATION: &str = "Regeneration";
pub const REGENERATE: &str = "Regenerate";
pub const REPAIR: &str = "Repair";
pub const RITUAL: &str = "Ritual";
pub const RUSHDOWN: &str = "Rushdown";
pub const RUPTURE: &str = "Rupture";
pub const SADISTIC: &str = "Sadistic";
pub const SHACKLED: &str = "Shackled";
pub const SHARP_HIDE: &str = "Sharp Hide";
pub const SHIFTING: &str = "Shifting";
pub const SIMMERING_RAGE: &str = "Simmering Rage";
pub const SLOW: &str = "Slow";
pub const SPORE_CLOUD: &str = "Spore Cloud";
pub const SPLIT: &str = "Split";
pub const STASIS: &str = "Stasis";
pub const STATIC_DISCHARGE: &str = "Static Discharge";
pub const STORM: &str = "Storm";
pub const STRENGTH: &str = "Strength";
pub const STRENGTH_DOWN: &str = "Strength Down";
pub const STRENGTH_UP: &str = "Strength Up";
pub const STUDY: &str = "Study";
pub const SURROUNDED: &str = "Surrounded";
pub const TIME_WARP: &str = "Time Warp";
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