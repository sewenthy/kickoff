use image::Rgba;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor, self};
use hex::encode;
use std::fmt;
use std::str::FromStr;

pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl From<css_color::Rgba> for Color {
    fn from(c: css_color::Rgba) -> Self {
        Color(
            (c.red * 255.) as u8,
            (c.green * 255.) as u8,
            (c.blue * 255.) as u8,
            (c.alpha * 255.) as u8,
        )
    }
}

impl Color {
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba([self.0, self.1, self.2, self.3])
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        let mut c = "#".to_string();
        c.push_str(&encode([self.0, self.1, self.2, self.3]));
        serializer.serialize_str(&c)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hex rgb or rgba color value")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where 
        E: de::Error
    {
        let c = css_color::Rgba::from_str(value);
        if let Ok(c) = c {
            Ok(Color::from(c))
        } else {
            Err(de::Error::custom(""))
        }
    }
}