pub mod fifo_buffer;
pub mod keymaps;
pub mod ps2;

// TODO: construct key `Pause` (0xE02AE037) and `PrintScreen` (0xE11D45E19DC5)
#[allow(dead_code)]
#[derive(Clone, Copy)]
enum ScanCodeValue {
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

// https://wiki.osdev.org/PS/2_Keyboard#Scan_Code_Set_1
const SCAN_CODE_SET_1_0: [Option<ScanCodeValue>; 0x60] = {
    let mut scancodes = [None; 0x60];
    scancodes[0x01] = Some(ScanCodeValue::Escape);
    scancodes[0x02] = Some(ScanCodeValue::One);
    scancodes[0x03] = Some(ScanCodeValue::Two);
    scancodes[0x04] = Some(ScanCodeValue::Three);
    scancodes[0x05] = Some(ScanCodeValue::Four);
    scancodes[0x06] = Some(ScanCodeValue::Five);
    scancodes[0x07] = Some(ScanCodeValue::Six);
    scancodes[0x08] = Some(ScanCodeValue::Seven);
    scancodes[0x09] = Some(ScanCodeValue::Eight);
    scancodes[0x0A] = Some(ScanCodeValue::Nine);
    scancodes[0x0B] = Some(ScanCodeValue::Zero);
    scancodes[0x0C] = Some(ScanCodeValue::Min);
    scancodes[0x0D] = Some(ScanCodeValue::Equal);
    scancodes[0x0E] = Some(ScanCodeValue::Backspace);
    scancodes[0x0F] = Some(ScanCodeValue::Tab);
    scancodes[0x10] = Some(ScanCodeValue::Q);
    scancodes[0x11] = Some(ScanCodeValue::W);
    scancodes[0x12] = Some(ScanCodeValue::E);
    scancodes[0x13] = Some(ScanCodeValue::R);
    scancodes[0x14] = Some(ScanCodeValue::T);
    scancodes[0x15] = Some(ScanCodeValue::Y);
    scancodes[0x16] = Some(ScanCodeValue::U);
    scancodes[0x17] = Some(ScanCodeValue::I);
    scancodes[0x18] = Some(ScanCodeValue::O);
    scancodes[0x19] = Some(ScanCodeValue::P);
    scancodes[0x1A] = Some(ScanCodeValue::LeftSquareBracket);
    scancodes[0x1B] = Some(ScanCodeValue::RightSquareBracket);
    scancodes[0x1C] = Some(ScanCodeValue::Enter);
    scancodes[0x1D] = Some(ScanCodeValue::LeftControl);
    scancodes[0x1E] = Some(ScanCodeValue::A);
    scancodes[0x1F] = Some(ScanCodeValue::S);
    scancodes[0x20] = Some(ScanCodeValue::D);
    scancodes[0x21] = Some(ScanCodeValue::F);
    scancodes[0x22] = Some(ScanCodeValue::G);
    scancodes[0x23] = Some(ScanCodeValue::H);
    scancodes[0x24] = Some(ScanCodeValue::J);
    scancodes[0x25] = Some(ScanCodeValue::K);
    scancodes[0x26] = Some(ScanCodeValue::L);
    scancodes[0x27] = Some(ScanCodeValue::Semicolon);
    scancodes[0x28] = Some(ScanCodeValue::SingleQuote);
    scancodes[0x29] = Some(ScanCodeValue::BackTick);
    scancodes[0x2A] = Some(ScanCodeValue::LeftShift);
    scancodes[0x2B] = Some(ScanCodeValue::BackSlash);
    scancodes[0x2C] = Some(ScanCodeValue::Z);
    scancodes[0x2D] = Some(ScanCodeValue::X);
    scancodes[0x2E] = Some(ScanCodeValue::C);
    scancodes[0x2F] = Some(ScanCodeValue::V);
    scancodes[0x30] = Some(ScanCodeValue::B);
    scancodes[0x31] = Some(ScanCodeValue::N);
    scancodes[0x32] = Some(ScanCodeValue::M);
    scancodes[0x33] = Some(ScanCodeValue::Comma);
    scancodes[0x34] = Some(ScanCodeValue::Dot);
    scancodes[0x35] = Some(ScanCodeValue::Slash);
    scancodes[0x36] = Some(ScanCodeValue::RightShift);
    scancodes[0x37] = Some(ScanCodeValue::KeypadStar);
    scancodes[0x38] = Some(ScanCodeValue::LeftAlt);
    scancodes[0x39] = Some(ScanCodeValue::Space);
    scancodes[0x3A] = Some(ScanCodeValue::CapsLock);
    scancodes[0x3B] = Some(ScanCodeValue::F1);
    scancodes[0x3C] = Some(ScanCodeValue::F2);
    scancodes[0x3D] = Some(ScanCodeValue::F3);
    scancodes[0x3E] = Some(ScanCodeValue::F4);
    scancodes[0x3F] = Some(ScanCodeValue::F5);
    scancodes[0x40] = Some(ScanCodeValue::F6);
    scancodes[0x41] = Some(ScanCodeValue::F7);
    scancodes[0x42] = Some(ScanCodeValue::F8);
    scancodes[0x43] = Some(ScanCodeValue::F9);
    scancodes[0x44] = Some(ScanCodeValue::F10);
    scancodes[0x45] = Some(ScanCodeValue::NumberLock);
    scancodes[0x46] = Some(ScanCodeValue::ScrollLock);
    scancodes[0x47] = Some(ScanCodeValue::Keypad7);
    scancodes[0x48] = Some(ScanCodeValue::Keypad8);
    scancodes[0x49] = Some(ScanCodeValue::Keypad9);
    scancodes[0x4A] = Some(ScanCodeValue::KeypadMin);
    scancodes[0x4B] = Some(ScanCodeValue::Keypad4);
    scancodes[0x4C] = Some(ScanCodeValue::Keypad5);
    scancodes[0x4D] = Some(ScanCodeValue::Keypad6);
    scancodes[0x4E] = Some(ScanCodeValue::KeypadPlus);
    scancodes[0x4F] = Some(ScanCodeValue::Keypad1);
    scancodes[0x50] = Some(ScanCodeValue::Keypad2);
    scancodes[0x51] = Some(ScanCodeValue::Keypad3);
    scancodes[0x52] = Some(ScanCodeValue::Keypad0);
    scancodes[0x53] = Some(ScanCodeValue::KeypadDot);
    scancodes[0x57] = Some(ScanCodeValue::F11);
    scancodes[0x58] = Some(ScanCodeValue::F12);
    scancodes
};

const SCAN_CODE_SET_1_1: [Option<ScanCodeValue>; 0x70] = {
    let mut scancodes = [None; 0x70];
    scancodes[0x10] = Some(ScanCodeValue::MultimediaPreviousTrack);
    scancodes[0x19] = Some(ScanCodeValue::MultimediaNextTrack);
    scancodes[0x1C] = Some(ScanCodeValue::KeypadEnter);
    scancodes[0x1D] = Some(ScanCodeValue::RightControl);
    scancodes[0x20] = Some(ScanCodeValue::MultimediaMute);
    scancodes[0x21] = Some(ScanCodeValue::MultimediaCalculator);
    scancodes[0x22] = Some(ScanCodeValue::MultimediaPlay);
    scancodes[0x24] = Some(ScanCodeValue::MultimediaStop);
    scancodes[0x2E] = Some(ScanCodeValue::MultimediaVolumeDown);
    scancodes[0x30] = Some(ScanCodeValue::MultimediaVolumeUp);
    scancodes[0x32] = Some(ScanCodeValue::MultimediaWWWHome);
    scancodes[0x35] = Some(ScanCodeValue::KeypadSlash);
    scancodes[0x38] = Some(ScanCodeValue::RightAlt);
    scancodes[0x47] = Some(ScanCodeValue::Home);
    scancodes[0x48] = Some(ScanCodeValue::CursorUp);
    scancodes[0x49] = Some(ScanCodeValue::PageUp);
    scancodes[0x4B] = Some(ScanCodeValue::CursorLeft);
    scancodes[0x4D] = Some(ScanCodeValue::CursorRight);
    scancodes[0x4F] = Some(ScanCodeValue::End);
    scancodes[0x50] = Some(ScanCodeValue::CursorDown);
    scancodes[0x51] = Some(ScanCodeValue::PageDown);
    scancodes[0x52] = Some(ScanCodeValue::Insert);
    scancodes[0x53] = Some(ScanCodeValue::Delete);
    scancodes[0x5B] = Some(ScanCodeValue::LeftGUI);
    scancodes[0x5C] = Some(ScanCodeValue::RightGUI);
    scancodes[0x5D] = Some(ScanCodeValue::Apps);
    scancodes[0x5E] = Some(ScanCodeValue::ACPIPower);
    scancodes[0x5F] = Some(ScanCodeValue::ACPISleep);
    scancodes[0x63] = Some(ScanCodeValue::ACPIWake);
    scancodes[0x65] = Some(ScanCodeValue::MultimediaWWWSearch);
    scancodes[0x66] = Some(ScanCodeValue::MultimediaWWWFavorites);
    scancodes[0x67] = Some(ScanCodeValue::MultimediaWWWRefresh);
    scancodes[0x68] = Some(ScanCodeValue::MultimediaWWWStop);
    scancodes[0x69] = Some(ScanCodeValue::MultimediaWWWForward);
    scancodes[0x6A] = Some(ScanCodeValue::MultimediaWWWBack);
    scancodes[0x6B] = Some(ScanCodeValue::MultimediaMyComputer);
    scancodes[0x6C] = Some(ScanCodeValue::MultimediaEmail);
    scancodes[0x6D] = Some(ScanCodeValue::MultimediaMediaSelect);
    scancodes
};

struct ScanCodeSet {
    scan_code_set_1_0: &'static [Option<ScanCodeValue>; 0x60],
    scan_code_set_1_1: &'static [Option<ScanCodeValue>; 0x70],
}

impl ScanCodeSet {
    fn new() -> ScanCodeSet {
        ScanCodeSet {
            scan_code_set_1_0: &SCAN_CODE_SET_1_0,
            scan_code_set_1_1: &SCAN_CODE_SET_1_1,
        }
    }

    fn from(&mut self, scancode: u16) -> Option<ScanCodeValue> {
        let scancode = scancode & 0xFF7F;

        if scancode & 0xFF00 == 0x0 {
            return self.scan_code_set_1_0[scancode as usize];
        } else if scancode & 0xFF00 == 0xE000 {
            return self.scan_code_set_1_1[(scancode & 0xFF) as usize];
        }
        None
    }
}
