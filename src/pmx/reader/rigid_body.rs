use crate::{
  pmx::rigid_body::*,
  reader::{helpers::ReadHelpers, DisplayReader},
  Index, Result, Settings,
};
use byteorder::{ReadBytesExt, LE};
use std::io::Read;
use std::marker::PhantomData;
use std::convert::TryFrom;

pub struct RigidBodyReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
}

impl<R: Read> RigidBodyReader<R> {
  pub fn new(mut v: DisplayReader<R>) -> Result<RigidBodyReader<R>> {
    while v.remaining > 0 {
      v.next::<i32, i32>()?;
    }
    let count = v.read.read_i32::<LE>()?;

    Ok(RigidBodyReader {
      settings: v.settings,
      count,
      remaining: count,
      read: v.read,
    })
  }

  pub fn next<BI: Index>(&mut self) -> Result<Option<RigidBody<BI>>> {
    if self.remaining <= 0 {
      return Ok(None);
    }

    self.remaining -= 1;

    Ok(Some(RigidBody {
      local_name: self.read.read_text(self.settings.text_encoding)?,
      universal_name: self.read.read_text(self.settings.text_encoding)?,
      bone_index: self.read.read_index(self.settings.bone_index_size)?,
      group_id: self.read.read_u8()?,
      non_collision_mask: self.read.read_u16::<LE>()?,
      shape: ShapeType::try_from(self.read.read_u8()?)?,
      shape_size: self.read.read_vec3()?,
      shape_position: self.read.read_vec3()?,
      shape_rotation: self.read.read_vec3()?,
      mass: self.read.read_f32::<LE>()?,
      move_attenuation: self.read.read_f32::<LE>()?,
      rotation_damping: self.read.read_f32::<LE>()?,
      repulsion: self.read.read_f32::<LE>()?,
      fiction: self.read.read_f32::<LE>()?,
      physics_mode: PhysicsMode::try_from(self.read.read_u8()?)?,
    }))
  }

  pub fn iter<BI>(&mut self) -> RigidBodyIterator<R, BI> {
    RigidBodyIterator {
      reader: self,
      phantom: PhantomData,
    }
  }
}

pub struct RigidBodyIterator<'a, R, BI = i32> {
  reader: &'a mut RigidBodyReader<R>,
  phantom: PhantomData<BI>,
}

impl<R: Read, BI: Index> Iterator for RigidBodyIterator<'_, R, BI> {
  type Item = Result<RigidBody<BI>>;

  fn next(&mut self) -> Option<Self::Item> {
    self.reader.next().map_or(None, |v| v.map(Ok))
  }
}
