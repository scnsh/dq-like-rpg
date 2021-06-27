use crate::resources::Skill;
use std::fmt;
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Eq, PartialEq, Copy, Debug, Hash)]
pub enum Item {
    SpellHeal(u32),
    SpellFire(u32),
    SpellIce(u32),
    IronBody,
    IronArm,
    IronLeg,
    IronHead,
    HeroSword,
    WisdomRing,
    FairyShield,
}
// impl Default for Item {
//     fn default() -> Self { Item::Sword }
// }
impl Display for Item {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}
impl Item {
    pub fn can_use(&self) -> Option<Skill> {
        match self {
            Self::SpellFire(lv) => Some(Skill::Spell(Item::SpellFire(*lv))),
            Self::SpellHeal(lv) => Some(Skill::Spell(Item::SpellHeal(*lv))),
            Self::SpellIce(lv) => Some(Skill::Spell(Item::SpellIce(*lv))),
            _ => None,
        }
    }
}

pub fn generate_items() -> Vec<Item> {
    vec![
        Item::SpellHeal(1),
        Item::SpellHeal(2),
        Item::SpellHeal(3),
        Item::SpellFire(1),
        Item::SpellFire(2),
        Item::SpellFire(3),
        Item::SpellIce(1),
        Item::SpellIce(2),
        Item::SpellIce(3),
        Item::IronBody,
        Item::IronArm,
        Item::IronLeg,
        Item::IronHead,
        Item::HeroSword,
        Item::WisdomRing,
        Item::FairyShield,
    ]
}
