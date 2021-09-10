use std::fmt::{Debug, Display, Formatter};
use itertools::Itertools;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Frame<BoneIndex, MorphIndex> {
	Bone(BoneIndex),
	Morph(MorphIndex),
}

impl<BoneIndex: Display, MorphIndex: Display> Display for Frame<BoneIndex, MorphIndex> {
	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
		match self {
			Frame::Bone(id) => write!(f, "bone {}", id),
			Frame::Morph(id) => write!(f, "morph {}", id),
		}
	}
}

pub struct DisplayFrame<BoneIndex, MorphIndex> {
	pub local_name: String,
	pub universal_name: String,
	pub special_flag: bool,
	pub frames: Vec<Frame<BoneIndex, MorphIndex>>,
}

impl<BoneIndex, MorphIndex> Display for DisplayFrame<BoneIndex, MorphIndex>
	where
		Frame<BoneIndex, MorphIndex>: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(
			f,
			r"local name: {}, universal name: {},
flag: {}, frames: {}",
			self.local_name,
			self.universal_name,
			if self.special_flag { "special" } else { "normal" },
			self.frames.iter().map(ToString::to_string).join(", "),
		)
	}
}
