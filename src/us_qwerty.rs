#[derive(Clone, Copy)]
pub enum SetModifier {
    Lowercase,
    Shift,
    Alt,
    AltShift,
    Control,
    ControlAlt,
    ControlAltShift,
    Num,
}

#[derive(Clone, Copy)]
pub enum Key {
    Char(char),
    LCTRL,
    RCTRL,
    LSHIFT,
    RSHIFT,
    LALT,
    RALT,
    CALOCK,
    NLOCK,
    SLOCK,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    END,
    DOWN,
    PGDN,
    LEFT,
    MID,
    RIGHT,
    HOME,
    UP,
    PGUP,
    INSRT,
    DEL,
    MIN,
    PLUS,
}

#[derive(Clone, Copy)]
pub struct Set {
    pub key: Key,
    pub set_modifier: Option<SetModifier>,
}

#[derive(Clone, Copy)]
pub struct KeySet {
    pub not_shift: Set,
    pub shift: Set,
    pub alt1: Set,
    pub alt2: Set,
    pub alt_shift: Set,
    pub ctrl: Set,
}

pub static US_QUERTY: &'static [Option<KeySet>] = &[
    None,
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('['),
            set_modifier: Some(SetModifier::Control),
        },
        shift: Set {
            key: Key::Char('['),
            set_modifier: Some(SetModifier::Control),
        },
        alt1: Set {
            key: Key::Char('['),
            set_modifier: Some(SetModifier::ControlAlt),
        },
        alt2: Set {
            key: Key::Char('['),
            set_modifier: Some(SetModifier::ControlAlt),
        },
        alt_shift: Set {
            key: Key::Char('['),
            set_modifier: Some(SetModifier::ControlAlt),
        },
        ctrl: Set {
            key: Key::Char('['),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('1'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('!'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('1'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('1'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('1'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('A'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('2'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('@'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('2'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('2'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('@'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('@'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('3'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('#'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('3'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('3'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('#'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('C'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('4'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('$'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('4'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('4'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('$'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('D'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('5'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('%'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('5'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('5'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('%'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('E'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('6'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('^'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('6'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('6'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('^'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('^'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('7'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('&'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('7'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('7'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('&'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('G'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('8'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('*'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('8'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('8'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('*'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('H'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('9'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('('),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('9'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('9'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('('),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('I'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('0'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char(')'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('0'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('0'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char(')'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('@'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('-'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('_'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('-'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('-'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('_'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('_'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('='),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('+'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('='),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('='),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('+'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('@'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('H'),
            set_modifier: Some(SetModifier::Control),
        },
        shift: Set {
            key: Key::Char('H'),
            set_modifier: Some(SetModifier::Control),
        },
        alt1: Set {
            key: Key::Char('H'),
            set_modifier: Some(SetModifier::ControlAlt),
        },
        alt2: Set {
            key: Key::Char('H'),
            set_modifier: Some(SetModifier::ControlAlt),
        },
        alt_shift: Set {
            key: Key::Char('H'),
            set_modifier: Some(SetModifier::ControlAlt),
        },
        ctrl: Set {
            key: Key::DEL,
            set_modifier: None,
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('I'),
            set_modifier: Some(SetModifier::Control),
        },
        shift: Set {
            key: Key::Char('I'),
            set_modifier: Some(SetModifier::Control),
        },
        alt1: Set {
            key: Key::Char('I'),
            set_modifier: Some(SetModifier::ControlAlt),
        },
        alt2: Set {
            key: Key::Char('I'),
            set_modifier: Some(SetModifier::ControlAlt),
        },
        alt_shift: Set {
            key: Key::Char('I'),
            set_modifier: Some(SetModifier::ControlAlt),
        },
        ctrl: Set {
            key: Key::Char('I'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('q'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('Q'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('q'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('q'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('Q'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('Q'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('w'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('W'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('w'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('w'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('W'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('W'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('e'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('E'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('e'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('e'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('E'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('E'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('r'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('R'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('r'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('r'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('R'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('R'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('t'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('T'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('t'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('t'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('T'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('T'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('y'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('Y'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('y'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('y'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('Y'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('Y'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('u'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('U'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('u'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('u'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('U'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('U'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('i'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('I'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('i'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('i'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('I'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('I'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('o'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('O'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('o'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('o'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('O'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('O'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('p'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('P'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('p'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('p'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('P'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('P'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('['),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('{'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('['),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('['),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('{'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('['),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char(']'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('}'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char(']'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char(']'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('}'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char(']'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('M'),
            set_modifier: Some(SetModifier::Control),
        },
        shift: Set {
            key: Key::Char('M'),
            set_modifier: Some(SetModifier::Control),
        },
        alt1: Set {
            key: Key::Char('M'),
            set_modifier: Some(SetModifier::ControlAlt),
        },
        alt2: Set {
            key: Key::Char('M'),
            set_modifier: Some(SetModifier::ControlAlt),
        },
        alt_shift: Set {
            key: Key::Char('M'),
            set_modifier: Some(SetModifier::ControlAlt),
        },
        ctrl: Set {
            key: Key::Char('J'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::LCTRL,
            set_modifier: None,
        },
        shift: Set {
            key: Key::LCTRL,
            set_modifier: None,
        },
        alt1: Set {
            key: Key::LCTRL,
            set_modifier: None,
        },
        alt2: Set {
            key: Key::LCTRL,
            set_modifier: None,
        },
        alt_shift: Set {
            key: Key::LCTRL,
            set_modifier: None,
        },
        ctrl: Set {
            key: Key::LCTRL,
            set_modifier: None,
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('a'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('A'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('a'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('a'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('A'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('A'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('s'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('S'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('s'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('s'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('S'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('S'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('d'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('D'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('d'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('d'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('D'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('D'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('f'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('F'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('f'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('f'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('F'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('F'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('g'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('G'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('g'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('g'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('G'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('G'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('h'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('H'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('h'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('h'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('H'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('H'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('j'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('J'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('j'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('j'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('J'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('J'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('k'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('K'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('k'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('k'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('K'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('K'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('l'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('L'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('l'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('l'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('L'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('L'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char(';'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char(':'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char(';'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char(';'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char(':'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('@'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('\''),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('"'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('\''),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('\''),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('"'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('@'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('`'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('~'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('`'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('`'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('~'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('@'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::LSHIFT,
            set_modifier: None,
        },
        shift: Set {
            key: Key::LSHIFT,
            set_modifier: None,
        },
        alt1: Set {
            key: Key::LSHIFT,
            set_modifier: None,
        },
        alt2: Set {
            key: Key::LSHIFT,
            set_modifier: None,
        },
        alt_shift: Set {
            key: Key::LSHIFT,
            set_modifier: None,
        },
        ctrl: Set {
            key: Key::LSHIFT,
            set_modifier: None,
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('\\'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('|'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('\\'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('\\'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('|'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('\\'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('z'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('Z'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('z'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('z'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('Z'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('Z'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('x'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('X'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('x'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('x'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('X'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('X'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('c'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('C'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('c'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('c'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('C'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('C'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('v'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('V'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('v'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('v'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('V'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('V'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('b'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('B'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('b'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('b'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('B'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('B'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('n'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('N'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('n'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('n'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('N'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('N'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('m'),
            set_modifier: Some(SetModifier::Lowercase),
        },
        shift: Set {
            key: Key::Char('M'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('m'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('m'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('M'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('M'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char(','),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('<'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char(','),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char(','),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('<'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('@'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('.'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('>'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('.'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('.'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('>'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('@'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('/'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('?'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('/'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('/'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('?'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('@'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::RSHIFT,
            set_modifier: None,
        },
        shift: Set {
            key: Key::RSHIFT,
            set_modifier: None,
        },
        alt1: Set {
            key: Key::RSHIFT,
            set_modifier: None,
        },
        alt2: Set {
            key: Key::RSHIFT,
            set_modifier: None,
        },
        alt_shift: Set {
            key: Key::RSHIFT,
            set_modifier: None,
        },
        ctrl: Set {
            key: Key::RSHIFT,
            set_modifier: None,
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('*'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('*'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char('*'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char('*'),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('*'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('@'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::LALT,
            set_modifier: None,
        },
        shift: Set {
            key: Key::LALT,
            set_modifier: None,
        },
        alt1: Set {
            key: Key::LALT,
            set_modifier: None,
        },
        alt2: Set {
            key: Key::LALT,
            set_modifier: None,
        },
        alt_shift: Set {
            key: Key::LALT,
            set_modifier: None,
        },
        ctrl: Set {
            key: Key::LALT,
            set_modifier: None,
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char(' '),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char(' '),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::Char(' '),
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::Char(' '),
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char(' '),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::Char('@'),
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::CALOCK,
            set_modifier: None,
        },
        shift: Set {
            key: Key::CALOCK,
            set_modifier: None,
        },
        alt1: Set {
            key: Key::CALOCK,
            set_modifier: None,
        },
        alt2: Set {
            key: Key::CALOCK,
            set_modifier: None,
        },
        alt_shift: Set {
            key: Key::CALOCK,
            set_modifier: None,
        },
        ctrl: Set {
            key: Key::CALOCK,
            set_modifier: None,
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::F1,
            set_modifier: None,
        },
        shift: Set {
            key: Key::F1,
            set_modifier: Some(SetModifier::Shift),
        },
        alt1: Set {
            key: Key::F1,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::F1,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::F1,
            set_modifier: Some(SetModifier::AltShift),
        },
        ctrl: Set {
            key: Key::F1,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::F2,
            set_modifier: None,
        },
        shift: Set {
            key: Key::F2,
            set_modifier: Some(SetModifier::Shift),
        },
        alt1: Set {
            key: Key::F2,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::F2,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::F2,
            set_modifier: Some(SetModifier::AltShift),
        },
        ctrl: Set {
            key: Key::F2,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::F3,
            set_modifier: None,
        },
        shift: Set {
            key: Key::F3,
            set_modifier: Some(SetModifier::Shift),
        },
        alt1: Set {
            key: Key::F3,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::F3,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::F3,
            set_modifier: Some(SetModifier::AltShift),
        },
        ctrl: Set {
            key: Key::F3,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::F4,
            set_modifier: None,
        },
        shift: Set {
            key: Key::F4,
            set_modifier: Some(SetModifier::Shift),
        },
        alt1: Set {
            key: Key::F4,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::F4,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::F4,
            set_modifier: Some(SetModifier::AltShift),
        },
        ctrl: Set {
            key: Key::F4,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::F5,
            set_modifier: None,
        },
        shift: Set {
            key: Key::F5,
            set_modifier: Some(SetModifier::Shift),
        },
        alt1: Set {
            key: Key::F5,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::F5,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::F5,
            set_modifier: Some(SetModifier::AltShift),
        },
        ctrl: Set {
            key: Key::F5,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::F6,
            set_modifier: None,
        },
        shift: Set {
            key: Key::F6,
            set_modifier: Some(SetModifier::Shift),
        },
        alt1: Set {
            key: Key::F6,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::F6,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::F6,
            set_modifier: Some(SetModifier::AltShift),
        },
        ctrl: Set {
            key: Key::F6,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::F7,
            set_modifier: None,
        },
        shift: Set {
            key: Key::F7,
            set_modifier: Some(SetModifier::Shift),
        },
        alt1: Set {
            key: Key::F7,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::F7,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::F7,
            set_modifier: Some(SetModifier::AltShift),
        },
        ctrl: Set {
            key: Key::F7,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::F8,
            set_modifier: None,
        },
        shift: Set {
            key: Key::F8,
            set_modifier: Some(SetModifier::Shift),
        },
        alt1: Set {
            key: Key::F8,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::F8,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::F8,
            set_modifier: Some(SetModifier::AltShift),
        },
        ctrl: Set {
            key: Key::F8,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::F9,
            set_modifier: None,
        },
        shift: Set {
            key: Key::F9,
            set_modifier: Some(SetModifier::Shift),
        },
        alt1: Set {
            key: Key::F9,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::F9,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::F9,
            set_modifier: Some(SetModifier::AltShift),
        },
        ctrl: Set {
            key: Key::F9,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::F10,
            set_modifier: None,
        },
        shift: Set {
            key: Key::F10,
            set_modifier: Some(SetModifier::Shift),
        },
        alt1: Set {
            key: Key::F10,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::F10,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::F10,
            set_modifier: Some(SetModifier::AltShift),
        },
        ctrl: Set {
            key: Key::F10,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::NLOCK,
            set_modifier: None,
        },
        shift: Set {
            key: Key::NLOCK,
            set_modifier: None,
        },
        alt1: Set {
            key: Key::NLOCK,
            set_modifier: None,
        },
        alt2: Set {
            key: Key::NLOCK,
            set_modifier: None,
        },
        alt_shift: Set {
            key: Key::NLOCK,
            set_modifier: None,
        },
        ctrl: Set {
            key: Key::NLOCK,
            set_modifier: None,
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::SLOCK,
            set_modifier: None,
        },
        shift: Set {
            key: Key::SLOCK,
            set_modifier: None,
        },
        alt1: Set {
            key: Key::SLOCK,
            set_modifier: None,
        },
        alt2: Set {
            key: Key::SLOCK,
            set_modifier: None,
        },
        alt_shift: Set {
            key: Key::SLOCK,
            set_modifier: None,
        },
        ctrl: Set {
            key: Key::SLOCK,
            set_modifier: None,
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::HOME,
            set_modifier: Some(SetModifier::Num),
        },
        shift: Set {
            key: Key::Char('7'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::HOME,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::HOME,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('7'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::HOME,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::UP,
            set_modifier: Some(SetModifier::Num),
        },
        shift: Set {
            key: Key::Char('8'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::UP,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::UP,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('8'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::UP,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::PGUP,
            set_modifier: Some(SetModifier::Num),
        },
        shift: Set {
            key: Key::Char('9'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::PGUP,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::PGUP,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('9'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::PGUP,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('-'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('-'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::MIN,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::MIN,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('-'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::MIN,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::LEFT,
            set_modifier: Some(SetModifier::Num),
        },
        shift: Set {
            key: Key::Char('4'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::LEFT,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::LEFT,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('4'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::LEFT,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::MID,
            set_modifier: Some(SetModifier::Num),
        },
        shift: Set {
            key: Key::Char('5'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::MID,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::MID,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('5'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::MID,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::RIGHT,
            set_modifier: Some(SetModifier::Num),
        },
        shift: Set {
            key: Key::Char('6'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::RIGHT,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::RIGHT,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('6'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::RIGHT,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::Char('+'),
            set_modifier: None,
        },
        shift: Set {
            key: Key::Char('+'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::PLUS,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::PLUS,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('+'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::PLUS,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::END,
            set_modifier: Some(SetModifier::Num),
        },
        shift: Set {
            key: Key::Char('1'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::END,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::END,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('1'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::END,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::DOWN,
            set_modifier: Some(SetModifier::Num),
        },
        shift: Set {
            key: Key::Char('2'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::DOWN,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::DOWN,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('2'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::DOWN,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::PGDN,
            set_modifier: Some(SetModifier::Num),
        },
        shift: Set {
            key: Key::Char('3'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::PGDN,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::PGDN,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('3'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::PGDN,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::INSRT,
            set_modifier: Some(SetModifier::Num),
        },
        shift: Set {
            key: Key::Char('0'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::INSRT,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::INSRT,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('0'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::INSRT,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::DEL,
            set_modifier: Some(SetModifier::Num),
        },
        shift: Set {
            key: Key::Char('.'),
            set_modifier: None,
        },
        alt1: Set {
            key: Key::DEL,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::DEL,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::Char('.'),
            set_modifier: Some(SetModifier::Alt),
        },
        ctrl: Set {
            key: Key::DEL,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    None,
    None,
    None,
    Some(KeySet {
        not_shift: Set {
            key: Key::F11,
            set_modifier: None,
        },
        shift: Set {
            key: Key::F11,
            set_modifier: Some(SetModifier::Shift),
        },
        alt1: Set {
            key: Key::F11,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::F11,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::F11,
            set_modifier: Some(SetModifier::AltShift),
        },
        ctrl: Set {
            key: Key::F11,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    Some(KeySet {
        not_shift: Set {
            key: Key::F12,
            set_modifier: None,
        },
        shift: Set {
            key: Key::F12,
            set_modifier: Some(SetModifier::Shift),
        },
        alt1: Set {
            key: Key::F12,
            set_modifier: Some(SetModifier::Alt),
        },
        alt2: Set {
            key: Key::F12,
            set_modifier: Some(SetModifier::Alt),
        },
        alt_shift: Set {
            key: Key::F12,
            set_modifier: Some(SetModifier::AltShift),
        },
        ctrl: Set {
            key: Key::F12,
            set_modifier: Some(SetModifier::Control),
        },
    }),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];
