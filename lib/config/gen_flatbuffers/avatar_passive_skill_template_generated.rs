// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

// struct RefineCost, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct RefineCost(pub [u8; 8]);
impl Default for RefineCost { 
  fn default() -> Self { 
    Self([0; 8])
  }
}
impl core::fmt::Debug for RefineCost {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("RefineCost")
      .field("item_id", &self.item_id())
      .field("number", &self.number())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for RefineCost {}
impl<'a> flatbuffers::Follow<'a> for RefineCost {
  type Inner = &'a RefineCost;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a RefineCost>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a RefineCost {
  type Inner = &'a RefineCost;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<RefineCost>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for RefineCost {
    type Output = RefineCost;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const RefineCost as *const u8, <Self as flatbuffers::Push>::size());
        dst.copy_from_slice(src);
    }
    #[inline]
    fn alignment() -> flatbuffers::PushAlignment {
        flatbuffers::PushAlignment::new(4)
    }
}

impl<'a> flatbuffers::Verifiable for RefineCost {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> RefineCost {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    item_id: u32,
    number: i32,
  ) -> Self {
    let mut s = Self([0; 8]);
    s.set_item_id(item_id);
    s.set_number(number);
    s
  }

  pub fn item_id(&self) -> u32 {
    let mut mem = core::mem::MaybeUninit::<<u32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_item_id(&mut self, x: u32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn number(&self) -> i32 {
    let mut mem = core::mem::MaybeUninit::<<i32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<i32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_number(&mut self, x: i32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<<i32 as EndianScalar>::Scalar>(),
      );
    }
  }

}

// struct Property, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Property(pub [u8; 8]);
impl Default for Property { 
  fn default() -> Self { 
    Self([0; 8])
  }
}
impl core::fmt::Debug for Property {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("Property")
      .field("property", &self.property())
      .field("value", &self.value())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Property {}
impl<'a> flatbuffers::Follow<'a> for Property {
  type Inner = &'a Property;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Property>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Property {
  type Inner = &'a Property;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Property>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Property {
    type Output = Property;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const Property as *const u8, <Self as flatbuffers::Push>::size());
        dst.copy_from_slice(src);
    }
    #[inline]
    fn alignment() -> flatbuffers::PushAlignment {
        flatbuffers::PushAlignment::new(4)
    }
}

impl<'a> flatbuffers::Verifiable for Property {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> Property {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    property: u32,
    value: i32,
  ) -> Self {
    let mut s = Self([0; 8]);
    s.set_property(property);
    s.set_value(value);
    s
  }

  pub fn property(&self) -> u32 {
    let mut mem = core::mem::MaybeUninit::<<u32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_property(&mut self, x: u32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn value(&self) -> i32 {
    let mut mem = core::mem::MaybeUninit::<<i32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<i32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_value(&mut self, x: i32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<<i32 as EndianScalar>::Scalar>(),
      );
    }
  }

}

pub enum AvatarPassiveSkillTemplateOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct AvatarPassiveSkillTemplate<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for AvatarPassiveSkillTemplate<'a> {
  type Inner = AvatarPassiveSkillTemplate<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> AvatarPassiveSkillTemplate<'a> {
  pub const VT_SKILL_ID: flatbuffers::VOffsetT = 4;
  pub const VT_AVATAR_ID: flatbuffers::VOffsetT = 6;
  pub const VT_MIN_AVATAR_LEVEL: flatbuffers::VOffsetT = 8;
  pub const VT_MIN_PASSIVE_SKILL_LEVEL: flatbuffers::VOffsetT = 10;
  pub const VT_UNLOCK_PASSIVE_SKILL_LEVEL: flatbuffers::VOffsetT = 12;
  pub const VT_UNK_5: flatbuffers::VOffsetT = 14;
  pub const VT_UNK_LEVELUP: flatbuffers::VOffsetT = 16;
  pub const VT_UNK_7: flatbuffers::VOffsetT = 18;
  pub const VT_UNK_8: flatbuffers::VOffsetT = 20;
  pub const VT_UNK_9: flatbuffers::VOffsetT = 22;
  pub const VT_UNK_10: flatbuffers::VOffsetT = 24;
  pub const VT_PROPERTYS: flatbuffers::VOffsetT = 26;
  pub const VT_NAMES: flatbuffers::VOffsetT = 28;
  pub const VT_DESCRIPTIONS: flatbuffers::VOffsetT = 30;
  pub const VT_MATERIALS_COSTS: flatbuffers::VOffsetT = 32;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    AvatarPassiveSkillTemplate { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args AvatarPassiveSkillTemplateArgs<'args>
  ) -> flatbuffers::WIPOffset<AvatarPassiveSkillTemplate<'bldr>> {
    let mut builder = AvatarPassiveSkillTemplateBuilder::new(_fbb);
    if let Some(x) = args.materials_costs { builder.add_materials_costs(x); }
    if let Some(x) = args.descriptions { builder.add_descriptions(x); }
    if let Some(x) = args.names { builder.add_names(x); }
    if let Some(x) = args.propertys { builder.add_propertys(x); }
    builder.add_unk_10(args.unk_10);
    builder.add_unk_9(args.unk_9);
    builder.add_unk_8(args.unk_8);
    builder.add_unk_7(args.unk_7);
    if let Some(x) = args.unk_levelup { builder.add_unk_levelup(x); }
    builder.add_unk_5(args.unk_5);
    builder.add_unlock_passive_skill_level(args.unlock_passive_skill_level);
    builder.add_min_passive_skill_level(args.min_passive_skill_level);
    builder.add_min_avatar_level(args.min_avatar_level);
    builder.add_avatar_id(args.avatar_id);
    builder.add_skill_id(args.skill_id);
    builder.finish()
  }


  #[inline]
  pub fn skill_id(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(AvatarPassiveSkillTemplate::VT_SKILL_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn avatar_id(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(AvatarPassiveSkillTemplate::VT_AVATAR_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn min_avatar_level(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(AvatarPassiveSkillTemplate::VT_MIN_AVATAR_LEVEL, Some(0)).unwrap()}
  }
  #[inline]
  pub fn min_passive_skill_level(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(AvatarPassiveSkillTemplate::VT_MIN_PASSIVE_SKILL_LEVEL, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unlock_passive_skill_level(&self) -> u32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u32>(AvatarPassiveSkillTemplate::VT_UNLOCK_PASSIVE_SKILL_LEVEL, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_5(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(AvatarPassiveSkillTemplate::VT_UNK_5, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_levelup(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(AvatarPassiveSkillTemplate::VT_UNK_LEVELUP, None)}
  }
  #[inline]
  pub fn unk_7(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(AvatarPassiveSkillTemplate::VT_UNK_7, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_8(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(AvatarPassiveSkillTemplate::VT_UNK_8, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_9(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(AvatarPassiveSkillTemplate::VT_UNK_9, Some(0)).unwrap()}
  }
  #[inline]
  pub fn unk_10(&self) -> i32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i32>(AvatarPassiveSkillTemplate::VT_UNK_10, Some(0)).unwrap()}
  }
  #[inline]
  pub fn propertys(&self) -> Option<flatbuffers::Vector<'a, Property>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, Property>>>(AvatarPassiveSkillTemplate::VT_PROPERTYS, None)}
  }
  #[inline]
  pub fn names(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>(AvatarPassiveSkillTemplate::VT_NAMES, None)}
  }
  #[inline]
  pub fn descriptions(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>(AvatarPassiveSkillTemplate::VT_DESCRIPTIONS, None)}
  }
  #[inline]
  pub fn materials_costs(&self) -> Option<flatbuffers::Vector<'a, RefineCost>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, RefineCost>>>(AvatarPassiveSkillTemplate::VT_MATERIALS_COSTS, None)}
  }
}

impl flatbuffers::Verifiable for AvatarPassiveSkillTemplate<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i32>("skill_id", Self::VT_SKILL_ID, false)?
     .visit_field::<u32>("avatar_id", Self::VT_AVATAR_ID, false)?
     .visit_field::<u32>("min_avatar_level", Self::VT_MIN_AVATAR_LEVEL, false)?
     .visit_field::<u32>("min_passive_skill_level", Self::VT_MIN_PASSIVE_SKILL_LEVEL, false)?
     .visit_field::<u32>("unlock_passive_skill_level", Self::VT_UNLOCK_PASSIVE_SKILL_LEVEL, false)?
     .visit_field::<i32>("unk_5", Self::VT_UNK_5, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("unk_levelup", Self::VT_UNK_LEVELUP, false)?
     .visit_field::<i32>("unk_7", Self::VT_UNK_7, false)?
     .visit_field::<i32>("unk_8", Self::VT_UNK_8, false)?
     .visit_field::<i32>("unk_9", Self::VT_UNK_9, false)?
     .visit_field::<i32>("unk_10", Self::VT_UNK_10, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, Property>>>("propertys", Self::VT_PROPERTYS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<&'_ str>>>>("names", Self::VT_NAMES, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<&'_ str>>>>("descriptions", Self::VT_DESCRIPTIONS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, RefineCost>>>("materials_costs", Self::VT_MATERIALS_COSTS, false)?
     .finish();
    Ok(())
  }
}
pub struct AvatarPassiveSkillTemplateArgs<'a> {
    pub skill_id: i32,
    pub avatar_id: u32,
    pub min_avatar_level: u32,
    pub min_passive_skill_level: u32,
    pub unlock_passive_skill_level: u32,
    pub unk_5: i32,
    pub unk_levelup: Option<flatbuffers::WIPOffset<&'a str>>,
    pub unk_7: i32,
    pub unk_8: i32,
    pub unk_9: i32,
    pub unk_10: i32,
    pub propertys: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, Property>>>,
    pub names: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>,
    pub descriptions: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>>,
    pub materials_costs: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, RefineCost>>>,
}
impl<'a> Default for AvatarPassiveSkillTemplateArgs<'a> {
  #[inline]
  fn default() -> Self {
    AvatarPassiveSkillTemplateArgs {
      skill_id: 0,
      avatar_id: 0,
      min_avatar_level: 0,
      min_passive_skill_level: 0,
      unlock_passive_skill_level: 0,
      unk_5: 0,
      unk_levelup: None,
      unk_7: 0,
      unk_8: 0,
      unk_9: 0,
      unk_10: 0,
      propertys: None,
      names: None,
      descriptions: None,
      materials_costs: None,
    }
  }
}

pub struct AvatarPassiveSkillTemplateBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> AvatarPassiveSkillTemplateBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_skill_id(&mut self, skill_id: i32) {
    self.fbb_.push_slot::<i32>(AvatarPassiveSkillTemplate::VT_SKILL_ID, skill_id, 0);
  }
  #[inline]
  pub fn add_avatar_id(&mut self, avatar_id: u32) {
    self.fbb_.push_slot::<u32>(AvatarPassiveSkillTemplate::VT_AVATAR_ID, avatar_id, 0);
  }
  #[inline]
  pub fn add_min_avatar_level(&mut self, min_avatar_level: u32) {
    self.fbb_.push_slot::<u32>(AvatarPassiveSkillTemplate::VT_MIN_AVATAR_LEVEL, min_avatar_level, 0);
  }
  #[inline]
  pub fn add_min_passive_skill_level(&mut self, min_passive_skill_level: u32) {
    self.fbb_.push_slot::<u32>(AvatarPassiveSkillTemplate::VT_MIN_PASSIVE_SKILL_LEVEL, min_passive_skill_level, 0);
  }
  #[inline]
  pub fn add_unlock_passive_skill_level(&mut self, unlock_passive_skill_level: u32) {
    self.fbb_.push_slot::<u32>(AvatarPassiveSkillTemplate::VT_UNLOCK_PASSIVE_SKILL_LEVEL, unlock_passive_skill_level, 0);
  }
  #[inline]
  pub fn add_unk_5(&mut self, unk_5: i32) {
    self.fbb_.push_slot::<i32>(AvatarPassiveSkillTemplate::VT_UNK_5, unk_5, 0);
  }
  #[inline]
  pub fn add_unk_levelup(&mut self, unk_levelup: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AvatarPassiveSkillTemplate::VT_UNK_LEVELUP, unk_levelup);
  }
  #[inline]
  pub fn add_unk_7(&mut self, unk_7: i32) {
    self.fbb_.push_slot::<i32>(AvatarPassiveSkillTemplate::VT_UNK_7, unk_7, 0);
  }
  #[inline]
  pub fn add_unk_8(&mut self, unk_8: i32) {
    self.fbb_.push_slot::<i32>(AvatarPassiveSkillTemplate::VT_UNK_8, unk_8, 0);
  }
  #[inline]
  pub fn add_unk_9(&mut self, unk_9: i32) {
    self.fbb_.push_slot::<i32>(AvatarPassiveSkillTemplate::VT_UNK_9, unk_9, 0);
  }
  #[inline]
  pub fn add_unk_10(&mut self, unk_10: i32) {
    self.fbb_.push_slot::<i32>(AvatarPassiveSkillTemplate::VT_UNK_10, unk_10, 0);
  }
  #[inline]
  pub fn add_propertys(&mut self, propertys: flatbuffers::WIPOffset<flatbuffers::Vector<'b , Property>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AvatarPassiveSkillTemplate::VT_PROPERTYS, propertys);
  }
  #[inline]
  pub fn add_names(&mut self, names: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AvatarPassiveSkillTemplate::VT_NAMES, names);
  }
  #[inline]
  pub fn add_descriptions(&mut self, descriptions: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AvatarPassiveSkillTemplate::VT_DESCRIPTIONS, descriptions);
  }
  #[inline]
  pub fn add_materials_costs(&mut self, materials_costs: flatbuffers::WIPOffset<flatbuffers::Vector<'b , RefineCost>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AvatarPassiveSkillTemplate::VT_MATERIALS_COSTS, materials_costs);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> AvatarPassiveSkillTemplateBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    AvatarPassiveSkillTemplateBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<AvatarPassiveSkillTemplate<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for AvatarPassiveSkillTemplate<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("AvatarPassiveSkillTemplate");
      ds.field("skill_id", &self.skill_id());
      ds.field("avatar_id", &self.avatar_id());
      ds.field("min_avatar_level", &self.min_avatar_level());
      ds.field("min_passive_skill_level", &self.min_passive_skill_level());
      ds.field("unlock_passive_skill_level", &self.unlock_passive_skill_level());
      ds.field("unk_5", &self.unk_5());
      ds.field("unk_levelup", &self.unk_levelup());
      ds.field("unk_7", &self.unk_7());
      ds.field("unk_8", &self.unk_8());
      ds.field("unk_9", &self.unk_9());
      ds.field("unk_10", &self.unk_10());
      ds.field("propertys", &self.propertys());
      ds.field("names", &self.names());
      ds.field("descriptions", &self.descriptions());
      ds.field("materials_costs", &self.materials_costs());
      ds.finish()
  }
}
pub enum AvatarPassiveSkillTemplateTbOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct AvatarPassiveSkillTemplateTb<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for AvatarPassiveSkillTemplateTb<'a> {
  type Inner = AvatarPassiveSkillTemplateTb<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> AvatarPassiveSkillTemplateTb<'a> {
  pub const VT_DATA: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    AvatarPassiveSkillTemplateTb { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args AvatarPassiveSkillTemplateTbArgs<'args>
  ) -> flatbuffers::WIPOffset<AvatarPassiveSkillTemplateTb<'bldr>> {
    let mut builder = AvatarPassiveSkillTemplateTbBuilder::new(_fbb);
    if let Some(x) = args.data { builder.add_data(x); }
    builder.finish()
  }


  #[inline]
  pub fn data(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<AvatarPassiveSkillTemplate<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<AvatarPassiveSkillTemplate>>>>(AvatarPassiveSkillTemplateTb::VT_DATA, None)}
  }
}

impl flatbuffers::Verifiable for AvatarPassiveSkillTemplateTb<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<AvatarPassiveSkillTemplate>>>>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct AvatarPassiveSkillTemplateTbArgs<'a> {
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<AvatarPassiveSkillTemplate<'a>>>>>,
}
impl<'a> Default for AvatarPassiveSkillTemplateTbArgs<'a> {
  #[inline]
  fn default() -> Self {
    AvatarPassiveSkillTemplateTbArgs {
      data: None,
    }
  }
}

pub struct AvatarPassiveSkillTemplateTbBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> AvatarPassiveSkillTemplateTbBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<AvatarPassiveSkillTemplate<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AvatarPassiveSkillTemplateTb::VT_DATA, data);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> AvatarPassiveSkillTemplateTbBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    AvatarPassiveSkillTemplateTbBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<AvatarPassiveSkillTemplateTb<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for AvatarPassiveSkillTemplateTb<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("AvatarPassiveSkillTemplateTb");
      ds.field("data", &self.data());
      ds.finish()
  }
}
