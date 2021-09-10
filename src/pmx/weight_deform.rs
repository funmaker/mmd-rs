pub struct Bdef1<BoneIndex> {
  pub bone_index: BoneIndex,
}

pub struct Bdef2<BoneIndex> {
  pub bone_1_index: BoneIndex,
  pub bone_2_index: BoneIndex,
  pub bone_1_weight: f32,
}

pub struct Bdef4<BoneIndex> {
  pub bone_1_index: BoneIndex,
  pub bone_2_index: BoneIndex,
  pub bone_3_index: BoneIndex,
  pub bone_4_index: BoneIndex,
  pub bone_1_weight: f32,
  pub bone_2_weight: f32,
  pub bone_3_weight: f32,
  pub bone_4_weight: f32,
}

pub struct Sdef<BoneIndex> {
  pub bone_1_index: BoneIndex,
  pub bone_2_index: BoneIndex,
  pub bone_1_weight: f32,
  pub c: [f32; 3],
  pub r0: [f32; 3],
  pub r1: [f32; 3],
}

pub struct Qdef<BoneIndex> {
  pub bone_1_index: BoneIndex,
  pub bone_2_index: BoneIndex,
  pub bone_3_index: BoneIndex,
  pub bone_4_index: BoneIndex,
  pub bone_1_weight: f32,
  pub bone_2_weight: f32,
  pub bone_3_weight: f32,
  pub bone_4_weight: f32,
}

pub enum WeightDeform<BoneIndex> {
  Bdef1(Bdef1<BoneIndex>),
  Bdef2(Bdef2<BoneIndex>),
  Bdef4(Bdef4<BoneIndex>),
  Sdef(Sdef<BoneIndex>),
  Qdef(Qdef<BoneIndex>),
}
