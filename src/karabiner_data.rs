use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Debug, Serialize, Clone)]
pub enum BundleIdentifier {
    #[serde(rename = "com.googlecode.iterm2")]
    ITerm2,
    #[serde(rename = "com.microsoft.VSCode")]
    VSCode,
    #[serde(rename = "io.dynalist")]
    Dynalist, // https://help.dynalist.io/article/91-keyboard-shortcut-reference
    #[serde(rename = "com.tinyspeck.slackmacgap")]
    Slack,
    #[serde(rename = "com.google.Chrome")]
    GoogleChrome,
    #[serde(rename = "notion.id")]
    Notion,
    #[serde(rename = "com.openai.chat")]
    ChatGPT,
}

#[derive(Debug, Serialize)]
pub struct Rule {
    pub description: String,
    pub manipulators: Vec<Manipulator>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Condition {
    OnApplication {
        r#type: ConditionType,
        bundle_identifiers: Vec<BundleIdentifier>,
    },
    WithVirtualKey {
        r#type: ConditionType,
        name: VirtualKey,
        value: VirtualKeyValue,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    FrontmostApplicationIf,
    VariableIf,
}

impl Condition {
    pub fn on_app(bundle_identifier: BundleIdentifier) -> Condition {
        Condition::OnApplication {
            r#type: ConditionType::FrontmostApplicationIf,
            bundle_identifiers: vec![bundle_identifier],
        }
    }

    pub fn with_vk1() -> Condition {
        Self::with_virtual_key(VirtualKey::Vk1)
    }

    pub fn with_vk2() -> Condition {
        Self::with_virtual_key(VirtualKey::Vk2)
    }

    pub fn with_vk3() -> Condition {
        Self::with_virtual_key(VirtualKey::Vk3)
    }

    pub fn with_vk4() -> Condition {
        Self::with_virtual_key(VirtualKey::Vk4)
    }

    pub fn with_virtual_key(virtual_key: VirtualKey) -> Condition {
        Condition::WithVirtualKey {
            r#type: ConditionType::VariableIf,
            name: virtual_key,
            value: VirtualKeyValue::On,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct From {
    pub key_code: KeyCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<FromModifier>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FromModifier {
    Optional(Vec<ModifierKey>),
    Mandatory(Vec<ModifierKey>),
}

#[derive(Debug, Serialize)]
pub struct SetVariable {
    pub name: VirtualKey,
    pub value: VirtualKeyValue,
}

#[derive(Debug, Serialize)]
pub struct MouseKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_wheel: Option<i32>,
}

#[derive(Debug, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum To {
    Variable {
        set_variable: SetVariable,
    },
    Key {
        key_code: KeyCode,
        #[serde(skip_serializing_if = "Option::is_none")]
        modifiers: Option<Vec<ModifierKey>>,
    },
    Mouse {
        mouse_key: MouseKey,
    },
    Click {
        pointing_button: PointingButton,
    },
    Command {
        shell_command: &'static str,
    },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PointingButton {
    Button1,
    Button2,
}

#[derive(Debug, Serialize)]
pub struct Manipulator {
    pub r#type: ManipulatorType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,

    pub from: From,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub to: Vec<To>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_after_key_up: Option<Vec<ToAfterKeyUp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_if_alone: Option<Vec<ToIfAlone>>,
}

impl Manipulator {
    pub fn builder() -> ManipulatorInitBuilder {
        ManipulatorInitBuilder::default()
    }
}

#[derive(Default)]
pub struct ManipulatorInitBuilder {
    conditions: Option<Vec<Condition>>,
    from: Option<From>,
    to: Vec<To>,
    to_after_key_up: Option<Vec<ToAfterKeyUp>>,
    to_if_alone: Option<Vec<ToIfAlone>>,
}

impl ManipulatorInitBuilder {
    pub fn condition(mut self, condition: Condition) -> Self {
        self.conditions.get_or_insert(vec![]).push(condition);
        self
    }

    pub fn conditions(mut self, conditions: Vec<Condition>) -> Self {
        self.conditions = Some(conditions);
        self
    }

    pub fn from_key(mut self, key_code: KeyCode) -> Self {
        self.from = Some(From {
            key_code,
            modifiers: None,
        });
        self
    }

    pub fn from_key_with_modifiers(mut self, key_code: KeyCode, modifiers: FromModifier) -> Self {
        self.from = Some(From {
            key_code,
            modifiers: Some(modifiers),
        });
        self
    }

    pub fn to_key(mut self, key_code: KeyCode, modifiers: Option<Vec<ModifierKey>>) -> Self {
        self.to.push(To::Key {
            key_code,
            modifiers,
        });
        self
    }

    pub fn to_command(mut self, command: &'static str) -> Self {
        self.to.push(To::Command {
            shell_command: command,
        });
        self
    }

    pub fn to_mouse(mut self, mouse_key: MouseKey) -> Self {
        self.to.push(To::Mouse { mouse_key });
        self
    }

    pub fn to_click(mut self, pointing_button: PointingButton) -> Self {
        self.to.push(To::Click { pointing_button });
        self
    }

    pub fn to_variable(mut self, set_variable: SetVariable) -> Self {
        self.to.push(To::Variable { set_variable });
        self
    }

    pub fn to_after_key_up(mut self, set_variable: SetVariable) -> Self {
        self.to_after_key_up
            .get_or_insert(vec![])
            .push(ToAfterKeyUp { set_variable });
        self
    }

    pub fn to_if_alone(mut self, key_code: KeyCode) -> Self {
        self.to_if_alone
            .get_or_insert(vec![])
            .push(ToIfAlone { key_code });
        self
    }

    pub fn build(self) -> Manipulator {
        Manipulator {
            r#type: ManipulatorType::Basic,
            conditions: self.conditions,
            from: self.from.unwrap_or(From {
                key_code: KeyCode::Escape,
                modifiers: None,
            }),
            to: self.to,
            to_after_key_up: self.to_after_key_up,
            to_if_alone: self.to_if_alone,
        }
    }
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ManipulatorType {
    #[default]
    Basic,
}

#[derive(Debug, Serialize)]
pub struct ToDelayedAction {
    pub to_if_invoked: Vec<To>,
}

#[derive(Debug, Serialize)]
pub struct ToIfAlone {
    pub key_code: KeyCode,
}

#[derive(Debug, Serialize)]
pub struct ToAfterKeyUp {
    pub set_variable: SetVariable,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VirtualKey {
    Vk1,
    Vk2,
    Vk3,
    Vk4,
}

#[derive(Serialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum VirtualKeyValue {
    On = 1,
    Off = 0,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModifierKey {
    Any,
    #[serde(rename = "control")]
    Ctrl,
    Shift,
    #[serde(rename = "option")]
    Opt,
    #[serde(rename = "command")]
    Cmd,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
// See https://github.com/pqrs-org/Karabiner-Elements/blob/a9154a6b073a3396631f43ed11f6dc603c28ea7b/src/share/types/key_code.hpp#L146-L360
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    #[serde(rename = "1")]
    Key1,
    #[serde(rename = "2")]
    Key2,
    #[serde(rename = "3")]
    Key3,
    #[serde(rename = "4")]
    Key4,
    #[serde(rename = "5")]
    Key5,
    #[serde(rename = "6")]
    Key6,
    #[serde(rename = "7")]
    Key7,
    #[serde(rename = "8")]
    Key8,
    #[serde(rename = "9")]
    Key9,
    #[serde(rename = "0")]
    Key0,
    ReturnOrEnter,
    Escape,
    DeleteOrBackspace,
    Tab,
    Spacebar,
    Hyphen,
    EqualSign,
    OpenBracket,
    CloseBracket,
    Backslash,
    NonUsPound,
    Semicolon,
    Quote,
    GraveAccentAndTilde,
    Comma,
    Period,
    Slash,
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
    F11,
    F12,
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    DeleteForward,
    End,
    PageDown,
    RightArrow,
    LeftArrow,
    DownArrow,
    UpArrow,
    KeypadNumLock,
    KeypadSlash,
    KeypadAsterisk,
    KeypadHyphen,
    KeypadPlus,
    KeypadEnter,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    Keypad0,
    KeypadPeriod,
    NonUsBackslash,
    Application,
    Power,
    KeypadEqualSign,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    Mute,
    VolumeDecrement,
    VolumeIncrement,
    LockingCapsLock,
    LockingNumLock,
    LockingScrollLock,
    KeypadComma,
    KeypadEqualSignAs400,
    International1,
    International2,
    International3,
    International4,
    International5,
    International6,
    International7,
    International8,
    International9,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    Lang6,
    Lang7,
    Lang8,
    Lang9,
    AlternateErase,
    SysReqOrAttention,
    Cancel,
    Clear,
    Prior,
    Return,
    Separator,
    Out,
    Oper,
    ClearOrAgain,
    CrSelOrProps,
    ExSel,
    LeftControl,
    LeftShift,
    LeftAlt,
    LeftGui,
    RightControl,
    RightShift,
    RightAlt,
    RightGui,
    VkNone,
    Fn,
    DisplayBrightnessDecrement,
    DisplayBrightnessIncrement,
    MissionControl,
    Launchpad,
    Dashboard,
    IlluminationDecrement,
    IlluminationIncrement,
    Rewind,
    PlayOrPause,
    Fastforward,
    Eject,
    AppleDisplayBrightnessDecrement,
    AppleDisplayBrightnessIncrement,
    AppleTopCaseDisplayBrightnessDecrement,
    AppleTopCaseDisplayBrightnessIncrement,
    LeftOption,
    LeftCommand,
    RightOption,
    RightCommand,
    JapaneseEisuu,
    JapaneseKana,
    JapanesePcNfer,
    JapanesePcXfer,
    JapanesePcKatakana,
    VkConsumerBrightnessDown,
    VkConsumerBrightnessUp,
    VkMissionControl,
    VkLaunchpad,
    VkDashboard,
    VkConsumerIlluminationDown,
    VkConsumerIlluminationUp,
    VkConsumerPrevious,
    VkConsumerPlay,
    VkConsumerNext,
    VolumeDown,
    VolumeUp,
}
