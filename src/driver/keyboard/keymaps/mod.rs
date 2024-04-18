pub mod us_qwerty;

pub type Keymap = [Option<KeymapSet>; 0x80];

#[derive(Clone, Copy)]
pub enum KeymapValue {
    Ascii(char),
    Lowercase(char),
    Alt(char),
    Control(char),
    ControlAlt(char),
    CapsLock,
    NumberLock,
    ScrollLock,
    LeftControl,
    LeftShift,
    LeftAlt,
    RightControl,
    RightShift,
    RightAlt,
    Delete,
    Right,
    AltRight,
    ControlRight,
    Left,
    AltLeft,
    ControlLeft,
    Down,
    AltDown,
    ControlDown,
    Up,
    AltUp,
    ControlUp,
}

#[derive(Clone, Copy)]
pub struct KeymapSet {
    pub not_shift: KeymapValue,
    pub shift: KeymapValue,
    pub alt1: KeymapValue,
    pub alt2: KeymapValue,
    pub alt_shift: KeymapValue,
    pub ctrl: KeymapValue,
}
