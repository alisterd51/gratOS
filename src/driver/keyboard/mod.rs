use alloc::collections::BTreeMap;

pub mod keymaps;
mod fifo_buffer;
pub mod ps2;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScanCodeValue {
    Escape,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Min,
    Equal,
    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LeftSquareBracket,
    RightSquareBracket,
    Enter,
    LeftControl,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    SingleQuote,
    BackTick,
    LeftShift,
    BackSlash,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    Slash,
    RightShift,
    KeypadStar,
    LeftAlt,
    Space,
    CapsLock,
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
    NumberLock,
    ScrollLock,
    Keypad7,
    Keypad8,
    Keypad9,
    KeypadMin,
    Keypad4,
    Keypad5,
    Keypad6,
    KeypadPlus,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad0,
    KeypadDot,
    F11,
    F12,
    MultimediaPreviousTrack,
    MultimediaNextTrack,
    KeypadEnter,
    RightControl,
    MultimediaMute,
    MultimediaCalculator,
    MultimediaPlay,
    MultimediaStop,
    MultimediaVolumeDown,
    MultimediaVolumeUp,
    MultimediaWWWHome,
    KeypadSlash,
    RightAlt,
    Home,
    CursorUp,
    PageUp,
    CursorLeft,
    CursorRight,
    End,
    CursorDown,
    PageDown,
    Insert,
    Delete,
    LeftGUI,
    RightGUI,
    Apps,
    ACPIPower,
    ACPISleep,
    ACPIWake,
    MultimediaWWWSearch,
    MultimediaWWWFavorites,
    MultimediaWWWRefresh,
    MultimediaWWWStop,
    MultimediaWWWForward,
    MultimediaWWWBack,
    MultimediaMyComputer,
    MultimediaEmail,
    MultimediaMediaSelect,
    PrintScreen,
    Pause,
}

struct ScanCodeSet {
    map: BTreeMap<u64, ScanCodeValue>,
}

// https://wiki.osdev.org/PS/2_Keyboard#Scan_Code_Set_1
#[allow(clippy::too_many_lines)]
impl ScanCodeSet {
    fn new() -> Self {
        let mut map: BTreeMap<u64, ScanCodeValue> = BTreeMap::new();

        map.insert(0x01, ScanCodeValue::Escape);
        map.insert(0x02, ScanCodeValue::One);
        map.insert(0x03, ScanCodeValue::Two);
        map.insert(0x04, ScanCodeValue::Three);
        map.insert(0x05, ScanCodeValue::Four);
        map.insert(0x06, ScanCodeValue::Five);
        map.insert(0x07, ScanCodeValue::Six);
        map.insert(0x08, ScanCodeValue::Seven);
        map.insert(0x09, ScanCodeValue::Eight);
        map.insert(0x0A, ScanCodeValue::Nine);
        map.insert(0x0B, ScanCodeValue::Zero);
        map.insert(0x0C, ScanCodeValue::Min);
        map.insert(0x0D, ScanCodeValue::Equal);
        map.insert(0x0E, ScanCodeValue::Backspace);
        map.insert(0x0F, ScanCodeValue::Tab);
        map.insert(0x10, ScanCodeValue::Q);
        map.insert(0x11, ScanCodeValue::W);
        map.insert(0x12, ScanCodeValue::E);
        map.insert(0x13, ScanCodeValue::R);
        map.insert(0x14, ScanCodeValue::T);
        map.insert(0x15, ScanCodeValue::Y);
        map.insert(0x16, ScanCodeValue::U);
        map.insert(0x17, ScanCodeValue::I);
        map.insert(0x18, ScanCodeValue::O);
        map.insert(0x19, ScanCodeValue::P);
        map.insert(0x1A, ScanCodeValue::LeftSquareBracket);
        map.insert(0x1B, ScanCodeValue::RightSquareBracket);
        map.insert(0x1C, ScanCodeValue::Enter);
        map.insert(0x1D, ScanCodeValue::LeftControl);
        map.insert(0x1E, ScanCodeValue::A);
        map.insert(0x1F, ScanCodeValue::S);
        map.insert(0x20, ScanCodeValue::D);
        map.insert(0x21, ScanCodeValue::F);
        map.insert(0x22, ScanCodeValue::G);
        map.insert(0x23, ScanCodeValue::H);
        map.insert(0x24, ScanCodeValue::J);
        map.insert(0x25, ScanCodeValue::K);
        map.insert(0x26, ScanCodeValue::L);
        map.insert(0x27, ScanCodeValue::Semicolon);
        map.insert(0x28, ScanCodeValue::SingleQuote);
        map.insert(0x29, ScanCodeValue::BackTick);
        map.insert(0x2A, ScanCodeValue::LeftShift);
        map.insert(0x2B, ScanCodeValue::BackSlash);
        map.insert(0x2C, ScanCodeValue::Z);
        map.insert(0x2D, ScanCodeValue::X);
        map.insert(0x2E, ScanCodeValue::C);
        map.insert(0x2F, ScanCodeValue::V);
        map.insert(0x30, ScanCodeValue::B);
        map.insert(0x31, ScanCodeValue::N);
        map.insert(0x32, ScanCodeValue::M);
        map.insert(0x33, ScanCodeValue::Comma);
        map.insert(0x34, ScanCodeValue::Dot);
        map.insert(0x35, ScanCodeValue::Slash);
        map.insert(0x36, ScanCodeValue::RightShift);
        map.insert(0x37, ScanCodeValue::KeypadStar);
        map.insert(0x38, ScanCodeValue::LeftAlt);
        map.insert(0x39, ScanCodeValue::Space);
        map.insert(0x3A, ScanCodeValue::CapsLock);
        map.insert(0x3B, ScanCodeValue::F1);
        map.insert(0x3C, ScanCodeValue::F2);
        map.insert(0x3D, ScanCodeValue::F3);
        map.insert(0x3E, ScanCodeValue::F4);
        map.insert(0x3F, ScanCodeValue::F5);
        map.insert(0x40, ScanCodeValue::F6);
        map.insert(0x41, ScanCodeValue::F7);
        map.insert(0x42, ScanCodeValue::F8);
        map.insert(0x43, ScanCodeValue::F9);
        map.insert(0x44, ScanCodeValue::F10);
        map.insert(0x45, ScanCodeValue::NumberLock);
        map.insert(0x46, ScanCodeValue::ScrollLock);
        map.insert(0x47, ScanCodeValue::Keypad7);
        map.insert(0x48, ScanCodeValue::Keypad8);
        map.insert(0x49, ScanCodeValue::Keypad9);
        map.insert(0x4A, ScanCodeValue::KeypadMin);
        map.insert(0x4B, ScanCodeValue::Keypad4);
        map.insert(0x4C, ScanCodeValue::Keypad5);
        map.insert(0x4D, ScanCodeValue::Keypad6);
        map.insert(0x4E, ScanCodeValue::KeypadPlus);
        map.insert(0x4F, ScanCodeValue::Keypad1);
        map.insert(0x50, ScanCodeValue::Keypad2);
        map.insert(0x51, ScanCodeValue::Keypad3);
        map.insert(0x52, ScanCodeValue::Keypad0);
        map.insert(0x53, ScanCodeValue::KeypadDot);
        map.insert(0x57, ScanCodeValue::F11);
        map.insert(0x58, ScanCodeValue::F12);

        map.insert(0xE010, ScanCodeValue::MultimediaPreviousTrack);
        map.insert(0xE019, ScanCodeValue::MultimediaNextTrack);
        map.insert(0xE01C, ScanCodeValue::KeypadEnter);
        map.insert(0xE01D, ScanCodeValue::RightControl);
        map.insert(0xE020, ScanCodeValue::MultimediaMute);
        map.insert(0xE021, ScanCodeValue::MultimediaCalculator);
        map.insert(0xE022, ScanCodeValue::MultimediaPlay);
        map.insert(0xE024, ScanCodeValue::MultimediaStop);
        map.insert(0xE02E, ScanCodeValue::MultimediaVolumeDown);
        map.insert(0xE030, ScanCodeValue::MultimediaVolumeUp);
        map.insert(0xE032, ScanCodeValue::MultimediaWWWHome);
        map.insert(0xE035, ScanCodeValue::KeypadSlash);
        map.insert(0xE038, ScanCodeValue::RightAlt);
        map.insert(0xE047, ScanCodeValue::Home);
        map.insert(0xE048, ScanCodeValue::CursorUp);
        map.insert(0xE049, ScanCodeValue::PageUp);
        map.insert(0xE04B, ScanCodeValue::CursorLeft);
        map.insert(0xE04D, ScanCodeValue::CursorRight);
        map.insert(0xE04F, ScanCodeValue::End);
        map.insert(0xE050, ScanCodeValue::CursorDown);
        map.insert(0xE051, ScanCodeValue::PageDown);
        map.insert(0xE052, ScanCodeValue::Insert);
        map.insert(0xE053, ScanCodeValue::Delete);
        map.insert(0xE05B, ScanCodeValue::LeftGUI);
        map.insert(0xE05C, ScanCodeValue::RightGUI);
        map.insert(0xE05D, ScanCodeValue::Apps);
        map.insert(0xE05E, ScanCodeValue::ACPIPower);
        map.insert(0xE05F, ScanCodeValue::ACPISleep);
        map.insert(0xE063, ScanCodeValue::ACPIWake);
        map.insert(0xE065, ScanCodeValue::MultimediaWWWSearch);
        map.insert(0xE066, ScanCodeValue::MultimediaWWWFavorites);
        map.insert(0xE067, ScanCodeValue::MultimediaWWWRefresh);
        map.insert(0xE068, ScanCodeValue::MultimediaWWWStop);
        map.insert(0xE069, ScanCodeValue::MultimediaWWWForward);
        map.insert(0xE06A, ScanCodeValue::MultimediaWWWBack);
        map.insert(0xE06B, ScanCodeValue::MultimediaMyComputer);
        map.insert(0xE06C, ScanCodeValue::MultimediaEmail);
        map.insert(0xE06D, ScanCodeValue::MultimediaMediaSelect);

        map.insert(0xE02A_E037, ScanCodeValue::Pause);
        map.insert(0xE11D_45E1_9DC5, ScanCodeValue::PrintScreen);

        Self { map }
    }

    fn from(&self, scancode: u64) -> Option<ScanCodeValue> {
        let scancode = match scancode {
            0xE0B7_E0AA => 0xE02A_E037,
            0xE11D_45E1_9DC5 => 0xE11D_45E1_9DC5,
            scancode if (scancode >> 8) == 0xE0 => scancode & 0xFF7F,
            scancode if scancode <= 0xFF => scancode & 0x7F,
            scancode => scancode,
        };

        self.map.get(&scancode).copied()
    }
}
