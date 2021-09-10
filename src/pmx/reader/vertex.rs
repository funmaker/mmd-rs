use crate::{
  pmx::weight_deform::*,
  reader::{helpers::ReadHelpers, HeaderReader},
  Error, Index, Result, Settings, Vertex,
};
use byteorder::{ReadBytesExt, LE};
use std::io::Read;
use std::marker::PhantomData;

pub struct VertexReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
}

impl<R: Read> VertexReader<R> {
  pub fn new(mut header: HeaderReader<R>) -> Result<VertexReader<R>> {
    let count = header.read.read_i32::<LE>()?;
    Ok(VertexReader {
      settings: header.settings,
      count,
      remaining: count,
      read: header.read,
    })
  }

  pub fn next_vertex<BoneIndex: Index>(&mut self) -> Result<Option<Vertex<BoneIndex>>> {
    if self.remaining == 0 {
      return Ok(None);
    }
    let position = self.read.read_vec3()?;
    let normal = self.read.read_vec3()?;
    let uv = self.read.read_vec2()?;
    let mut additional = [[0f32; 4]; 4];
    for i in 0..self.settings.additional_vec4_count {
      additional[i as usize] = self.read.read_vec4()?;
    }

    let weight_deform = match self.read.read_u8()? {
      0u8 => WeightDeform::Bdef1(Bdef1::<BoneIndex> {
        bone_index: self.read.read_index(self.settings.bone_index_size)?,
      }),
      1u8 => WeightDeform::Bdef2(Bdef2::<BoneIndex> {
        bone_1_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_2_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_1_weight: self.read.read_f32::<LE>()?,
      }),
      2u8 => WeightDeform::Bdef4(Bdef4::<BoneIndex> {
        bone_1_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_2_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_3_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_4_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_1_weight: self.read.read_f32::<LE>()?,
        bone_2_weight: self.read.read_f32::<LE>()?,
        bone_3_weight: self.read.read_f32::<LE>()?,
        bone_4_weight: self.read.read_f32::<LE>()?,
      }),
      3u8 => WeightDeform::Sdef(Sdef::<BoneIndex> {
        bone_1_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_2_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_1_weight: self.read.read_f32::<LE>()?,
        c: self.read.read_vec3()?,
        r0: self.read.read_vec3()?,
        r1: self.read.read_vec3()?,
      }),
      4u8 => WeightDeform::Qdef(Qdef::<BoneIndex> {
        bone_1_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_2_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_3_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_4_index: self.read.read_index(self.settings.bone_index_size)?,
        bone_1_weight: self.read.read_f32::<LE>()?,
        bone_2_weight: self.read.read_f32::<LE>()?,
        bone_3_weight: self.read.read_f32::<LE>()?,
        bone_4_weight: self.read.read_f32::<LE>()?,
      }),
      e => return Err(Error::UnknownWeightType(e)),
    };

    self.remaining -= 1;
    Ok(Some(Vertex {
      position,
      normal,
      uv,
      additional,
      weight_deform,
      edge_scale: self.read.read_f32::<LE>()?,
    }))
  }

  pub fn iter<BoneIndex>(&mut self) -> VertexIterator<R, BoneIndex> {
    VertexIterator {
      reader: self,
      phantom: PhantomData,
    }
  }
}

pub struct VertexIterator<'a, R, BoneIndex = i32> {
  reader: &'a mut VertexReader<R>,
  phantom: PhantomData<BoneIndex>,
}

impl<'a, R: Read, BoneIndex: Index> Iterator for VertexIterator<'a, R, BoneIndex> {
  type Item = Result<Vertex<BoneIndex>>;

  fn next(&mut self) -> Option<Self::Item> {
    self.reader.next_vertex().map_or(None, |v| v.map(Ok))
  }
}
