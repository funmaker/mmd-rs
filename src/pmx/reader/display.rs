use crate::{
  pmx::display::*,
  reader::{helpers::ReadHelpers, MorphReader},
  Error, Index, Result, Settings,
};
use byteorder::{ReadBytesExt, LE};
use std::io::Read;
use std::marker::PhantomData;

pub struct DisplayReader<R> {
  pub settings: Settings,
  pub count: i32,
  pub remaining: i32,
  pub(crate) read: R,
}

impl<R: Read> DisplayReader<R> {
  pub fn new(mut v: MorphReader<R>) -> Result<DisplayReader<R>> {
    while v.remaining > 0 {
      v.next::<i32, i32, i32, i32, i32>()?;
    }
    let count = v.read.read_i32::<LE>()?;

    Ok(DisplayReader {
      settings: v.settings,
      count,
      remaining: count,
      read: v.read,
    })
  }

  pub fn next<BoneIndex, MorphIndex>(&mut self) -> Result<Option<DisplayFrame<BoneIndex, MorphIndex>>>
  where
    BoneIndex: Index,
    MorphIndex: Index,
  {
    if self.remaining <= 0 {
      return Ok(None);
    }

    self.remaining -= 1;
    
    let local_name = self.read.read_text(self.settings.text_encoding)?;
    let universal_name = self.read.read_text(self.settings.text_encoding)?;
    let special_flag = self.read.read_u8()? != 0;
    let frame_count = self.read.read_u32::<LE>()?;
    let mut frames = Vec::with_capacity(frame_count as usize);
    
    for _ in 0..frame_count {
      let frame = match self.read.read_u8()? {
        0 => Frame::Bone(self.read.read_index(self.settings.bone_index_size)?),
        1 => Frame::Morph(self.read.read_index(self.settings.morph_index_size)?),
        e => return Err(Error::InvalidFrameType(e)),
      };
      
      frames.push(frame);
    }

    Ok(Some(DisplayFrame {
      local_name,
      universal_name,
      special_flag,
      frames,
    }))
  }

  pub fn iter<BoneIndex, MorphIndex>(&mut self) -> DisplayIterator<R, BoneIndex, MorphIndex> {
    DisplayIterator {
      reader: self,
      phantom: PhantomData,
    }
  }
}

pub struct DisplayIterator<'a, R, BoneIndex = i32, MorphIndex = i32> {
  reader: &'a mut DisplayReader<R>,
  phantom: PhantomData<(BoneIndex, MorphIndex)>,
}

impl<R: Read, BoneIndex: Index, MorphIndex: Index> Iterator for DisplayIterator<'_, R, BoneIndex, MorphIndex> {
  type Item = Result<DisplayFrame<BoneIndex, MorphIndex>>;

  fn next(&mut self) -> Option<Self::Item> {
    self.reader.next().map_or(None, |v| v.map(Ok))
  }
}
