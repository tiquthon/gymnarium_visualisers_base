//! Mainly copied from https://docs.piston.rs/piston_window/piston_window/enum.Input.html

use std::path::PathBuf;
use std::cmp::Ordering;

/// Models input events.
#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum Input {
    /// Changed button state.
    Button(ButtonArgs),
    /// Moved mouse cursor.
    Move(Motion),
    /// Text (usually from keyboard).
    Text(String),
    /// Window got resized.
    Resize(ResizeArgs),
    /// Window gained or lost focus.
    Focus(bool),
    /// Window gained or lost cursor.
    Cursor(bool),
    /// A file is being dragged or dropped over the window.
    FileDrag(FileDrag),
    /// Window closed.
    Close(CloseArgs),
}

/// Button arguments.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ButtonArgs {
    /// New state of the button.
    pub state: ButtonState,
    /// The button that changed state.
    pub button: Button,
    /// An optional scancode that tells the physical layout of a keyboard key.
    /// For other devices than keyboard, this is set to `None`.
    ///
    /// Scancode follows SDL (https://wiki.libsdl.org/SDL_Scancode).
    ///
    /// This is stored here to make `Button` equality check work with keyboard layouts.
    ///
    /// Some window backends might not support scancodes.
    /// To test a window backend, use https://github.com/PistonDevelopers/piston-examples/tree/master/user_input
    pub scancode: Option<i32>,
}

/// Stores button state.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ButtonState {
    /// Button was pressed.
    Press,
    /// Button was released.
    Release,
}

/// Models different kinds of buttons.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
pub enum Button {
    /// A keyboard button.
    Keyboard(Key),
    /// A mouse button.
    Mouse(MouseButton),
    /// A controller button.
    Controller(ControllerButton),
    /// A controller hat (d-Pad)
    Hat(ControllerHat),
}

/// Represent a keyboard key.
/// Keycodes follows SDL http://wiki.libsdl.org/SDLKeycodeLookup
// Allowing clippy::derive_has_xor_eq, because I didn't create this code and it worked there.
#[allow(missing_docs, clippy::derive_hash_xor_eq)]
#[derive(Copy, Clone, Deserialize, Serialize, Debug, Hash)]
pub enum Key {
    Unknown = 0x00,
    Backspace = 0x08,
    Tab = 0x09,
    Return = 0x0D,
    Escape = 0x1B,
    Space = 0x20,
    Exclaim = 0x21,
    Quotedbl = 0x22,
    Hash = 0x23,
    Dollar = 0x24,
    Percent = 0x25,
    Ampersand = 0x26,
    Quote = 0x27,
    LeftParen = 0x28,
    RightParen = 0x29,
    Asterisk = 0x2A,
    Plus = 0x2B,
    Comma = 0x2C,
    Minus = 0x2D,
    Period = 0x2E,
    Slash = 0x2F,
    D0 = 0x30,
    D1 = 0x31,
    D2 = 0x32,
    D3 = 0x33,
    D4 = 0x34,
    D5 = 0x35,
    D6 = 0x36,
    D7 = 0x37,
    D8 = 0x38,
    D9 = 0x39,
    Colon = 0x3A,
    Semicolon = 0x3B,
    Less = 0x3C,
    Equals = 0x3D,
    Greater = 0x3E,
    Question = 0x3F,
    At = 0x40,
    LeftBracket = 0x5B,
    Backslash = 0x5C,
    RightBracket = 0x5D,
    Caret = 0x5E,
    Underscore = 0x5F,
    Backquote = 0x60,
    A = 0x61,
    B = 0x62,
    C = 0x63,
    D = 0x64,
    E = 0x65,
    F = 0x66,
    G = 0x67,
    H = 0x68,
    I = 0x69,
    J = 0x6A,
    K = 0x6B,
    L = 0x6C,
    M = 0x6D,
    N = 0x6E,
    O = 0x6F,
    P = 0x70,
    Q = 0x71,
    R = 0x72,
    S = 0x73,
    T = 0x74,
    U = 0x75,
    V = 0x76,
    W = 0x77,
    X = 0x78,
    Y = 0x79,
    Z = 0x7A,
    Delete = 0x7F,
    CapsLock = 0x40000039,
    F1 = 0x4000003A,
    F2 = 0x4000003B,
    F3 = 0x4000003C,
    F4 = 0x4000003D,
    F5 = 0x4000003E,
    F6 = 0x4000003F,
    F7 = 0x40000040,
    F8 = 0x40000041,
    F9 = 0x40000042,
    F10 = 0x40000043,
    F11 = 0x40000044,
    F12 = 0x40000045,
    PrintScreen = 0x40000046,
    ScrollLock = 0x40000047,
    Pause = 0x40000048,
    Insert = 0x40000049,
    Home = 0x4000004A,
    PageUp = 0x4000004B,
    End = 0x4000004D,
    PageDown = 0x4000004E,
    Right = 0x4000004F,
    Left = 0x40000050,
    Down = 0x40000051,
    Up = 0x40000052,
    NumLockClear = 0x40000053,
    NumPadDivide = 0x40000054,
    NumPadMultiply = 0x40000055,
    NumPadMinus = 0x40000056,
    NumPadPlus = 0x40000057,
    NumPadEnter = 0x40000058,
    NumPad1 = 0x40000059,
    NumPad2 = 0x4000005A,
    NumPad3 = 0x4000005B,
    NumPad4 = 0x4000005C,
    NumPad5 = 0x4000005D,
    NumPad6 = 0x4000005E,
    NumPad7 = 0x4000005F,
    NumPad8 = 0x40000060,
    NumPad9 = 0x40000061,
    NumPad0 = 0x40000062,
    NumPadPeriod = 0x40000063,
    Application = 0x40000065,
    Power = 0x40000066,
    NumPadEquals = 0x40000067,
    F13 = 0x40000068,
    F14 = 0x40000069,
    F15 = 0x4000006A,
    F16 = 0x4000006B,
    F17 = 0x4000006C,
    F18 = 0x4000006D,
    F19 = 0x4000006E,
    F20 = 0x4000006F,
    F21 = 0x40000070,
    F22 = 0x40000071,
    F23 = 0x40000072,
    F24 = 0x40000073,
    Execute = 0x40000074,
    Help = 0x40000075,
    Menu = 0x40000076,
    Select = 0x40000077,
    Stop = 0x40000078,
    Again = 0x40000079,
    Undo = 0x4000007A,
    Cut = 0x4000007B,
    Copy = 0x4000007C,
    Paste = 0x4000007D,
    Find = 0x4000007E,
    Mute = 0x4000007F,
    VolumeUp = 0x40000080,
    VolumeDown = 0x40000081,
    NumPadComma = 0x40000085,
    NumPadEqualsAS400 = 0x40000086,
    AltErase = 0x40000099,
    Sysreq = 0x4000009A,
    Cancel = 0x4000009B,
    Clear = 0x4000009C,
    Prior = 0x4000009D,
    Return2 = 0x4000009E,
    Separator = 0x4000009F,
    Out = 0x400000A0,
    Oper = 0x400000A1,
    ClearAgain = 0x400000A2,
    CrSel = 0x400000A3,
    ExSel = 0x400000A4,
    NumPad00 = 0x400000B0,
    NumPad000 = 0x400000B1,
    ThousandsSeparator = 0x400000B2,
    DecimalSeparator = 0x400000B3,
    CurrencyUnit = 0x400000B4,
    CurrencySubUnit = 0x400000B5,
    NumPadLeftParen = 0x400000B6,
    NumPadRightParen = 0x400000B7,
    NumPadLeftBrace = 0x400000B8,
    NumPadRightBrace = 0x400000B9,
    NumPadTab = 0x400000BA,
    NumPadBackspace = 0x400000BB,
    NumPadA = 0x400000BC,
    NumPadB = 0x400000BD,
    NumPadC = 0x400000BE,
    NumPadD = 0x400000BF,
    NumPadE = 0x400000C0,
    NumPadF = 0x400000C1,
    NumPadXor = 0x400000C2,
    NumPadPower = 0x400000C3,
    NumPadPercent = 0x400000C4,
    NumPadLess = 0x400000C5,
    NumPadGreater = 0x400000C6,
    NumPadAmpersand = 0x400000C7,
    NumPadDblAmpersand = 0x400000C8,
    NumPadVerticalBar = 0x400000C9,
    NumPadDblVerticalBar = 0x400000CA,
    NumPadColon = 0x400000CB,
    NumPadHash = 0x400000CC,
    NumPadSpace = 0x400000CD,
    NumPadAt = 0x400000CE,
    NumPadExclam = 0x400000CF,
    NumPadMemStore = 0x400000D0,
    NumPadMemRecall = 0x400000D1,
    NumPadMemClear = 0x400000D2,
    NumPadMemAdd = 0x400000D3,
    NumPadMemSubtract = 0x400000D4,
    NumPadMemMultiply = 0x400000D5,
    NumPadMemDivide = 0x400000D6,
    NumPadPlusMinus = 0x400000D7,
    NumPadClear = 0x400000D8,
    NumPadClearEntry = 0x400000D9,
    NumPadBinary = 0x400000DA,
    NumPadOctal = 0x400000DB,
    NumPadDecimal = 0x400000DC,
    NumPadHexadecimal = 0x400000DD,
    LCtrl = 0x400000E0,
    LShift = 0x400000E1,
    LAlt = 0x400000E2,
    LGui = 0x400000E3,
    RCtrl = 0x400000E4,
    RShift = 0x400000E5,
    RAlt = 0x400000E6,
    RGui = 0x400000E7,
    Mode = 0x40000101,
    AudioNext = 0x40000102,
    AudioPrev = 0x40000103,
    AudioStop = 0x40000104,
    AudioPlay = 0x40000105,
    AudioMute = 0x40000106,
    MediaSelect = 0x40000107,
    Www = 0x40000108,
    Mail = 0x40000109,
    Calculator = 0x4000010A,
    Computer = 0x4000010B,
    AcSearch = 0x4000010C,
    AcHome = 0x4000010D,
    AcBack = 0x4000010E,
    AcForward = 0x4000010F,
    AcStop = 0x40000110,
    AcRefresh = 0x40000111,
    AcBookmarks = 0x40000112,
    BrightnessDown = 0x40000113,
    BrightnessUp = 0x40000114,
    DisplaySwitch = 0x40000115,
    KbdIllumToggle = 0x40000116,
    KbdIllumDown = 0x40000117,
    KbdIllumUp = 0x40000118,
    Eject = 0x40000119,
    Sleep = 0x4000011A,
}

impl From<u32> for Key {
    fn from(val: u32) -> Key {
        match val {
            0x00 => Key::Unknown,
            0x08 => Key::Backspace,
            0x09 => Key::Tab,
            0x0D => Key::Return,
            0x1B => Key::Escape,
            0x20 => Key::Space,
            0x21 => Key::Exclaim,
            0x22 => Key::Quotedbl,
            0x23 => Key::Hash,
            0x24 => Key::Dollar,
            0x25 => Key::Percent,
            0x26 => Key::Ampersand,
            0x27 => Key::Quote,
            0x28 => Key::LeftParen,
            0x29 => Key::RightParen,
            0x2A => Key::Asterisk,
            0x2B => Key::Plus,
            0x2C => Key::Comma,
            0x2D => Key::Minus,
            0x2E => Key::Period,
            0x2F => Key::Slash,
            0x30 => Key::D0,
            0x31 => Key::D1,
            0x32 => Key::D2,
            0x33 => Key::D3,
            0x34 => Key::D4,
            0x35 => Key::D5,
            0x36 => Key::D6,
            0x37 => Key::D7,
            0x38 => Key::D8,
            0x39 => Key::D9,
            0x3A => Key::Colon,
            0x3B => Key::Semicolon,
            0x3C => Key::Less,
            0x3D => Key::Equals,
            0x3E => Key::Greater,
            0x3F => Key::Question,
            0x40 => Key::At,
            0x5B => Key::LeftBracket,
            0x5C => Key::Backslash,
            0x5D => Key::RightBracket,
            0x5E => Key::Caret,
            0x5F => Key::Underscore,
            0x60 => Key::Backquote,
            0x61 => Key::A,
            0x62 => Key::B,
            0x63 => Key::C,
            0x64 => Key::D,
            0x65 => Key::E,
            0x66 => Key::F,
            0x67 => Key::G,
            0x68 => Key::H,
            0x69 => Key::I,
            0x6A => Key::J,
            0x6B => Key::K,
            0x6C => Key::L,
            0x6D => Key::M,
            0x6E => Key::N,
            0x6F => Key::O,
            0x70 => Key::P,
            0x71 => Key::Q,
            0x72 => Key::R,
            0x73 => Key::S,
            0x74 => Key::T,
            0x75 => Key::U,
            0x76 => Key::V,
            0x77 => Key::W,
            0x78 => Key::X,
            0x79 => Key::Y,
            0x7A => Key::Z,
            0x7F => Key::Delete,
            0x40000039 => Key::CapsLock,
            0x4000003A => Key::F1,
            0x4000003B => Key::F2,
            0x4000003C => Key::F3,
            0x4000003D => Key::F4,
            0x4000003E => Key::F5,
            0x4000003F => Key::F6,
            0x40000040 => Key::F7,
            0x40000041 => Key::F8,
            0x40000042 => Key::F9,
            0x40000043 => Key::F10,
            0x40000044 => Key::F11,
            0x40000045 => Key::F12,
            0x40000046 => Key::PrintScreen,
            0x40000047 => Key::ScrollLock,
            0x40000048 => Key::Pause,
            0x40000049 => Key::Insert,
            0x4000004A => Key::Home,
            0x4000004B => Key::PageUp,
            0x4000004D => Key::End,
            0x4000004E => Key::PageDown,
            0x4000004F => Key::Right,
            0x40000050 => Key::Left,
            0x40000051 => Key::Down,
            0x40000052 => Key::Up,
            0x40000053 => Key::NumLockClear,
            0x40000054 => Key::NumPadDivide,
            0x40000055 => Key::NumPadMultiply,
            0x40000056 => Key::NumPadMinus,
            0x40000057 => Key::NumPadPlus,
            0x40000058 => Key::NumPadEnter,
            0x40000059 => Key::NumPad1,
            0x4000005A => Key::NumPad2,
            0x4000005B => Key::NumPad3,
            0x4000005C => Key::NumPad4,
            0x4000005D => Key::NumPad5,
            0x4000005E => Key::NumPad6,
            0x4000005F => Key::NumPad7,
            0x40000060 => Key::NumPad8,
            0x40000061 => Key::NumPad9,
            0x40000062 => Key::NumPad0,
            0x40000063 => Key::NumPadPeriod,
            0x40000065 => Key::Application,
            0x40000066 => Key::Power,
            0x40000067 => Key::NumPadEquals,
            0x40000068 => Key::F13,
            0x40000069 => Key::F14,
            0x4000006A => Key::F15,
            0x4000006B => Key::F16,
            0x4000006C => Key::F17,
            0x4000006D => Key::F18,
            0x4000006E => Key::F19,
            0x4000006F => Key::F20,
            0x40000070 => Key::F21,
            0x40000071 => Key::F22,
            0x40000072 => Key::F23,
            0x40000073 => Key::F24,
            0x40000074 => Key::Execute,
            0x40000075 => Key::Help,
            0x40000076 => Key::Menu,
            0x40000077 => Key::Select,
            0x40000078 => Key::Stop,
            0x40000079 => Key::Again,
            0x4000007A => Key::Undo,
            0x4000007B => Key::Cut,
            0x4000007C => Key::Copy,
            0x4000007D => Key::Paste,
            0x4000007E => Key::Find,
            0x4000007F => Key::Mute,
            0x40000080 => Key::VolumeUp,
            0x40000081 => Key::VolumeDown,
            0x40000085 => Key::NumPadComma,
            0x40000086 => Key::NumPadEqualsAS400,
            0x40000099 => Key::AltErase,
            0x4000009A => Key::Sysreq,
            0x4000009B => Key::Cancel,
            0x4000009C => Key::Clear,
            0x4000009D => Key::Prior,
            0x4000009E => Key::Return2,
            0x4000009F => Key::Separator,
            0x400000A0 => Key::Out,
            0x400000A1 => Key::Oper,
            0x400000A2 => Key::ClearAgain,
            0x400000A3 => Key::CrSel,
            0x400000A4 => Key::ExSel,
            0x400000B0 => Key::NumPad00,
            0x400000B1 => Key::NumPad000,
            0x400000B2 => Key::ThousandsSeparator,
            0x400000B3 => Key::DecimalSeparator,
            0x400000B4 => Key::CurrencyUnit,
            0x400000B5 => Key::CurrencySubUnit,
            0x400000B6 => Key::NumPadLeftParen,
            0x400000B7 => Key::NumPadRightParen,
            0x400000B8 => Key::NumPadLeftBrace,
            0x400000B9 => Key::NumPadRightBrace,
            0x400000BA => Key::NumPadTab,
            0x400000BB => Key::NumPadBackspace,
            0x400000BC => Key::NumPadA,
            0x400000BD => Key::NumPadB,
            0x400000BE => Key::NumPadC,
            0x400000BF => Key::NumPadD,
            0x400000C0 => Key::NumPadE,
            0x400000C1 => Key::NumPadF,
            0x400000C2 => Key::NumPadXor,
            0x400000C3 => Key::NumPadPower,
            0x400000C4 => Key::NumPadPercent,
            0x400000C5 => Key::NumPadLess,
            0x400000C6 => Key::NumPadGreater,
            0x400000C7 => Key::NumPadAmpersand,
            0x400000C8 => Key::NumPadDblAmpersand,
            0x400000C9 => Key::NumPadVerticalBar,
            0x400000CA => Key::NumPadDblVerticalBar,
            0x400000CB => Key::NumPadColon,
            0x400000CC => Key::NumPadHash,
            0x400000CD => Key::NumPadSpace,
            0x400000CE => Key::NumPadAt,
            0x400000CF => Key::NumPadExclam,
            0x400000D0 => Key::NumPadMemStore,
            0x400000D1 => Key::NumPadMemRecall,
            0x400000D2 => Key::NumPadMemClear,
            0x400000D3 => Key::NumPadMemAdd,
            0x400000D4 => Key::NumPadMemSubtract,
            0x400000D5 => Key::NumPadMemMultiply,
            0x400000D6 => Key::NumPadMemDivide,
            0x400000D7 => Key::NumPadPlusMinus,
            0x400000D8 => Key::NumPadClear,
            0x400000D9 => Key::NumPadClearEntry,
            0x400000DA => Key::NumPadBinary,
            0x400000DB => Key::NumPadOctal,
            0x400000DC => Key::NumPadDecimal,
            0x400000DD => Key::NumPadHexadecimal,
            0x400000E0 => Key::LCtrl,
            0x400000E1 => Key::LShift,
            0x400000E2 => Key::LAlt,
            0x400000E3 => Key::LGui,
            0x400000E4 => Key::RCtrl,
            0x400000E5 => Key::RShift,
            0x400000E6 => Key::RAlt,
            0x400000E7 => Key::RGui,
            0x40000101 => Key::Mode,
            0x40000102 => Key::AudioNext,
            0x40000103 => Key::AudioPrev,
            0x40000104 => Key::AudioStop,
            0x40000105 => Key::AudioPlay,
            0x40000106 => Key::AudioMute,
            0x40000107 => Key::MediaSelect,
            0x40000108 => Key::Www,
            0x40000109 => Key::Mail,
            0x4000010A => Key::Calculator,
            0x4000010B => Key::Computer,
            0x4000010C => Key::AcSearch,
            0x4000010D => Key::AcHome,
            0x4000010E => Key::AcBack,
            0x4000010F => Key::AcForward,
            0x40000110 => Key::AcStop,
            0x40000111 => Key::AcRefresh,
            0x40000112 => Key::AcBookmarks,
            0x40000113 => Key::BrightnessDown,
            0x40000114 => Key::BrightnessUp,
            0x40000115 => Key::DisplaySwitch,
            0x40000116 => Key::KbdIllumToggle,
            0x40000117 => Key::KbdIllumDown,
            0x40000118 => Key::KbdIllumUp,
            0x40000119 => Key::Eject,
            0x4000011A => Key::Sleep,

            _ => Key::Unknown,
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Key) -> bool {
        (*self as i32) == (*other as i32)
    }
}

impl Eq for Key {}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Key) -> Option<Ordering> {
        let (s_id, o_id) = (*self as i32, *other as i32);
        s_id.partial_cmp(&o_id)
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Key) -> Ordering {
        let (s_id, o_id) = (*self as i32, *other as i32);
        s_id.cmp(&o_id)
    }
}

impl Key {
    /// Returns an id of the key
    #[inline(always)]
    pub fn code(&self) -> i32 {
        *self as i32
    }
}

impl From<Key> for u32 {
    #[inline(always)]
    fn from(key: Key) -> u32 {
        key as u32
    }
}

/// Represent a mouse button.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
pub enum MouseButton {
    /// Unknown mouse button.
    Unknown,
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
    /// Extra mouse button number 1.
    X1,
    /// Extra mouse button number 2.
    X2,
    /// Mouse button number 6.
    Button6,
    /// Mouse button number 7.
    Button7,
    /// Mouse button number 8.
    Button8,
}

impl From<u32> for MouseButton {
    fn from(n: u32) -> MouseButton {
        match n {
            0 => MouseButton::Unknown,
            1 => MouseButton::Left,
            2 => MouseButton::Right,
            3 => MouseButton::Middle,
            4 => MouseButton::X1,
            5 => MouseButton::X2,
            6 => MouseButton::Button6,
            7 => MouseButton::Button7,
            8 => MouseButton::Button8,
            _ => MouseButton::Unknown,
        }
    }
}

impl From<MouseButton> for u32 {
    fn from(button: MouseButton) -> u32 {
        match button {
            MouseButton::Unknown => 0,
            MouseButton::Left => 1,
            MouseButton::Right => 2,
            MouseButton::Middle => 3,
            MouseButton::X1 => 4,
            MouseButton::X2 => 5,
            MouseButton::Button6 => 6,
            MouseButton::Button7 => 7,
            MouseButton::Button8 => 8,
        }
    }
}

/// Components of a controller button event. Not guaranteed consistent across
/// backends.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ControllerButton {
    /// Which controller was the button on.
    pub id: u32,
    /// Which button was pressed.
    pub button: u8,
}

impl ControllerButton {
    /// Create a new ControllerButton object. Intended for use by backends when
    /// emitting events.
    pub fn new(id: u32, button: u8) -> Self {
        ControllerButton {
            id,
            button,
        }
    }
}

/// Components of a controller hat move event (d-Pad).
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct ControllerHat {
    /// Which Controller was the button on.
    pub id: u32,
    /// Which button was pressed.
    pub state: HatState,
    /// Which hat on the controller was changed
    pub which: u8,
}

impl ControllerHat {
    /// Create a new ControllerButton object. Intended for use by backends when
    /// emitting events.
    pub fn new(id: u32, which: u8, state: HatState) -> Self {
        ControllerHat {
            id, state, which,
        }
    }
}

/// Stores controller hat state.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum HatState {
    /// Centered (no direction).
    Centered,
    /// Up direction.
    Up,
    /// Right direction.
    Right,
    /// Down direction.
    Down,
    /// Left direction.
    Left,
    /// Right-up direction.
    RightUp,
    /// Right-down direction.
    RightDown,
    /// Left-up direction.
    LeftUp,
    /// Left-down direction.
    LeftDown,
}

/// Models different kinds of motion.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Debug)]
pub enum Motion {
    /// Position in window coordinates.
    MouseCursor([f64; 2]),
    /// Position in relative coordinates.
    MouseRelative([f64; 2]),
    /// Position in scroll ticks.
    MouseScroll([f64; 2]),
    /// Controller axis move event.
    ControllerAxis(ControllerAxisArgs),
    /// Touch event.
    Touch(TouchArgs),
}

/// Components of a controller axis move event. Not guaranteed consistent across
/// backends.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Debug)]
pub struct ControllerAxisArgs {
    /// Which controller moved.
    pub id: u32,
    /// The axis that moved.
    pub axis: u8,
    /// Position of the controller. Usually [-1.0, 1.0], though backends may use
    /// a different range for various devices.
    pub position: f64,
}

impl ControllerAxisArgs {
    /// Create a new ControllerAxisArgs object. Intended for use by backends when
    /// emitting events.
    pub fn new(id: u32, axis: u8, position: f64) -> Self {
        ControllerAxisArgs {
            id,
            axis,
            position,
        }
    }
}

/// Touch arguments
///
/// The `id` might be reused for different touches that do not overlap in time.
///
/// - Coordinates are normalized to support both touch screens and trackpads
/// - Supports both 2D and 3D touch
/// - The pressure direction vector should have maximum length 1
///
/// For 2D touch the pressure is pointed in the z direction.
/// Use `.pressure()` to get the pressure magnitude.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Debug)]
pub struct TouchArgs {
    /// A unique identifier for touch device.
    pub device: i64,
    /// A unique identifier for touch event.
    pub id: i64,
    /// The touch position, normalized 0..1.
    pub position_3d: [f64; 3],
    /// The touch pressure vector, normalized 0..1.
    pub pressure_3d: [f64; 3],
    /// Whether the touch is in 3D.
    pub is_3d: bool,
    /// The touch state.
    pub touch: Touch,
}

impl TouchArgs {
    /// Creates arguments for 2D touch.
    pub fn new(device: i64, id: i64, position: [f64; 2], pressure: f64, touch: Touch) -> TouchArgs {
        TouchArgs {
            device,
            id,
            position_3d: [position[0], position[1], 0.0],
            pressure_3d: [0.0, 0.0, pressure],
            is_3d: false,
            touch,
        }
    }

    /// Creates arguments for 3D touch.
    ///
    /// The pressure direction vector should have maximum length 1.
    pub fn new_3d(device: i64,
                  id: i64,
                  position_3d: [f64; 3],
                  pressure_3d: [f64; 3],
                  touch: Touch)
                  -> TouchArgs {
        TouchArgs {
            device,
            id,
            position_3d,
            pressure_3d,
            is_3d: true,
            touch,
        }
    }

    /// The position of the touch in 2D.
    pub fn position(&self) -> [f64; 2] {
        [self.position_3d[0], self.position_3d[1]]
    }

    /// The position of the touch in 3D.
    pub fn position_3d(&self) -> [f64; 3] {self.position_3d}

    /// The pressure magnitude, normalized 0..1.
    pub fn pressure(&self) -> f64 {
        let px = self.pressure_3d[0];
        let py = self.pressure_3d[1];
        let pz = self.pressure_3d[2];
        (px * px + py * py + pz * pz).sqrt()
    }

    /// The pressure vector in 3D.
    pub fn pressure_3d(&self) -> [f64; 3] {self.pressure_3d}
}

/// Stores the touch state.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Touch {
    /// The start of touch, for example
    /// a finger pressed down on a touch screen.
    Start,
    /// The move of touch, for example
    /// a finger moving while touching a touch screen.
    Move,
    /// The end of touch, for example
    /// taking a finger away from a touch screen.
    End,
    /// The cancel of touch, for example
    /// the window loses focus.
    Cancel,
}

/// Resize arguments.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct ResizeArgs {
    /// The width and height of rendered area in points.
    pub window_size: [f64; 2],
    /// The width of rendered area in pixels.
    pub draw_size: [u32; 2],
}

impl ResizeArgs {
    /// Returns viewport information filling entire render area.
    pub fn viewport(&self) -> Viewport {
        Viewport {
            rect: [0, 0, self.draw_size[0] as i32, self.draw_size[1] as i32],
            window_size: self.window_size,
            draw_size: self.draw_size,
        }
    }
}

/// Stores viewport information.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Viewport {
    /// Viewport in pixels.
    /// ```[x, y, width height]``` where ```(x, y)``` is lower left corner.
    pub rect: [i32; 4],
    /// The size of frame buffer in pixels.
    pub draw_size: [u32; 2],
    /// The size of window in points.
    pub window_size: [f64; 2],
}

/*
impl Viewport {
    /// Computes absolute transform for 2D graphics,
    /// which uses a row major 2x3 matrix.
    ////
    /// The origin is in the upper left corner of the viewport rectangle.
    /// The x axis points to the right, and the y axis points down.
    /// The units are in points (window coordinates).
    ///
    /// It is assumed that the underlying coordinate system is normalized
    /// with the origin in the center, such that ```(-1.0, 1.0)``` in the
    /// underlying coordinate system corresponds to the
    /// upper left corner of the viewport.
    pub fn abs_transform<T: Float>(&self) -> [[T; 3]; 2] {
        let (dw, dh) = (self.draw_size[0] as f64, self.draw_size[1] as f64);
        let sx = 2.0 * (dw / self.window_size[0]) / self.rect[2] as f64;
        let sy = -2.0 * (dh / self.window_size[1]) / self.rect[3] as f64;
        let f = |x| FromPrimitive::from_f64(x);
        [
            [f(sx), f(0.0), f(-1.0)],
            [f(0.0), f(sy), f(1.0)]
        ]
    }
}
 */

/// Models dragging and dropping files.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Hash)]
pub enum FileDrag {
    /// A file is being hovered over the window.
    Hover(PathBuf),
    /// A file has been dropped into the window.
    Drop(PathBuf),
    /// A file was hovered, but has exited the window.
    Cancel,
}

/// Close arguments.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize, Hash)]
pub struct CloseArgs;
