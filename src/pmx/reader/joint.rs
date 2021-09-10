use crate::{
  pmx::joint::*,
  reader::{helpers::ReadHelpers, RigidBodyReader},
  Index, Result, Settings,
};
use byteorder::{ReadBytesExt, LE};
use std::io::Read;
use std::marker::PhantomData;
use std::convert::TryFrom;

pub struct JointReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
}

impl<R: Read> JointReader<R> {
  pub fn new(mut v: RigidBodyReader<R>) -> Result<JointReader<R>> {
    while v.remaining > 0 {
      v.next::<i32>()?;
    }
    let count = v.read.read_i32::<LE>()?;

    Ok(JointReader {
      settings: v.settings,
      count,
      remaining: count,
      read: v.read,
    })
  }

  pub fn next<RigidBodyIndex: Index>(&mut self) -> Result<Option<Joint<RigidBodyIndex>>> {
    if self.remaining <= 0 {
      return Ok(None);
    }

    self.remaining -= 1;

    Ok(Some(Joint {
      local_name: self.read.read_text(self.settings.text_encoding)?,
      universal_name: self.read.read_text(self.settings.text_encoding)?,
      joint_type: JointType::try_from(self.read.read_u8()?)?,
      rigid_body_a: self.read.read_index(self.settings.rigid_body_index_size)?,
      rigid_body_b: self.read.read_index(self.settings.rigid_body_index_size)?,
      position: self.read.read_vec3()?,
      rotation: self.read.read_vec3()?,
      position_min: self.read.read_vec3()?,
      position_max: self.read.read_vec3()?,
      rotation_min: self.read.read_vec3()?,
      rotation_max: self.read.read_vec3()?,
      position_spring: self.read.read_vec3()?,
      rotation_spring: self.read.read_vec3()?,
    }))
  }

  pub fn iter<RigidBodyIndex>(&mut self) -> JointIterator<R, RigidBodyIndex> {
    JointIterator {
      reader: self,
      phantom: PhantomData,
    }
  }
}

pub struct JointIterator<'a, R, RigidBodyIndex = i32> {
  reader: &'a mut JointReader<R>,
  phantom: PhantomData<RigidBodyIndex>,
}

impl<R: Read, RigidBodyIndex: Index> Iterator for JointIterator<'_, R, RigidBodyIndex> {
  type Item = Result<Joint<RigidBodyIndex>>;

  fn next(&mut self) -> Option<Self::Item> {
    self.reader.next().map_or(None, |v| v.map(Ok))
  }
}
