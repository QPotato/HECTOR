extern crate uid;

pub use super::frame::{Frame};
pub use super::Access;
pub type Label = uid::Id<u16>;
type LocalTemp = uid::Id<u16>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Temp {
    FRAME_POINTER,
    // Other special temps.
    Local(LocalTemp),
}
// unsafe? Is there a pure way?
// YES - use unique ids again.
// But, not sure how to make string from uid.
pub fn newtemp() -> Temp {
    //String::from("Not_implemented")
    Temp::Local(LocalTemp::new())
}

pub fn newlabel() -> Label {
    //String::from("Not_implemented")
    Label::new()
}

pub type LevelId = uid::Id<u16>;

#[derive(Clone, Debug)]
pub struct Level {
    pub frame: Frame,
    pub nesting_depth: i64,
    pub id: LevelId,
}

impl Level {
    pub fn outermost() -> Level {
        Level {
            frame: Frame::new(
                Label::new(),
                vec![],
            ),
            nesting_depth: -1,
            id: LevelId::new(),
        }
    }

    pub fn new(parent_level: Level, name: Label, formals: Vec<bool>) -> Level {
        Level {
            frame: Frame::new(name, formals),
            nesting_depth: parent_level.nesting_depth + 1,
            id: LevelId::new(),
        }
    }

    pub fn alloc_arg(self: &mut Self, escape: bool) -> Access {
        self.frame.alloc_arg(escape)
    }

    pub fn alloc_local(self: &mut Self, escape: bool)  -> Access {
        self.frame.alloc_local(escape)
    }
}