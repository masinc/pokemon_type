use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Debug;

const TYPE_COUNT: usize = 18;

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum Type {
    Normal,
    Fight,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}

impl Eq for Type {}

pub enum Localization {
    English,
    Japanese,
}

#[derive(Debug, PartialEq, Hash, Clone)]
struct Combat<'a> {
    attack: &'a Type,
    block: &'a Type,
}

impl<'a> Eq for Combat<'a> {}

impl<'a> Combat<'a> {
    fn new(attack: &'a Type, block: &'a Type) -> Self {
        Self {
            attack: attack,
            block: block,
        }
    }
}

impl Type {
    pub fn types() -> [Type; TYPE_COUNT] {
        use Type::*;
        [
            Normal, Fight, Flying, Poison, Ground, Rock, Bug, Ghost, Steel, Fire, Water, Grass,
            Electric, Psychic, Ice, Dragon, Dark, Fairy,
        ]
    }

    pub fn from_str(s: &str) -> Option<Type> {
        for ty in Type::types().iter() {
            if ty.to_str(&Localization::English) == s {
                return Some(*ty);
            }
        }
        None
    }

    pub fn blocks_rates(block_types: &[&Type]) -> HashMap<Type, f64> {
        block_types
            .iter()
            .flat_map(|block_type| block_type.block_rates())
            .into_group_map()
            .into_iter()
            .map(|(key, v)| (key.clone(), v.iter().fold(1.0_f64, |acc, x| acc * x)))
            .collect()
    }

    pub fn combat1(atack_type: &Type, block_type: &Type) -> f64 {
        let combat = Combat::new(&atack_type, &block_type);
        let r = TYPE_EFFECT.get(&combat).unwrap();
        *r
    }

    pub fn combat_n(attack_type: &Type, block_types: &[&Type]) -> f64 {
        block_types
            .iter()
            .map(|block_type| Type::combat1(attack_type, block_type))
            .fold(1.0, |a, b| a * b)
    }

    pub fn attack_rate1(&self, block_type: &Type) -> f64 {
        Type::combat1(self, block_type)
    }

    pub fn attack_rate_n(&self, block_types: &[&Type]) -> f64 {
        Self::combat_n(self, block_types)
    }

    pub fn attack_rates(&self) -> HashMap<Type, f64> {
        Type::types()
            .iter()
            .map(|block_type| (*block_type, self.attack_rate1(block_type)))
            .collect()
    }

    pub fn block_rates(&self) -> HashMap<Type, f64> {
        Type::types()
            .iter()
            .map(|attack_type| (*attack_type, attack_type.attack_rate1(self)))
            .collect()
    }

    pub fn to_str(&self, l10n: &Localization) -> &'static str {
        match l10n {
            &Localization::Japanese => TYPE_STR_JAPANESE.get(&self).unwrap(),
            &Localization::English => TYPE_STR_ENGLISH.get(&self).unwrap(),
        }
    }
}

lazy_static! {
    static ref TYPE_STR_JAPANESE: HashMap<Type, &'static str> = {
        use Type::*;
        hashmap! {
            Normal => "ノーマル",
            Fight  => "格闘",
            Flying => "飛行",
            Poison => "毒",
            Ground => "地面",
            Rock => "岩",
            Bug => "虫",
            Ghost => "ゴースト",
            Steel => "鋼",
            Fire => "炎",
            Water => "水",
            Grass => "草",
            Electric => "雷",
            Psychic => "エスパー",
            Ice => "氷",
            Dragon => "ドラゴン",
            Dark => "悪",
            Fairy => "フェアリー",
        }
    };

    static ref TYPE_STR_ENGLISH: HashMap<Type, &'static str> = {
        use Type::*;
        hashmap! {
            Normal => "Normal",
            Fight  => "Fight",
            Flying => "Flying",
            Poison => "Poison",
            Ground => "Ground",
            Rock => "Rock",
            Bug => "Bug",
            Ghost => "Ghost",
            Steel => "Steel",
            Fire => "Fire",
            Water => "Water",
            Grass => "Grass",
            Electric => "Electric",
            Psychic => "Psychic",
            Ice => "Ice",
            Dragon => "Dragon",
            Dark => "Dark",
            Fairy => "Fairy",
        }
    };


    static ref TYPE_EFFECT: HashMap<Combat<'static>, f64> = {
        use Type::*;
        hashmap! {
            // atack: normal
            Combat::new(&Normal, &Normal)      => 1.0,
            Combat::new(&Normal, &Fight)       => 1.0,
            Combat::new(&Normal, &Flying)      => 1.0,
            Combat::new(&Normal, &Poison)      => 1.0,
            Combat::new(&Normal, &Ground)      => 1.0,
            Combat::new(&Normal, &Rock)        => 0.5,
            Combat::new(&Normal, &Bug)         => 1.0,
            Combat::new(&Normal, &Ghost)       => 0.0,
            Combat::new(&Normal, &Steel)       => 0.5,
            Combat::new(&Normal, &Fire)        => 1.0,
            Combat::new(&Normal, &Water)       => 1.0,
            Combat::new(&Normal, &Grass)       => 1.0,
            Combat::new(&Normal, &Electric)    => 1.0,
            Combat::new(&Normal, &Psychic)     => 1.0,
            Combat::new(&Normal, &Ice)         => 1.0,
            Combat::new(&Normal, &Dragon)      => 1.0,
            Combat::new(&Normal, &Dark)        => 1.0,
            Combat::new(&Normal, &Fairy)       => 1.0,

            // atack: fight
            Combat::new(&Fight, &Normal)      => 2.0,
            Combat::new(&Fight, &Fight)       => 1.0,
            Combat::new(&Fight, &Flying)      => 0.5,
            Combat::new(&Fight, &Poison)      => 0.5,
            Combat::new(&Fight, &Ground)      => 1.0,
            Combat::new(&Fight, &Rock)        => 2.0,
            Combat::new(&Fight, &Bug)         => 0.5,
            Combat::new(&Fight, &Ghost)       => 0.0,
            Combat::new(&Fight, &Steel)       => 2.0,
            Combat::new(&Fight, &Fire)        => 1.0,
            Combat::new(&Fight, &Water)       => 1.0,
            Combat::new(&Fight, &Grass)       => 1.0,
            Combat::new(&Fight, &Electric)    => 1.0,
            Combat::new(&Fight, &Psychic)     => 0.5,
            Combat::new(&Fight, &Ice)         => 2.0,
            Combat::new(&Fight, &Dragon)      => 1.0,
            Combat::new(&Fight, &Dark)        => 2.0,
            Combat::new(&Fight, &Fairy)       => 0.5,

            // atack: flying
            Combat::new(&Flying, &Normal)      => 1.0,
            Combat::new(&Flying, &Fight)       => 2.0,
            Combat::new(&Flying, &Flying)      => 1.0,
            Combat::new(&Flying, &Poison)      => 1.0,
            Combat::new(&Flying, &Ground)      => 1.0,
            Combat::new(&Flying, &Rock)        => 0.5,
            Combat::new(&Flying, &Bug)         => 2.0,
            Combat::new(&Flying, &Ghost)       => 1.0,
            Combat::new(&Flying, &Steel)       => 0.5,
            Combat::new(&Flying, &Fire)        => 1.0,
            Combat::new(&Flying, &Water)       => 1.0,
            Combat::new(&Flying, &Grass)       => 2.0,
            Combat::new(&Flying, &Electric)    => 0.5,
            Combat::new(&Flying, &Psychic)     => 1.0,
            Combat::new(&Flying, &Ice)         => 1.0,
            Combat::new(&Flying, &Dragon)      => 1.0,
            Combat::new(&Flying, &Dark)        => 1.0,
            Combat::new(&Flying, &Fairy)       => 1.0,

            // atack: poison
            Combat::new(&Poison, &Normal)      => 1.0,
            Combat::new(&Poison, &Fight)       => 1.0,
            Combat::new(&Poison, &Flying)      => 1.0,
            Combat::new(&Poison, &Poison)      => 0.5,
            Combat::new(&Poison, &Ground)      => 0.5,
            Combat::new(&Poison, &Rock)        => 0.5,
            Combat::new(&Poison, &Bug)         => 1.0,
            Combat::new(&Poison, &Ghost)       => 0.5,
            Combat::new(&Poison, &Steel)       => 0.0,
            Combat::new(&Poison, &Fire)        => 1.0,
            Combat::new(&Poison, &Water)       => 1.0,
            Combat::new(&Poison, &Grass)       => 2.0,
            Combat::new(&Poison, &Electric)    => 1.0,
            Combat::new(&Poison, &Psychic)     => 1.0,
            Combat::new(&Poison, &Ice)         => 1.0,
            Combat::new(&Poison, &Dragon)      => 1.0,
            Combat::new(&Poison, &Dark)        => 1.0,
            Combat::new(&Poison, &Fairy)       => 2.0,

            // atack: ground
            Combat::new(&Ground, &Normal)      => 1.0,
            Combat::new(&Ground, &Fight)       => 1.0,
            Combat::new(&Ground, &Flying)      => 0.0,
            Combat::new(&Ground, &Poison)      => 2.0,
            Combat::new(&Ground, &Ground)      => 1.0,
            Combat::new(&Ground, &Rock)        => 2.0,
            Combat::new(&Ground, &Bug)         => 0.5,
            Combat::new(&Ground, &Ghost)       => 1.0,
            Combat::new(&Ground, &Steel)       => 2.0,
            Combat::new(&Ground, &Fire)        => 2.0,
            Combat::new(&Ground, &Water)       => 1.0,
            Combat::new(&Ground, &Grass)       => 0.5,
            Combat::new(&Ground, &Electric)    => 2.0,
            Combat::new(&Ground, &Psychic)     => 1.0,
            Combat::new(&Ground, &Ice)         => 1.0,
            Combat::new(&Ground, &Dragon)      => 1.0,
            Combat::new(&Ground, &Dark)        => 1.0,
            Combat::new(&Ground, &Fairy)       => 1.0,

            // atack: rock
            Combat::new(&Rock, &Normal)      => 1.0,
            Combat::new(&Rock, &Fight)       => 0.5,
            Combat::new(&Rock, &Flying)      => 2.0,
            Combat::new(&Rock, &Poison)      => 1.0,
            Combat::new(&Rock, &Ground)      => 0.5,
            Combat::new(&Rock, &Rock)        => 1.0,
            Combat::new(&Rock, &Bug)         => 2.0,
            Combat::new(&Rock, &Ghost)       => 1.0,
            Combat::new(&Rock, &Steel)       => 0.5,
            Combat::new(&Rock, &Fire)        => 2.0,
            Combat::new(&Rock, &Water)       => 1.0,
            Combat::new(&Rock, &Grass)       => 1.0,
            Combat::new(&Rock, &Electric)    => 1.0,
            Combat::new(&Rock, &Psychic)     => 1.0,
            Combat::new(&Rock, &Ice)         => 2.0,
            Combat::new(&Rock, &Dragon)      => 1.0,
            Combat::new(&Rock, &Dark)        => 1.0,
            Combat::new(&Rock, &Fairy)       => 1.0,

            // atack: bug
            Combat::new(&Bug, &Normal)      => 1.0,
            Combat::new(&Bug, &Fight)       => 0.5,
            Combat::new(&Bug, &Flying)      => 0.5,
            Combat::new(&Bug, &Poison)      => 0.5,
            Combat::new(&Bug, &Ground)      => 1.0,
            Combat::new(&Bug, &Rock)        => 1.0,
            Combat::new(&Bug, &Bug)         => 1.0,
            Combat::new(&Bug, &Ghost)       => 0.5,
            Combat::new(&Bug, &Steel)       => 0.5,
            Combat::new(&Bug, &Fire)        => 0.5,
            Combat::new(&Bug, &Water)       => 1.0,
            Combat::new(&Bug, &Grass)       => 2.0,
            Combat::new(&Bug, &Electric)    => 1.0,
            Combat::new(&Bug, &Psychic)     => 2.0,
            Combat::new(&Bug, &Ice)         => 1.0,
            Combat::new(&Bug, &Dragon)      => 1.0,
            Combat::new(&Bug, &Dark)        => 2.0,
            Combat::new(&Bug, &Fairy)       => 0.5,

            // atack: ghost
            Combat::new(&Ghost, &Normal)      => 0.0,
            Combat::new(&Ghost, &Fight)       => 1.0,
            Combat::new(&Ghost, &Flying)      => 1.0,
            Combat::new(&Ghost, &Poison)      => 1.0,
            Combat::new(&Ghost, &Ground)      => 1.0,
            Combat::new(&Ghost, &Rock)        => 1.0,
            Combat::new(&Ghost, &Bug)         => 1.0,
            Combat::new(&Ghost, &Ghost)       => 2.0,
            Combat::new(&Ghost, &Steel)       => 1.0,
            Combat::new(&Ghost, &Fire)        => 1.0,
            Combat::new(&Ghost, &Water)       => 1.0,
            Combat::new(&Ghost, &Grass)       => 1.0,
            Combat::new(&Ghost, &Electric)    => 1.0,
            Combat::new(&Ghost, &Psychic)     => 2.0,
            Combat::new(&Ghost, &Ice)         => 1.0,
            Combat::new(&Ghost, &Dragon)      => 1.0,
            Combat::new(&Ghost, &Dark)        => 0.5,
            Combat::new(&Ghost, &Fairy)       => 1.0,

            // atack: steel
            Combat::new(&Steel, &Normal)      => 1.0,
            Combat::new(&Steel, &Fight)       => 1.0,
            Combat::new(&Steel, &Flying)      => 1.0,
            Combat::new(&Steel, &Poison)      => 1.0,
            Combat::new(&Steel, &Ground)      => 1.0,
            Combat::new(&Steel, &Rock)        => 2.0,
            Combat::new(&Steel, &Bug)         => 1.0,
            Combat::new(&Steel, &Ghost)       => 1.0,
            Combat::new(&Steel, &Steel)       => 0.5,
            Combat::new(&Steel, &Fire)        => 0.5,
            Combat::new(&Steel, &Water)       => 0.5,
            Combat::new(&Steel, &Grass)       => 1.0,
            Combat::new(&Steel, &Electric)    => 0.5,
            Combat::new(&Steel, &Psychic)     => 1.0,
            Combat::new(&Steel, &Ice)         => 2.0,
            Combat::new(&Steel, &Dragon)      => 1.0,
            Combat::new(&Steel, &Dark)        => 1.0,
            Combat::new(&Steel, &Fairy)       => 2.0,

            // atack: fire
            Combat::new(&Fire, &Normal)      => 1.0,
            Combat::new(&Fire, &Fight)       => 1.0,
            Combat::new(&Fire, &Flying)      => 1.0,
            Combat::new(&Fire, &Poison)      => 1.0,
            Combat::new(&Fire, &Ground)      => 1.0,
            Combat::new(&Fire, &Rock)        => 0.5,
            Combat::new(&Fire, &Bug)         => 2.0,
            Combat::new(&Fire, &Ghost)       => 1.0,
            Combat::new(&Fire, &Steel)       => 2.0,
            Combat::new(&Fire, &Fire)        => 0.5,
            Combat::new(&Fire, &Water)       => 0.5,
            Combat::new(&Fire, &Grass)       => 2.0,
            Combat::new(&Fire, &Electric)    => 1.0,
            Combat::new(&Fire, &Psychic)     => 1.0,
            Combat::new(&Fire, &Ice)         => 2.0,
            Combat::new(&Fire, &Dragon)      => 0.5,
            Combat::new(&Fire, &Dark)        => 1.0,
            Combat::new(&Fire, &Fairy)       => 1.0,

            // atack: water
            Combat::new(&Water, &Normal)      => 1.0,
            Combat::new(&Water, &Fight)       => 1.0,
            Combat::new(&Water, &Flying)      => 1.0,
            Combat::new(&Water, &Poison)      => 1.0,
            Combat::new(&Water, &Ground)      => 2.0,
            Combat::new(&Water, &Rock)        => 2.0,
            Combat::new(&Water, &Bug)         => 1.0,
            Combat::new(&Water, &Ghost)       => 1.0,
            Combat::new(&Water, &Steel)       => 1.0,
            Combat::new(&Water, &Fire)        => 2.0,
            Combat::new(&Water, &Water)       => 0.5,
            Combat::new(&Water, &Grass)       => 0.5,
            Combat::new(&Water, &Electric)    => 1.0,
            Combat::new(&Water, &Psychic)     => 1.0,
            Combat::new(&Water, &Ice)         => 1.0,
            Combat::new(&Water, &Dragon)      => 0.5,
            Combat::new(&Water, &Dark)        => 1.0,
            Combat::new(&Water, &Fairy)       => 1.0,

            // atack: grass
            Combat::new(&Grass, &Normal)      => 1.0,
            Combat::new(&Grass, &Fight)       => 1.0,
            Combat::new(&Grass, &Flying)      => 0.5,
            Combat::new(&Grass, &Poison)      => 0.5,
            Combat::new(&Grass, &Ground)      => 2.0,
            Combat::new(&Grass, &Rock)        => 2.0,
            Combat::new(&Grass, &Bug)         => 0.5,
            Combat::new(&Grass, &Ghost)       => 1.0,
            Combat::new(&Grass, &Steel)       => 0.5,
            Combat::new(&Grass, &Fire)        => 0.5,
            Combat::new(&Grass, &Water)       => 2.0,
            Combat::new(&Grass, &Grass)       => 0.5,
            Combat::new(&Grass, &Electric)    => 1.0,
            Combat::new(&Grass, &Psychic)     => 1.0,
            Combat::new(&Grass, &Ice)         => 1.0,
            Combat::new(&Grass, &Dragon)      => 0.5,
            Combat::new(&Grass, &Dark)        => 1.0,
            Combat::new(&Grass, &Fairy)       => 1.0,

            // atack: electric
            Combat::new(&Electric, &Normal)      => 1.0,
            Combat::new(&Electric, &Fight)       => 1.0,
            Combat::new(&Electric, &Flying)      => 2.0,
            Combat::new(&Electric, &Poison)      => 1.0,
            Combat::new(&Electric, &Ground)      => 0.0,
            Combat::new(&Electric, &Rock)        => 1.0,
            Combat::new(&Electric, &Bug)         => 1.0,
            Combat::new(&Electric, &Ghost)       => 1.0,
            Combat::new(&Electric, &Steel)       => 1.0,
            Combat::new(&Electric, &Fire)        => 1.0,
            Combat::new(&Electric, &Water)       => 2.0,
            Combat::new(&Electric, &Grass)       => 0.5,
            Combat::new(&Electric, &Electric)    => 0.5,
            Combat::new(&Electric, &Psychic)     => 1.0,
            Combat::new(&Electric, &Ice)         => 1.0,
            Combat::new(&Electric, &Dragon)      => 0.5,
            Combat::new(&Electric, &Dark)        => 1.0,
            Combat::new(&Electric, &Fairy)       => 1.0,

            // atack: psychic
            Combat::new(&Psychic, &Normal)      => 1.0,
            Combat::new(&Psychic, &Fight)       => 2.0,
            Combat::new(&Psychic, &Flying)      => 1.0,
            Combat::new(&Psychic, &Poison)      => 2.0,
            Combat::new(&Psychic, &Ground)      => 1.0,
            Combat::new(&Psychic, &Rock)        => 1.0,
            Combat::new(&Psychic, &Bug)         => 1.0,
            Combat::new(&Psychic, &Ghost)       => 1.0,
            Combat::new(&Psychic, &Steel)       => 0.5,
            Combat::new(&Psychic, &Fire)        => 1.0,
            Combat::new(&Psychic, &Water)       => 1.0,
            Combat::new(&Psychic, &Grass)       => 1.0,
            Combat::new(&Psychic, &Electric)    => 1.0,
            Combat::new(&Psychic, &Psychic)     => 0.5,
            Combat::new(&Psychic, &Ice)         => 1.0,
            Combat::new(&Psychic, &Dragon)      => 1.0,
            Combat::new(&Psychic, &Dark)        => 0.0,
            Combat::new(&Psychic, &Fairy)       => 1.0,

            // atack: Ice
            Combat::new(&Ice, &Normal)      => 1.0,
            Combat::new(&Ice, &Fight)       => 1.0,
            Combat::new(&Ice, &Flying)      => 2.0,
            Combat::new(&Ice, &Poison)      => 1.0,
            Combat::new(&Ice, &Ground)      => 2.0,
            Combat::new(&Ice, &Rock)        => 1.0,
            Combat::new(&Ice, &Bug)         => 1.0,
            Combat::new(&Ice, &Ghost)       => 1.0,
            Combat::new(&Ice, &Steel)       => 0.5,
            Combat::new(&Ice, &Fire)        => 0.5,
            Combat::new(&Ice, &Water)       => 0.5,
            Combat::new(&Ice, &Grass)       => 2.0,
            Combat::new(&Ice, &Electric)    => 1.0,
            Combat::new(&Ice, &Psychic)     => 1.0,
            Combat::new(&Ice, &Ice)         => 0.5,
            Combat::new(&Ice, &Dragon)      => 2.0,
            Combat::new(&Ice, &Dark)        => 1.0,
            Combat::new(&Ice, &Fairy)       => 1.0,

            // atack: Dragon
            Combat::new(&Dragon, &Normal)      => 1.0,
            Combat::new(&Dragon, &Fight)       => 1.0,
            Combat::new(&Dragon, &Flying)      => 1.0,
            Combat::new(&Dragon, &Poison)      => 1.0,
            Combat::new(&Dragon, &Ground)      => 1.0,
            Combat::new(&Dragon, &Rock)        => 1.0,
            Combat::new(&Dragon, &Bug)         => 1.0,
            Combat::new(&Dragon, &Ghost)       => 1.0,
            Combat::new(&Dragon, &Steel)       => 0.5,
            Combat::new(&Dragon, &Fire)        => 1.0,
            Combat::new(&Dragon, &Water)       => 1.0,
            Combat::new(&Dragon, &Grass)       => 1.0,
            Combat::new(&Dragon, &Electric)    => 1.0,
            Combat::new(&Dragon, &Psychic)     => 1.0,
            Combat::new(&Dragon, &Ice)         => 1.0,
            Combat::new(&Dragon, &Dragon)      => 2.0,
            Combat::new(&Dragon, &Dark)        => 1.0,
            Combat::new(&Dragon, &Fairy)       => 0.0,

            // atack: Dark
            Combat::new(&Dark, &Normal)      => 1.0,
            Combat::new(&Dark, &Fight)       => 0.5,
            Combat::new(&Dark, &Flying)      => 1.0,
            Combat::new(&Dark, &Poison)      => 1.0,
            Combat::new(&Dark, &Ground)      => 1.0,
            Combat::new(&Dark, &Rock)        => 1.0,
            Combat::new(&Dark, &Bug)         => 1.0,
            Combat::new(&Dark, &Ghost)       => 2.0,
            Combat::new(&Dark, &Steel)       => 1.0,
            Combat::new(&Dark, &Fire)        => 1.0,
            Combat::new(&Dark, &Water)       => 1.0,
            Combat::new(&Dark, &Grass)       => 1.0,
            Combat::new(&Dark, &Electric)    => 1.0,
            Combat::new(&Dark, &Psychic)     => 2.0,
            Combat::new(&Dark, &Ice)         => 1.0,
            Combat::new(&Dark, &Dragon)      => 1.0,
            Combat::new(&Dark, &Dark)        => 0.5,
            Combat::new(&Dark, &Fairy)       => 0.5,

            // atack: Fairy
            Combat::new(&Fairy, &Normal)      => 1.0,
            Combat::new(&Fairy, &Fight)       => 2.0,
            Combat::new(&Fairy, &Flying)      => 1.0,
            Combat::new(&Fairy, &Poison)      => 0.5,
            Combat::new(&Fairy, &Ground)      => 1.0,
            Combat::new(&Fairy, &Rock)        => 1.0,
            Combat::new(&Fairy, &Bug)         => 1.0,
            Combat::new(&Fairy, &Ghost)       => 1.0,
            Combat::new(&Fairy, &Steel)       => 0.5,
            Combat::new(&Fairy, &Fire)        => 0.5,
            Combat::new(&Fairy, &Water)       => 1.0,
            Combat::new(&Fairy, &Grass)       => 1.0,
            Combat::new(&Fairy, &Electric)    => 1.0,
            Combat::new(&Fairy, &Psychic)     => 1.0,
            Combat::new(&Fairy, &Ice)         => 1.0,
            Combat::new(&Fairy, &Dragon)      => 2.0,
            Combat::new(&Fairy, &Dark)        => 2.0,
            Combat::new(&Fairy, &Fairy)       => 1.0,
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combat_normal_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Normal, &Normal), 1.0);
        assert_eq!(Type::combat1(&Normal, &Fight), 1.0);
        assert_eq!(Type::combat1(&Normal, &Flying), 1.0);
        assert_eq!(Type::combat1(&Normal, &Poison), 1.0);
        assert_eq!(Type::combat1(&Normal, &Ground), 1.0);
        assert_eq!(Type::combat1(&Normal, &Rock), 0.5);
        assert_eq!(Type::combat1(&Normal, &Bug), 1.0);
        assert_eq!(Type::combat1(&Normal, &Ghost), 0.0);
        assert_eq!(Type::combat1(&Normal, &Steel), 0.5);
        assert_eq!(Type::combat1(&Normal, &Fire), 1.0);
        assert_eq!(Type::combat1(&Normal, &Water), 1.0);
        assert_eq!(Type::combat1(&Normal, &Grass), 1.0);
        assert_eq!(Type::combat1(&Normal, &Electric), 1.0);
        assert_eq!(Type::combat1(&Normal, &Psychic), 1.0);
        assert_eq!(Type::combat1(&Normal, &Ice), 1.0);
        assert_eq!(Type::combat1(&Normal, &Dragon), 1.0);
        assert_eq!(Type::combat1(&Normal, &Dark), 1.0);
        assert_eq!(Type::combat1(&Normal, &Fairy), 1.0);
    }

    #[test]
    fn combat_fight_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Fight, &Normal), 2.0);
        assert_eq!(Type::combat1(&Fight, &Fight), 1.0);
        assert_eq!(Type::combat1(&Fight, &Flying), 0.5);
        assert_eq!(Type::combat1(&Fight, &Poison), 0.5);
        assert_eq!(Type::combat1(&Fight, &Ground), 1.0);
        assert_eq!(Type::combat1(&Fight, &Rock), 2.0);
        assert_eq!(Type::combat1(&Fight, &Bug), 0.5);
        assert_eq!(Type::combat1(&Fight, &Ghost), 0.0);
        assert_eq!(Type::combat1(&Fight, &Steel), 2.0);
        assert_eq!(Type::combat1(&Fight, &Fire), 1.0);
        assert_eq!(Type::combat1(&Fight, &Water), 1.0);
        assert_eq!(Type::combat1(&Fight, &Grass), 1.0);
        assert_eq!(Type::combat1(&Fight, &Electric), 1.0);
        assert_eq!(Type::combat1(&Fight, &Psychic), 0.5);
        assert_eq!(Type::combat1(&Fight, &Ice), 2.0);
        assert_eq!(Type::combat1(&Fight, &Dragon), 1.0);
        assert_eq!(Type::combat1(&Fight, &Dark), 2.0);
        assert_eq!(Type::combat1(&Fight, &Fairy), 0.5);
    }

    #[test]
    fn combat_flying_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Flying, &Normal), 1.0);
        assert_eq!(Type::combat1(&Flying, &Fight), 2.0);
        assert_eq!(Type::combat1(&Flying, &Flying), 1.0);
        assert_eq!(Type::combat1(&Flying, &Poison), 1.0);
        assert_eq!(Type::combat1(&Flying, &Ground), 1.0);
        assert_eq!(Type::combat1(&Flying, &Rock), 0.5);
        assert_eq!(Type::combat1(&Flying, &Bug), 2.0);
        assert_eq!(Type::combat1(&Flying, &Ghost), 1.0);
        assert_eq!(Type::combat1(&Flying, &Steel), 0.5);
        assert_eq!(Type::combat1(&Flying, &Fire), 1.0);
        assert_eq!(Type::combat1(&Flying, &Water), 1.0);
        assert_eq!(Type::combat1(&Flying, &Grass), 2.0);
        assert_eq!(Type::combat1(&Flying, &Electric), 0.5);
        assert_eq!(Type::combat1(&Flying, &Psychic), 1.0);
        assert_eq!(Type::combat1(&Flying, &Ice), 1.0);
        assert_eq!(Type::combat1(&Flying, &Dragon), 1.0);
        assert_eq!(Type::combat1(&Flying, &Dark), 1.0);
        assert_eq!(Type::combat1(&Flying, &Fairy), 1.0);
    }

    #[test]
    fn combat_poison_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Poison, &Normal), 1.0);
        assert_eq!(Type::combat1(&Poison, &Fight), 1.0);
        assert_eq!(Type::combat1(&Poison, &Flying), 1.0);
        assert_eq!(Type::combat1(&Poison, &Poison), 0.5);
        assert_eq!(Type::combat1(&Poison, &Ground), 0.5);
        assert_eq!(Type::combat1(&Poison, &Rock), 0.5);
        assert_eq!(Type::combat1(&Poison, &Bug), 1.0);
        assert_eq!(Type::combat1(&Poison, &Ghost), 0.5);
        assert_eq!(Type::combat1(&Poison, &Steel), 0.0);
        assert_eq!(Type::combat1(&Poison, &Fire), 1.0);
        assert_eq!(Type::combat1(&Poison, &Water), 1.0);
        assert_eq!(Type::combat1(&Poison, &Grass), 2.0);
        assert_eq!(Type::combat1(&Poison, &Electric), 1.0);
        assert_eq!(Type::combat1(&Poison, &Psychic), 1.0);
        assert_eq!(Type::combat1(&Poison, &Ice), 1.0);
        assert_eq!(Type::combat1(&Poison, &Dragon), 1.0);
        assert_eq!(Type::combat1(&Poison, &Dark), 1.0);
        assert_eq!(Type::combat1(&Poison, &Fairy), 2.0);
    }

    #[test]
    fn combat_ground_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Ground, &Normal), 1.0);
        assert_eq!(Type::combat1(&Ground, &Fight), 1.0);
        assert_eq!(Type::combat1(&Ground, &Flying), 0.0);
        assert_eq!(Type::combat1(&Ground, &Poison), 2.0);
        assert_eq!(Type::combat1(&Ground, &Ground), 1.0);
        assert_eq!(Type::combat1(&Ground, &Rock), 2.0);
        assert_eq!(Type::combat1(&Ground, &Bug), 0.5);
        assert_eq!(Type::combat1(&Ground, &Ghost), 1.0);
        assert_eq!(Type::combat1(&Ground, &Steel), 2.0);
        assert_eq!(Type::combat1(&Ground, &Fire), 2.0);
        assert_eq!(Type::combat1(&Ground, &Water), 1.0);
        assert_eq!(Type::combat1(&Ground, &Grass), 0.5);
        assert_eq!(Type::combat1(&Ground, &Electric), 2.0);
        assert_eq!(Type::combat1(&Ground, &Psychic), 1.0);
        assert_eq!(Type::combat1(&Ground, &Ice), 1.0);
        assert_eq!(Type::combat1(&Ground, &Dragon), 1.0);
        assert_eq!(Type::combat1(&Ground, &Dark), 1.0);
        assert_eq!(Type::combat1(&Ground, &Fairy), 1.0);
    }

    #[test]
    fn combat_rock_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Rock, &Normal), 1.0);
        assert_eq!(Type::combat1(&Rock, &Fight), 0.5);
        assert_eq!(Type::combat1(&Rock, &Flying), 2.0);
        assert_eq!(Type::combat1(&Rock, &Poison), 1.0);
        assert_eq!(Type::combat1(&Rock, &Ground), 0.5);
        assert_eq!(Type::combat1(&Rock, &Rock), 1.0);
        assert_eq!(Type::combat1(&Rock, &Bug), 2.0);
        assert_eq!(Type::combat1(&Rock, &Ghost), 1.0);
        assert_eq!(Type::combat1(&Rock, &Steel), 0.5);
        assert_eq!(Type::combat1(&Rock, &Fire), 2.0);
        assert_eq!(Type::combat1(&Rock, &Water), 1.0);
        assert_eq!(Type::combat1(&Rock, &Grass), 1.0);
        assert_eq!(Type::combat1(&Rock, &Electric), 1.0);
        assert_eq!(Type::combat1(&Rock, &Psychic), 1.0);
        assert_eq!(Type::combat1(&Rock, &Ice), 2.0);
        assert_eq!(Type::combat1(&Rock, &Dragon), 1.0);
        assert_eq!(Type::combat1(&Rock, &Dark), 1.0);
        assert_eq!(Type::combat1(&Rock, &Fairy), 1.0);
    }

    #[test]
    fn combat_bug_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Bug, &Normal), 1.0);
        assert_eq!(Type::combat1(&Bug, &Fight), 0.5);
        assert_eq!(Type::combat1(&Bug, &Flying), 0.5);
        assert_eq!(Type::combat1(&Bug, &Poison), 0.5);
        assert_eq!(Type::combat1(&Bug, &Ground), 1.0);
        assert_eq!(Type::combat1(&Bug, &Rock), 1.0);
        assert_eq!(Type::combat1(&Bug, &Bug), 1.0);
        assert_eq!(Type::combat1(&Bug, &Ghost), 0.5);
        assert_eq!(Type::combat1(&Bug, &Steel), 0.5);
        assert_eq!(Type::combat1(&Bug, &Fire), 0.5);
        assert_eq!(Type::combat1(&Bug, &Water), 1.0);
        assert_eq!(Type::combat1(&Bug, &Grass), 2.0);
        assert_eq!(Type::combat1(&Bug, &Electric), 1.0);
        assert_eq!(Type::combat1(&Bug, &Psychic), 2.0);
        assert_eq!(Type::combat1(&Bug, &Ice), 1.0);
        assert_eq!(Type::combat1(&Bug, &Dragon), 1.0);
        assert_eq!(Type::combat1(&Bug, &Dark), 2.0);
        assert_eq!(Type::combat1(&Bug, &Fairy), 0.5);
    }

    #[test]
    fn combat_ghost_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Ghost, &Normal), 0.0);
        assert_eq!(Type::combat1(&Ghost, &Fight), 1.0);
        assert_eq!(Type::combat1(&Ghost, &Flying), 1.0);
        assert_eq!(Type::combat1(&Ghost, &Poison), 1.0);
        assert_eq!(Type::combat1(&Ghost, &Ground), 1.0);
        assert_eq!(Type::combat1(&Ghost, &Rock), 1.0);
        assert_eq!(Type::combat1(&Ghost, &Bug), 1.0);
        assert_eq!(Type::combat1(&Ghost, &Ghost), 2.0);
        assert_eq!(Type::combat1(&Ghost, &Steel), 1.0);
        assert_eq!(Type::combat1(&Ghost, &Fire), 1.0);
        assert_eq!(Type::combat1(&Ghost, &Water), 1.0);
        assert_eq!(Type::combat1(&Ghost, &Grass), 1.0);
        assert_eq!(Type::combat1(&Ghost, &Electric), 1.0);
        assert_eq!(Type::combat1(&Ghost, &Psychic), 2.0);
        assert_eq!(Type::combat1(&Ghost, &Ice), 1.0);
        assert_eq!(Type::combat1(&Ghost, &Dragon), 1.0);
        assert_eq!(Type::combat1(&Ghost, &Dark), 0.5);
        assert_eq!(Type::combat1(&Ghost, &Fairy), 1.0);
    }

    #[test]
    fn combat_steel_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Steel, &Normal), 1.0);
        assert_eq!(Type::combat1(&Steel, &Fight), 1.0);
        assert_eq!(Type::combat1(&Steel, &Flying), 1.0);
        assert_eq!(Type::combat1(&Steel, &Poison), 1.0);
        assert_eq!(Type::combat1(&Steel, &Ground), 1.0);
        assert_eq!(Type::combat1(&Steel, &Rock), 2.0);
        assert_eq!(Type::combat1(&Steel, &Bug), 1.0);
        assert_eq!(Type::combat1(&Steel, &Ghost), 1.0);
        assert_eq!(Type::combat1(&Steel, &Steel), 0.5);
        assert_eq!(Type::combat1(&Steel, &Fire), 0.5);
        assert_eq!(Type::combat1(&Steel, &Water), 0.5);
        assert_eq!(Type::combat1(&Steel, &Grass), 1.0);
        assert_eq!(Type::combat1(&Steel, &Electric), 0.5);
        assert_eq!(Type::combat1(&Steel, &Psychic), 1.0);
        assert_eq!(Type::combat1(&Steel, &Ice), 2.0);
        assert_eq!(Type::combat1(&Steel, &Dragon), 1.0);
        assert_eq!(Type::combat1(&Steel, &Dark), 1.0);
        assert_eq!(Type::combat1(&Steel, &Fairy), 2.0);
    }

    #[test]
    fn combat_fire_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Fire, &Normal), 1.0);
        assert_eq!(Type::combat1(&Fire, &Fight), 1.0);
        assert_eq!(Type::combat1(&Fire, &Flying), 1.0);
        assert_eq!(Type::combat1(&Fire, &Poison), 1.0);
        assert_eq!(Type::combat1(&Fire, &Ground), 1.0);
        assert_eq!(Type::combat1(&Fire, &Rock), 0.5);
        assert_eq!(Type::combat1(&Fire, &Bug), 2.0);
        assert_eq!(Type::combat1(&Fire, &Ghost), 1.0);
        assert_eq!(Type::combat1(&Fire, &Steel), 2.0);
        assert_eq!(Type::combat1(&Fire, &Fire), 0.5);
        assert_eq!(Type::combat1(&Fire, &Water), 0.5);
        assert_eq!(Type::combat1(&Fire, &Grass), 2.0);
        assert_eq!(Type::combat1(&Fire, &Electric), 1.0);
        assert_eq!(Type::combat1(&Fire, &Psychic), 1.0);
        assert_eq!(Type::combat1(&Fire, &Ice), 2.0);
        assert_eq!(Type::combat1(&Fire, &Dragon), 0.5);
        assert_eq!(Type::combat1(&Fire, &Dark), 1.0);
        assert_eq!(Type::combat1(&Fire, &Fairy), 1.0);
    }

    #[test]
    fn combat_water_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Water, &Normal), 1.0);
        assert_eq!(Type::combat1(&Water, &Fight), 1.0);
        assert_eq!(Type::combat1(&Water, &Flying), 1.0);
        assert_eq!(Type::combat1(&Water, &Poison), 1.0);
        assert_eq!(Type::combat1(&Water, &Ground), 2.0);
        assert_eq!(Type::combat1(&Water, &Rock), 2.0);
        assert_eq!(Type::combat1(&Water, &Bug), 1.0);
        assert_eq!(Type::combat1(&Water, &Ghost), 1.0);
        assert_eq!(Type::combat1(&Water, &Steel), 1.0);
        assert_eq!(Type::combat1(&Water, &Fire), 2.0);
        assert_eq!(Type::combat1(&Water, &Water), 0.5);
        assert_eq!(Type::combat1(&Water, &Grass), 0.5);
        assert_eq!(Type::combat1(&Water, &Electric), 1.0);
        assert_eq!(Type::combat1(&Water, &Psychic), 1.0);
        assert_eq!(Type::combat1(&Water, &Ice), 1.0);
        assert_eq!(Type::combat1(&Water, &Dragon), 0.5);
        assert_eq!(Type::combat1(&Water, &Dark), 1.0);
        assert_eq!(Type::combat1(&Water, &Fairy), 1.0);
    }

    #[test]
    fn combat_grass_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Grass, &Normal), 1.0);
        assert_eq!(Type::combat1(&Grass, &Fight), 1.0);
        assert_eq!(Type::combat1(&Grass, &Flying), 0.5);
        assert_eq!(Type::combat1(&Grass, &Poison), 0.5);
        assert_eq!(Type::combat1(&Grass, &Ground), 2.0);
        assert_eq!(Type::combat1(&Grass, &Rock), 2.0);
        assert_eq!(Type::combat1(&Grass, &Bug), 0.5);
        assert_eq!(Type::combat1(&Grass, &Ghost), 1.0);
        assert_eq!(Type::combat1(&Grass, &Steel), 0.5);
        assert_eq!(Type::combat1(&Grass, &Fire), 0.5);
        assert_eq!(Type::combat1(&Grass, &Water), 2.0);
        assert_eq!(Type::combat1(&Grass, &Grass), 0.5);
        assert_eq!(Type::combat1(&Grass, &Electric), 1.0);
        assert_eq!(Type::combat1(&Grass, &Psychic), 1.0);
        assert_eq!(Type::combat1(&Grass, &Ice), 1.0);
        assert_eq!(Type::combat1(&Grass, &Dragon), 0.5);
        assert_eq!(Type::combat1(&Grass, &Dark), 1.0);
        assert_eq!(Type::combat1(&Grass, &Fairy), 1.0);
    }

    #[test]
    fn combat_electric_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Electric, &Normal), 1.0);
        assert_eq!(Type::combat1(&Electric, &Fight), 1.0);
        assert_eq!(Type::combat1(&Electric, &Flying), 2.0);
        assert_eq!(Type::combat1(&Electric, &Poison), 1.0);
        assert_eq!(Type::combat1(&Electric, &Ground), 0.0);
        assert_eq!(Type::combat1(&Electric, &Rock), 1.0);
        assert_eq!(Type::combat1(&Electric, &Bug), 1.0);
        assert_eq!(Type::combat1(&Electric, &Ghost), 1.0);
        assert_eq!(Type::combat1(&Electric, &Steel), 1.0);
        assert_eq!(Type::combat1(&Electric, &Fire), 1.0);
        assert_eq!(Type::combat1(&Electric, &Water), 2.0);
        assert_eq!(Type::combat1(&Electric, &Grass), 0.5);
        assert_eq!(Type::combat1(&Electric, &Electric), 0.5);
        assert_eq!(Type::combat1(&Electric, &Psychic), 1.0);
        assert_eq!(Type::combat1(&Electric, &Ice), 1.0);
        assert_eq!(Type::combat1(&Electric, &Dragon), 0.5);
        assert_eq!(Type::combat1(&Electric, &Dark), 1.0);
        assert_eq!(Type::combat1(&Electric, &Fairy), 1.0);
    }

    #[test]
    fn combat_psychic_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Psychic, &Normal), 1.0);
        assert_eq!(Type::combat1(&Psychic, &Fight), 2.0);
        assert_eq!(Type::combat1(&Psychic, &Flying), 1.0);
        assert_eq!(Type::combat1(&Psychic, &Poison), 2.0);
        assert_eq!(Type::combat1(&Psychic, &Ground), 1.0);
        assert_eq!(Type::combat1(&Psychic, &Rock), 1.0);
        assert_eq!(Type::combat1(&Psychic, &Bug), 1.0);
        assert_eq!(Type::combat1(&Psychic, &Ghost), 1.0);
        assert_eq!(Type::combat1(&Psychic, &Steel), 0.5);
        assert_eq!(Type::combat1(&Psychic, &Fire), 1.0);
        assert_eq!(Type::combat1(&Psychic, &Water), 1.0);
        assert_eq!(Type::combat1(&Psychic, &Grass), 1.0);
        assert_eq!(Type::combat1(&Psychic, &Electric), 1.0);
        assert_eq!(Type::combat1(&Psychic, &Psychic), 0.5);
        assert_eq!(Type::combat1(&Psychic, &Ice), 1.0);
        assert_eq!(Type::combat1(&Psychic, &Dragon), 1.0);
        assert_eq!(Type::combat1(&Psychic, &Dark), 0.0);
        assert_eq!(Type::combat1(&Psychic, &Fairy), 1.0);
    }

    #[test]
    fn combat_ice_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Ice, &Normal), 1.0);
        assert_eq!(Type::combat1(&Ice, &Fight), 1.0);
        assert_eq!(Type::combat1(&Ice, &Flying), 2.0);
        assert_eq!(Type::combat1(&Ice, &Poison), 1.0);
        assert_eq!(Type::combat1(&Ice, &Ground), 2.0);
        assert_eq!(Type::combat1(&Ice, &Rock), 1.0);
        assert_eq!(Type::combat1(&Ice, &Bug), 1.0);
        assert_eq!(Type::combat1(&Ice, &Ghost), 1.0);
        assert_eq!(Type::combat1(&Ice, &Steel), 0.5);
        assert_eq!(Type::combat1(&Ice, &Fire), 0.5);
        assert_eq!(Type::combat1(&Ice, &Water), 0.5);
        assert_eq!(Type::combat1(&Ice, &Grass), 2.0);
        assert_eq!(Type::combat1(&Ice, &Electric), 1.0);
        assert_eq!(Type::combat1(&Ice, &Psychic), 1.0);
        assert_eq!(Type::combat1(&Ice, &Ice), 0.5);
        assert_eq!(Type::combat1(&Ice, &Dragon), 2.0);
        assert_eq!(Type::combat1(&Ice, &Dark), 1.0);
        assert_eq!(Type::combat1(&Ice, &Fairy), 1.0);
    }

    #[test]
    fn combat_dragon_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Dragon, &Normal), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Fight), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Flying), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Poison), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Ground), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Rock), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Bug), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Ghost), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Steel), 0.5);
        assert_eq!(Type::combat1(&Dragon, &Fire), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Water), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Grass), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Electric), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Psychic), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Ice), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Dragon), 2.0);
        assert_eq!(Type::combat1(&Dragon, &Dark), 1.0);
        assert_eq!(Type::combat1(&Dragon, &Fairy), 0.0);
    }

    #[test]
    fn combat_dark_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Dark, &Normal), 1.0);
        assert_eq!(Type::combat1(&Dark, &Fight), 0.5);
        assert_eq!(Type::combat1(&Dark, &Flying), 1.0);
        assert_eq!(Type::combat1(&Dark, &Poison), 1.0);
        assert_eq!(Type::combat1(&Dark, &Ground), 1.0);
        assert_eq!(Type::combat1(&Dark, &Rock), 1.0);
        assert_eq!(Type::combat1(&Dark, &Bug), 1.0);
        assert_eq!(Type::combat1(&Dark, &Ghost), 2.0);
        assert_eq!(Type::combat1(&Dark, &Steel), 1.0);
        assert_eq!(Type::combat1(&Dark, &Fire), 1.0);
        assert_eq!(Type::combat1(&Dark, &Water), 1.0);
        assert_eq!(Type::combat1(&Dark, &Grass), 1.0);
        assert_eq!(Type::combat1(&Dark, &Electric), 1.0);
        assert_eq!(Type::combat1(&Dark, &Psychic), 2.0);
        assert_eq!(Type::combat1(&Dark, &Ice), 1.0);
        assert_eq!(Type::combat1(&Dark, &Dragon), 1.0);
        assert_eq!(Type::combat1(&Dark, &Dark), 0.5);
        assert_eq!(Type::combat1(&Dark, &Fairy), 0.5);
    }

    #[test]
    fn combat_fairy_to() {
        use super::Type::*;
        assert_eq!(Type::combat1(&Fairy, &Normal), 1.0);
        assert_eq!(Type::combat1(&Fairy, &Fight), 2.0);
        assert_eq!(Type::combat1(&Fairy, &Flying), 1.0);
        assert_eq!(Type::combat1(&Fairy, &Poison), 0.5);
        assert_eq!(Type::combat1(&Fairy, &Ground), 1.0);
        assert_eq!(Type::combat1(&Fairy, &Rock), 1.0);
        assert_eq!(Type::combat1(&Fairy, &Bug), 1.0);
        assert_eq!(Type::combat1(&Fairy, &Ghost), 1.0);
        assert_eq!(Type::combat1(&Fairy, &Steel), 0.5);
        assert_eq!(Type::combat1(&Fairy, &Fire), 0.5);
        assert_eq!(Type::combat1(&Fairy, &Water), 1.0);
        assert_eq!(Type::combat1(&Fairy, &Grass), 1.0);
        assert_eq!(Type::combat1(&Fairy, &Electric), 1.0);
        assert_eq!(Type::combat1(&Fairy, &Psychic), 1.0);
        assert_eq!(Type::combat1(&Fairy, &Ice), 1.0);
        assert_eq!(Type::combat1(&Fairy, &Dragon), 2.0);
        assert_eq!(Type::combat1(&Fairy, &Dark), 2.0);
        assert_eq!(Type::combat1(&Fairy, &Fairy), 1.0);
    }

    #[test]
    fn to_str_jp() {
        use super::Type::*;
        use Localization::Japanese as Jp;
        assert_eq!(Normal.to_str(&Jp), "ノーマル");
        assert_eq!(Fight.to_str(&Jp), "格闘");
        assert_eq!(Flying.to_str(&Jp), "飛行");
        assert_eq!(Poison.to_str(&Jp), "毒");
        assert_eq!(Ground.to_str(&Jp), "地面");
        assert_eq!(Rock.to_str(&Jp), "岩");
        assert_eq!(Bug.to_str(&Jp), "虫");
        assert_eq!(Ghost.to_str(&Jp), "ゴースト");
        assert_eq!(Steel.to_str(&Jp), "鋼");
        assert_eq!(Fire.to_str(&Jp), "炎");
        assert_eq!(Water.to_str(&Jp), "水");
        assert_eq!(Grass.to_str(&Jp), "草");
        assert_eq!(Electric.to_str(&Jp), "雷");
        assert_eq!(Psychic.to_str(&Jp), "エスパー");
        assert_eq!(Ice.to_str(&Jp), "氷");
        assert_eq!(Dragon.to_str(&Jp), "ドラゴン");
        assert_eq!(Dark.to_str(&Jp), "悪");
        assert_eq!(Fairy.to_str(&Jp), "フェアリー");
    }

    #[test]
    fn to_str_en() {
        use super::Type::*;
        use Localization::English as En;
        assert_eq!(Normal.to_str(&En), "Normal");
        assert_eq!(Fight.to_str(&En), "Fight");
        assert_eq!(Flying.to_str(&En), "Flying");
        assert_eq!(Poison.to_str(&En), "Poison");
        assert_eq!(Ground.to_str(&En), "Ground");
        assert_eq!(Rock.to_str(&En), "Rock");
        assert_eq!(Bug.to_str(&En), "Bug");
        assert_eq!(Ghost.to_str(&En), "Ghost");
        assert_eq!(Steel.to_str(&En), "Steel");
        assert_eq!(Fire.to_str(&En), "Fire");
        assert_eq!(Water.to_str(&En), "Water");
        assert_eq!(Grass.to_str(&En), "Grass");
        assert_eq!(Electric.to_str(&En), "Electric");
        assert_eq!(Psychic.to_str(&En), "Psychic");
        assert_eq!(Ice.to_str(&En), "Ice");
        assert_eq!(Dragon.to_str(&En), "Dragon");
        assert_eq!(Dark.to_str(&En), "Dark");
        assert_eq!(Fairy.to_str(&En), "Fairy");
    }
}
