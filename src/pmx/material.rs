use crate::Error;
use enumflags2::BitFlags;
use itertools::Itertools;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};

#[derive(BitFlags, Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum DrawingFlags {
  NoCull = 0b00000001,
  GroundShadow = 0b00000010,
  DrawShadow = 0b00000100,
  ReceiveShadow = 0b00001000,
  HasEdge = 0b00010000,
  VertexColor = 0b00100000,
  PointDrawing = 0b01000000,
  LineDrawing = 0b10000000,
}

struct DrawingFlagsFmt(BitFlags<DrawingFlags>);

impl Display for DrawingFlagsFmt {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "{}", self.0.iter().map(|v| format!("{:?}", v)).join("|"))
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnvironmentBlendMode {
  Disabled = 0,
  Multiply = 1,
  Additive = 2,
  AdditionalVec4 = 3,
}

impl Display for EnvironmentBlendMode {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      EnvironmentBlendMode::Disabled => write!(f, "disabled"),
      EnvironmentBlendMode::Multiply => write!(f, "*"),
      EnvironmentBlendMode::Additive => write!(f, "+"),
      EnvironmentBlendMode::AdditionalVec4 => write!(f, "in vertex"),
    }
  }
}

impl TryFrom<u8> for EnvironmentBlendMode {
  type Error = Error;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    Ok(match value {
      0 => EnvironmentBlendMode::Disabled,
      1 => EnvironmentBlendMode::Multiply,
      2 => EnvironmentBlendMode::Additive,
      3 => EnvironmentBlendMode::AdditionalVec4,
      e => return Err(Error::InvalidEnvironmentBlendMode(e)),
    })
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Toon<TextureIndex = i32> {
  Texture(TextureIndex),
  Internal(u8),
}

impl<TextureIndex: Display> Display for Toon<TextureIndex> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    match self {
      Toon::Texture(t) => write!(f, "texture({})", t),
      Toon::Internal(i) => write!(f, "internal({})", i),
    }
  }
}

pub struct Material<TextureIndex = i32> {
  pub local_name: String,
  pub universal_name: String,
  pub diffuse_color: [f32; 4],
  pub specular_color: [f32; 3],
  pub specular_strength: f32,
  pub ambient_color: [f32; 3],
  pub draw_flags: BitFlags<DrawingFlags>,
  pub edge_color: [f32; 4],
  pub edge_scale: f32,
  pub texture_index: TextureIndex,
  pub environment_index: TextureIndex,
  pub environment_blend_mode: EnvironmentBlendMode,
  pub toon: Toon<TextureIndex>,
  pub metadata: String,
  pub surface_count: i32,
}

impl<TextureIndex: Display> Display for Material<TextureIndex> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(
      f,
      r"local name: {}, universal name: {}
diffuse: {:?}, specular: {:?}/{}, ambient: {:?}, flags: {}
edge: {:?}/{}, texture: {}, environment: {}/{},
toon: {}, metadata: {}, surfaces: {}",
      self.local_name,
      self.universal_name,
      self.diffuse_color,
      self.specular_color,
      self.specular_strength,
      self.ambient_color,
      DrawingFlagsFmt(self.draw_flags),
      self.edge_color,
      self.edge_scale,
      self.texture_index,
      self.environment_index,
      self.environment_blend_mode,
      self.toon,
      self.metadata,
      self.surface_count
    )
  }
}
