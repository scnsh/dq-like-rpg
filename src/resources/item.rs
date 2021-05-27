use std::fmt;
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum Item {
    Sword,
    IronBody,
    IronLeg,
    SpellHeal1,
    SpellHeal2,
    SpellHeal3,
    SpellFire1,
    SpellFire2,
    SpellFire3,
}
impl Default for Item {
    fn default() -> Self { Item::Sword }
}
impl Display for Item {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

