use im::{HashMap, Vector};
use uuid::Uuid;

use crate::{
    models::{
        self,
        buffs::BaseBuff,
        cards::BaseCard,
        core::{Amount, CardLocation, CardType, Condition, OrbType, When, FightType, DeckOperation},
        monsters::{BaseMonster, Intent, MonsterMove},
        potions::BasePotion,
        relics::BaseRelic,
    },
    spireai::references::{
        BuffReference, CardReference, CreatureReference, PotionReference, RelicReference, MonsterReference,
    },
};

use super::probability::Probability;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Vars {
    pub n: i16,
    pub n_reset: i16,
    pub x: i16,
}

impl Vars {
    pub fn new() -> Vars {
        Vars {
            n: 0,
            n_reset: 0,
            x: 0,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Orb {
    pub base: OrbType,
    pub n: u16,
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct HpRange {
    pub amount: u16,
    pub max: u16
}

impl HpRange {
    pub fn reduce_max_hp(&mut self, reduction: u16) {
        self.max -= reduction;
        self.amount = self.max.min(self.amount);
    }

    pub fn add(&mut self, amount: f64) {
        self.amount = self.max.min((amount - 0.0001).ceil() as u16 + self.amount)
    }

    pub fn new(amount: u16) -> Self {
        Self {
            amount,
            max: amount
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Creature {
    pub hp: HpRange,
    pub monster: Option<MonsterReference>,
    pub buffs_when: HashMap<When, Vector<Uuid>>,
    pub buff_names: HashMap<String, Uuid>,
    pub buffs: HashMap<Uuid, Buff>,
    pub block: u16,
}

impl Creature {
    pub fn add_buff(&mut self, name: &str, amount: i16) {
        if let Some(uuid) = self.buff_names.get(name) {
            let buff = self.buffs.get_mut(uuid).unwrap();
            if !buff.base.repeats {
                if !buff.base.singular {
                    buff.vars.x += amount
                }
            } else {
                buff.stacked_vars.push(Vars {
                    n: 0,
                    n_reset: 0,
                    x: amount,
                })
            }
        } else {
            let new_buff = Buff::by_name(name, amount);
            for effects in &new_buff.base.effects {
                self.buffs_when
                    .entry(effects.when.clone())
                    .or_insert_with(Vector::new)
                    .push_back(new_buff.uuid)
            }
            self.buff_names.insert(name.to_string(), new_buff.uuid);
            self.buffs.insert(new_buff.uuid, new_buff);
        }
    }

    pub fn creature_ref(&self) -> CreatureReference 
    {
        match self.monster {
            Some(_ref) => CreatureReference::Creature(_ref),
            None => CreatureReference::Player
        }
    }

    pub fn is_player(&self) -> bool {
        self.monster.is_none()
    }

    pub fn buffs(&self) -> impl Iterator<Item = BuffReference> + '_ {
        self.buffs.values().map(move |b| BuffReference {
            base: b.base,
            creature: self.creature_ref(),
            buff: b.uuid,
        })
    }

    pub fn has_buff(&self, name: &str) -> bool {
        self.buff_names.contains_key(name)
    }

    pub fn player(hp: HpRange) -> Creature {
        Creature {
            hp,
            monster: None,
            buffs_when: HashMap::new(),
            buff_names: HashMap::new(),
            buffs: HashMap::new(),
            block: 0,
        }
    }

    pub fn monster(hp: HpRange, monster: MonsterReference) -> Creature {
        Creature {
            hp,
            monster: Some(monster),
            buffs_when: HashMap::new(),
            buff_names: HashMap::new(),
            buffs: HashMap::new(),
            block: 0,
        }
    }

    pub fn get_buff_amount(&self, name: &str) -> i16 {
        self.find_buff(name).map_or(0, |b| b.vars.n)
    }

    pub fn find_buff(&self, name: &str) -> Option<&Buff> {
        self.buff_names.get(name).map(|u| &self.buffs[u])
    }
    pub fn find_buff_mut(&mut self, name: &str) -> Option<&mut Buff> {
        let uuid = self.buff_names.get(name).copied();
        match uuid {
            Some(u) => self.buffs.get_mut(&u),
            None => None,
        }
    }

    pub fn remove_buff(&mut self, buff: BuffReference) {
        self.remove_buff_by_uuid(buff.buff);
    }

    pub fn remove_buff_by_name(&mut self, name: &str) {
        if let Some(uuid) = self.buff_names.get(name).copied() {
            self.remove_buff_by_uuid(uuid)
        };
    }

    pub fn remove_buff_by_uuid(&mut self, uuid: Uuid) {
        let removed = self.buffs.remove(&uuid).unwrap();
        self.buff_names.remove(&removed.base.name);

        for (_, uuids) in self.buffs_when.iter_mut() {
            if let Some(index) = uuids.index_of(&uuid) {
                uuids.remove(index);
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct CardOffer {
    pub base: &'static BaseCard,
    pub upgraded: bool,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Card {
    pub base: &'static BaseCard,
    pub cost: u8,
    pub base_cost: u8,
    pub cost_until_played: bool,
    pub uuid: Uuid,
    pub vars: Vars,
    pub retain: bool,
    pub upgrades: u8,
    pub bottled: bool,
}

impl Card {
    pub fn duplicate(&self) -> Self {
        let mut card = self.clone();
        card.uuid = Uuid::new_v4();
        card.bottled = false;
        card
    }

    pub fn is_innate(&self) -> bool {
        self.bottled
            || match self.base.innate {
                Condition::Always => true,
                Condition::Never => false,
                Condition::Upgraded => self.upgrades > 0,
                _ => panic!("Unexpected innate condition"),
            }
    }

    pub fn by_name(name: &str) -> Self {
        Self::new(models::cards::by_name(name))
    }

    pub fn new(base: &'static BaseCard) -> Self {
        let uuid = Uuid::new_v4();

        let cost = match base.cost {
            Amount::Fixed(cost) => cost as u8,
            Amount::Upgradable { amount, .. } => amount as u8,
            Amount::X => 0,
            Amount::Custom => match base.name.as_str() {
                "Blood for Blood" => 4,
                "Eviscerate" => 3,
                "Force Field" => 4,
                _ => panic!("Custom cost amount on an unknown card"),
            },
            _ => panic!("Unexpected cost amount"),
        };

        let retain = match base.retain {
            Condition::Always => true,
            Condition::Never => false,
            Condition::Upgraded => false,
            _ => panic!("Unexpected retain condition"),
        };

        Card {
            base,
            uuid,
            base_cost: cost,
            cost,
            cost_until_played: false,
            retain,
            vars: Vars::new(),
            upgrades: 0,
            bottled: false,
        }
    }

    pub fn removable(&self) -> bool {
        if self.bottled {
            return false;
        }

        !(self.base.name == "Ascender's Bane"
            || self.base.name == "Curse of the Bell"
            || self.base.name == "Necronomicurse")
    }

    pub fn upgradable(&self) -> bool {
        match self.base._type {
            CardType::Attack | CardType::Skill | CardType::Power => {
                self.upgrades == 0 && self.base.name != "Searing Blow"
            }
            CardType::Status => false,
            CardType::Curse => false,
            CardType::All => panic!("Unexpected All on card type"),
        }
    }

    pub fn reference(&self, location: CardLocation) -> CardReference {
        CardReference {
            base: self.base,
            uuid: self.uuid,
            location,
        }
    }

    pub fn targeted(&self) -> bool {
        match &self.base.targeted {
            Condition::Never => false,
            Condition::Always => true,
            Condition::Not(b) => {
                if b.as_ref() == &Condition::Upgraded {
                    self.upgrades > 0
                } else {
                    panic!("Unexpected condition!")
                }
            }
            _ => panic!("Unexpected condition!"),
        }
    }

    pub fn upgrade(&mut self) {
        match self.base._type {
            CardType::Status | CardType::Curse => {}
            _ => {
                if self.upgrades == 0 || self.base.name == "Searing Blow" {
                    self.upgrades += 1;

                    if let Amount::Upgradable { upgraded, .. } = self.base.cost {
                        let diff = self.base_cost - upgraded as u8;
                        self.base_cost -= diff;
                        self.cost = self.base_cost.saturating_sub(diff);
                    }

                    if let Condition::Upgraded = self.base.retain {
                        self.retain = true;
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Buff {
    pub base: &'static BaseBuff,
    pub uuid: Uuid,
    pub vars: Vars,
    pub stacked_vars: Vec<Vars>,
    pub card_stasis: Option<Uuid>,
}

impl Buff {
    pub fn by_name(name: &str, amount: i16) -> Self {
        Self::new(models::buffs::by_name(name), amount)
    }

    pub fn new(base: &'static BaseBuff, amount: i16) -> Self {
        if !base.repeats {
            Buff {
                base,
                uuid: Uuid::new_v4(),
                vars: Vars {
                    n: 0,
                    n_reset: 0,
                    x: amount,
                },
                stacked_vars: vec![],
                card_stasis: None,
            }
        } else {
            Buff {
                base,
                uuid: Uuid::new_v4(),
                vars: Vars::new(),
                stacked_vars: vec![Vars {
                    n: 0,
                    n_reset: 0,
                    x: amount,
                }],
                card_stasis: None,
            }
        }
    }

    pub fn reference(&self, creature: CreatureReference) -> BuffReference {
        BuffReference {
            base: self.base,
            creature,
            buff: self.uuid,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Relic {
    pub base: &'static BaseRelic,
    pub uuid: Uuid,
    pub vars: Vars,
    pub enabled: bool,
}

impl Relic {
    pub fn reference(&self) -> RelicReference {
        RelicReference {
            base: self.base,
            relic: self.uuid,
        }
    }

    pub fn new(base: &'static BaseRelic) -> Self {
        let uuid = Uuid::new_v4();
        let mut relic = Relic {
            base,
            uuid,
            vars: Vars::new(),
            enabled: true,
        };
        relic.vars.x = base.starting_x;
        relic
    }

    pub fn by_name(name: &str) -> Self {
        Self::new(models::relics::by_name(name))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Monster {
    pub base: &'static BaseMonster,
    pub uuid: Uuid,
    pub creature: Creature,
    pub position: usize,
    pub targetable: bool,
    pub intent: Intent,
    pub vars: Vars,
    pub whens: HashMap<When, &'static String>,
    pub phase: usize,
    pub index: usize,
    pub current_move_options: Vector<(&'static MonsterMove, u8)>,
    pub last_move: Option<&'static MonsterMove>,
    pub last_move_count: u8,
}

impl Monster {
    pub fn with_hp(name: &str, max_hp: u16) -> Self {
        Self::create(models::monsters::by_name(name), max_hp)
    }
        
    pub fn new(name: &str, asc: u8, probability: &mut Probability) -> Self {
        let base = crate::models::monsters::by_name(name);
        let upgrade_asc = match base.fight_type {
            FightType::Common => 7,
            FightType::Elite { .. } => 8,
            FightType::Boss => 9,
        };

        let hp_range = if asc >= upgrade_asc {
            &base.hp_range_asc
        } else {
            &base.hp_range
        };

        let hp = probability
            .range((hp_range.max - hp_range.min + 1) as usize) as u16
            + hp_range.min;

        Monster::create(base, hp)
    }

    pub fn create(base: &'static BaseMonster, max_hp: u16) -> Self {
        let uuid = Uuid::new_v4();
        let reference = MonsterReference {
            base,
            uuid,
        };

        Monster {
            base,
            uuid,
            creature: Creature::monster(HpRange::new(max_hp), reference),
            position: 0,
            targetable: true,
            intent: Intent::None,
            vars: Vars::new(),
            whens: HashMap::new(),
            phase: 0,
            index: 0,
            current_move_options: Vector::new(),
            last_move: None,
            last_move_count: 0,
        }
    }
}

impl BasePotion {
    pub fn reference(&'static self, index: usize) -> PotionReference {
        PotionReference {
            base: self,
            index,
        }
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct RewardState {
    pub rewards: Vector<Reward>,
    pub deck_operation: Option<DeckOperation>,
    pub viewing_reward: Option<usize>,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Reward {
    CardChoice(Vector<CardOffer>, Option<FightType>, bool), // True if colorless
    Gold(u16),
    Relic(&'static BaseRelic),
    Potion(&'static BasePotion),
    EmeraldKey,
    SapphireKey,
    SapphireLinkedRelic(&'static BaseRelic),
}