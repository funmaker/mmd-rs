use thiserror::Error;
use std::borrow::Cow;

#[derive(Debug, Error)]
pub enum Error {
  #[error("{}", _0)]
  Io(#[from] std::io::Error),
  #[error("Wrong signature {:?}", _0)]
  WrongSignature([u8; 4]),
  #[error("Globals count less than 8 {}", _0)]
  GlobalsCountLessThan8(u8),
  #[error("Unknown index size {}", _0)]
  UnknownIndexSize(u8),
  #[error("Unknown text encoding {}", _0)]
  UnknownTextEncoding(u8),
  #[error("Decode text {}", _0)]
  DecodeText(Cow<'static, str>),
  #[error("Unknown weigh type {}", _0)]
  UnknownWeightType(u8),
  #[error("Index overflow {}", _0)]
  IndexOverflow(i64),
  #[error("Invalid environment blendMode {}", _0)]
  InvalidEnvironmentBlendMode(u8),
  #[error("Invalid toon reference {}", _0)]
  InvalidToonReference(u8),
  #[error("Invalid morph type {}", _0)]
  InvalidMorphType(u8),
  #[error("Invalid material offset method {}", _0)]
  InvalidMaterialOffsetMethod(u8),
  #[error("Invalid display frame type {}", _0)]
  InvalidFrameType(u8),
  #[error("Invalid rigid body shape type {}", _0)]
  InvalidShapeType(u8),
  #[error("Invalid rigid body physics mode {}", _0)]
  InvalidPhysicsMode(u8),
  #[error("Invalid joint type {}", _0)]
  InvalidJointType(u8),
}

pub type Result<T> = std::result::Result<T, Error>;
