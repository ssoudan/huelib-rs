#![allow(clippy::needless_update)]

use crate::resource::{self, Adjuster, Alert, ColorMode, Effect};
use crate::Color;
use derive_setters::Setters;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

/// A light.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Light {
    /// Identifier of the light.
    #[serde(skip)]
    pub id: String,
    /// Name of the light.
    pub name: String,
    /// Type of the light.
    #[serde(rename = "type")]
    pub kind: String,
    /// Current state of the light.
    pub state: State,
    /// The hardware model of the light.
    #[serde(rename = "modelid")]
    pub model_id: String,
    /// Unique ID of the light.
    #[serde(rename = "uniqueid")]
    pub unique_id: String,
    /// Product ID of the light.
    #[serde(rename = "productid")]
    pub product_id: Option<String>,
    /// Product name of the light.
    #[serde(rename = "productname")]
    pub product_name: Option<String>,
    /// Manufacturer name of the light.
    #[serde(rename = "manufacturername")]
    pub manufacturer_name: Option<String>,
    /// The software version running on the light.
    #[serde(rename = "swversion")]
    pub software_version: String,
    /// Information about software updates of the light.
    #[serde(rename = "swupdate")]
    pub software_update: SoftwareUpdate,
    /// Configuration of the light.
    pub config: Config,
    /// Capabilities of the light.
    pub capabilities: Capabilities,
}

impl resource::Resource for Light {}

impl Light {
    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
}

/// State of a light.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct State {
    /// Whether the light is on.
    pub on: Option<bool>,
    /// Brightness of the light.
    ///
    /// The maximum brightness is 254 and 1 is the minimum brightness.
    #[serde(rename = "bri")]
    pub brightness: Option<u8>,
    /// Hue of the light.
    ///
    /// Both 0 and 65535 are red, 25500 is green and 46920 is blue.
    pub hue: Option<u16>,
    /// Saturation of the light.
    ///
    /// The most saturated (colored) is 254 and 0 is the least saturated (white).
    #[serde(rename = "sat")]
    pub saturation: Option<u8>,
    /// X and y coordinates of a color in CIE color space. Both values must be between 0 and 1.
    #[serde(rename = "xy")]
    pub color_space_coordinates: Option<(f32, f32)>,
    /// Mired color temperature of the light.
    #[serde(rename = "ct")]
    pub color_temperature: Option<u16>,
    /// Alert effect of the light.
    pub alert: Option<Alert>,
    /// Dynamic effect of the light.
    pub effect: Option<Effect>,
    /// Color mode of the light.
    #[serde(rename = "colormode")]
    pub color_mode: Option<ColorMode>,
    /// Whether the light can be reached by the bridge.
    pub reachable: bool,
}

/// Information about software updates of a light.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct SoftwareUpdate {
    /// State of software updates.
    pub state: SoftwareUpdateState,
    /// When the last update was installed.
    #[serde(rename = "lastinstall")]
    pub last_install: Option<chrono::NaiveDateTime>,
}

/// State of a software update.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SoftwareUpdateState {
    /// No updates are available.
    NoUpdates,
    /// Device cannot be updated.
    NotUpdatable,
    /// Device is downloading new updates.
    Transferring,
    /// Device is ready to install new updates.
    ReadyToInstall,
    // TODO: Add missing variants for states (https://github.com/yuqio/huelib-rs/issues/1)
}

/// Configuration of a light.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Config {
    /// Arche type of the light.
    #[serde(rename = "archetype")]
    pub arche_type: String,
    /// Function of the light.
    pub function: String,
    /// Direction of the light.
    pub direction: String,
    /// Startup configuration of the light.
    pub startup: Option<StartupConfig>,
}

/// Startup configuration of a light.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct StartupConfig {
    /// Mode of the startup.
    pub mode: String,
    /// Whether startup is configured for the light.
    pub configured: bool,
}

/// Capabilities of a light.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Capabilities {
    /// Whether the light is certified.
    pub certified: bool,
    /// Control capabilities of the light.
    pub control: ControlCapabilities,
    /// Streaming capabilities of the light.
    pub streaming: StreamingCapabilities,
}

/// Control capabilities of a light.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ControlCapabilities {
    /// Minimal dimlevel of the light.
    #[serde(rename = "mindimlevel")]
    pub min_dimlevel: Option<usize>,
    /// Maximal lumen of the light.
    #[serde(rename = "maxlumen")]
    pub max_lumen: Option<usize>,
    /// Color gamut of the light.
    #[serde(rename = "colorgamut")]
    pub color_gamut: Option<Vec<(f32, f32)>>,
    /// Type of the color gamut of the light.
    #[serde(rename = "colorgamuttype")]
    pub color_gamut_type: Option<String>,
    /// Maximal/minimal color temperature of the light.
    #[serde(rename = "ct")]
    pub color_temperature: Option<ColorTemperatureCapabilities>,
}

/// Color temperature capabilities of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct ColorTemperatureCapabilities {
    /// Minimal color temperature.
    pub min: usize,
    /// Maximal color temperature.
    pub max: usize,
}

/// Streaming capabilities of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct StreamingCapabilities {
    /// Whether a renderer is enabled.
    pub renderer: bool,
    /// Whether a proxy is enabled.
    pub proxy: bool,
}

/// Modifier for light attributes.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct AttributeModifier {
    /// Sets the name of the light.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl resource::Modifier for AttributeModifier {}

impl AttributeModifier {
    /// Creates a new [`AttributeModifier`].
    pub fn new() -> Self {
        Self::default()
    }
}

/// Static modifier for the light state.
///
/// In comparison to [`StateModifier`], this modifier cannot increment/decrement any attributes or
/// change the alert effect.
///
/// This modifier is used in [`scene::Modifier`] and [`scene::Creator`].
///
/// [`scene::Modifier`]: super::scene::Modifier
/// [`scene::Creator`]: super::scene::Creator
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct StaticStateModifier {
    /// Turns the light on or off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    /// Sets the brightness of the light.
    #[serde(skip_serializing_if = "Option::is_none", rename = "bri")]
    pub brightness: Option<u8>,
    /// Sets the hue of the light.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<u16>,
    /// Sets the saturation of a light.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sat")]
    pub saturation: Option<u8>,
    /// Sets the color space coordinates of the light.
    #[serde(skip_serializing_if = "Option::is_none", rename = "xy")]
    pub color_space_coordinates: Option<(f32, f32)>,
    /// Sets the color temperature of a light.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ct")]
    pub color_temperature: Option<u16>,
    /// Sets the dynamic effect of a light.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<Effect>,
    /// Sets the transition duration of state changes.
    ///
    /// This is given as a multiple of 100ms.
    #[serde(skip_serializing_if = "Option::is_none", rename = "transitiontime")]
    pub transition_time: Option<u16>,
}

impl resource::Modifier for StaticStateModifier {}

impl StaticStateModifier {
    /// Creates a new [`StaticStateModifier`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Convenient method to set the [`color_space_coordinates`] and [`brightness`] fields.
    ///
    /// [`color_space_coordinates`]: Self::color_space_coordinates
    /// [`brightness`]: Self::brightness
    pub fn with_color(self, value: Color) -> Self {
        let mut modifier = Self {
            color_space_coordinates: Some(value.space_coordinates),
            ..self
        };
        if let Some(brightness) = value.brightness {
            modifier.brightness = Some(brightness);
        }
        modifier
    }
}

/// Modifier for the light state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct StateModifier {
    /// Turns the light on or off.
    pub on: Option<bool>,
    /// Sets the brightness of the light.
    pub brightness: Option<Adjuster<u8>>,
    /// Sets the hue of a light.
    pub hue: Option<Adjuster<u16>>,
    /// Sets the saturation of a light.
    pub saturation: Option<Adjuster<u8>>,
    /// Sets the color space coordinates of the light.
    pub color_space_coordinates: Option<Adjuster<(f32, f32)>>,
    /// Sets the color temperature of a light.
    pub color_temperature: Option<Adjuster<u16>>,
    /// Sets the alert effect of a light.
    pub alert: Option<Alert>,
    /// Sets the dynamic effect of a light.
    pub effect: Option<Effect>,
    /// Sets the transition duration of state changes.
    ///
    /// This is given as a multiple of 100ms.
    pub transition_time: Option<u16>,
}

impl resource::Modifier for StateModifier {}

impl StateModifier {
    /// Creates a new [`StateModifier`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Convenient method to set the [`color_space_coordinates`] and [`brightness`] fields.
    ///
    /// [`color_space_coordinates`]: Self::color_space_coordinates
    /// [`brightness`]: Self::brightness
    pub fn with_color(self, value: Color) -> Self {
        let mut modifier = Self {
            color_space_coordinates: Some(Adjuster::Override(value.space_coordinates)),
            ..self
        };
        if let Some(brightness) = value.brightness {
            modifier.brightness = Some(Adjuster::Override(brightness));
        }
        modifier
    }
}

impl Serialize for StateModifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        custom_serialize! {
            serializer, "StateModifier";
            on => (&self.on),
            bri => (&self.brightness, to_override),
            bri_inc => (&self.brightness, to_increment, i16),
            hue => (&self.hue, to_override),
            hue_inc => (&self.hue, to_increment, i32),
            sat => (&self.saturation, to_override),
            sat_inc => (&self.saturation, to_increment, i16),
            xy => (&self.color_space_coordinates, to_override),
            xy_inc => (&self.color_space_coordinates, to_increment_tuple, f32),
            ct => (&self.color_temperature, to_override),
            ct_inc => (&self.color_temperature, to_increment, i32),
            alert => (&self.alert),
            effect => (&self.effect),
            transitiontime => (&self.transition_time),
        }
    }
}
