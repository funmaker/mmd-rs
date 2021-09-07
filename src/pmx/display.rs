use std::fmt::{Debug, Display, Formatter};
use itertools::Itertools;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Frame<BI, MoI> {
	Bone(BI),
	Morph(MoI),
}

impl<BI: Display, MoI: Display> Display for Frame<BI, MoI> {
	fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
		match self {
			Frame::Bone(id) => write!(f, "bone {}", id),
			Frame::Morph(id) => write!(f, "morph {}", id),
		}
	}
}

pub struct DisplayFrame<BI, MoI> {
	pub local_name: String,
	pub universal_name: String,
	pub special_flag: bool,
	pub frames: Vec<Frame<BI, MoI>>,
}

impl<BI, MoI> Display for DisplayFrame<BI, MoI>
	where
		Frame<BI, MoI>: Display,
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
