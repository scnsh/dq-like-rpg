use std::fmt;
use std::fmt::Display;
use crate::resources::Skill;

#[derive(Clone, Copy, Debug)]
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
            _ => None
        }
    }
}
