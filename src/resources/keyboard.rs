use specs::prelude::*;

#[derive(Default, Debug)]
pub struct Keyboard {
    pub pressing: BitSet,

    pub just_pressed: BitSet,
    pub just_released: BitSet,
}

#[derive(Copy, Clone, Debug)]
pub enum Key {
    A, B, C,
}

impl Key {
    pub fn lookup(mut code: u32) -> Option<Self> {
        if (97..=122).contains(&code) {
            code -= 32;
        }

        match code {
            65 => Some(Key::A),
            66 => Some(Key::B),
            67 => Some(Key::C),

            _ => None,
        }
    }
}
