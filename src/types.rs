use serde::{Deserialize, Serialize};

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Off = 0,
    FixedOn = 1,
    Respire = 2,
    Rainbow = 3,
    FlashAway = 4,
    Raindrops = 5,
    RipplesShining = 6,
    StarsTwinkle = 7,
    RetroSnake = 8,
    NeonStream = 9,
    Reaction = 10,
    SineWave = 11,
    RotatingWindmill = 12,
    ColorfulWaterfall = 13,
    Blossoming = 14,
    SelfDefine = 15,
}

impl TryFrom<u16> for Effect {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Effect::Off,
            1 => Effect::FixedOn,
            2 => Effect::Respire,
            3 => Effect::Rainbow,
            4 => Effect::FlashAway,
            5 => Effect::Raindrops,
            6 => Effect::RipplesShining,
            7 => Effect::StarsTwinkle,
            8 => Effect::RetroSnake,
            9 => Effect::NeonStream,
            10 => Effect::Reaction,
            11 => Effect::SineWave,
            12 => Effect::RotatingWindmill,
            13 => Effect::ColorfulWaterfall,
            14 => Effect::Blossoming,
            15 => Effect::SelfDefine,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default, Serialize)]
#[repr(u8)]
pub enum KeyLayer {
    #[default]
    Normal = 0,
    Fn = 1,
    Fn1 = 2,
}

impl TryFrom<u8> for KeyLayer {
    type Error = String;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(KeyLayer::Normal),
            1 => Ok(KeyLayer::Fn),
            2 => Ok(KeyLayer::Fn1),
            _ => Err(format!("Invalid KeyLayer: {}", v)),
        }
    }
}

impl<'de> Deserialize<'de> for KeyLayer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = u8::deserialize(deserializer)?;
        match v {
            0 => Ok(KeyLayer::Normal),
            1 => Ok(KeyLayer::Fn),
            2 => Ok(KeyLayer::Fn1),
            _ => Err(serde::de::Error::custom("invalid KeyLayer value")),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum KeyType {
    #[default]
    Basic,
    Macro,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn is_on(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }

    pub fn create(r: u8, g: u8, b: u8) -> Self {
        let a = if r == 0 && g == 0 && b == 0 { 0 } else { 1 };
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeviceInfo {
    pub mac_mode: bool,
    pub polling_level: u8,
    pub low_latency: bool,
    pub win_lock: bool,
    pub light_mode: Effect,
    pub sleep_level: u8,
    pub light_params: Vec<LightParam>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BatteryStatus {
    pub charging: bool,
    pub level: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct Key {
    pub name: String,
    pub value: String,
    pub pos: usize,
    pub light_pos: usize,
    pub effect_pos: usize,
    pub layer: KeyLayer,
    pub key_type: KeyType,
    pub color: Color,
    #[serde(default)]
    pub macro_data: Option<MacroData>,
}

impl Key {
    pub fn new_basic(name: String, value: String, pos: usize, layer: KeyLayer) -> Self {
        Self {
            name,
            value,
            pos,
            layer,
            key_type: KeyType::Basic,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MacroType {
    Button,
    Circle,
    Repeat,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LightParam {
    pub bright: u8,
    pub speed: u8,
}

impl LightParam {
    pub fn new(bright: u8, speed: u8) -> Self {
        Self { bright, speed }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MacroData {
    pub macro_type: MacroType,
    pub count: u8,
    pub data: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub value: u32,
}
