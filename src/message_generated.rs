// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod asted {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};
#[allow(unused_imports, dead_code)]
pub mod interface {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_REQUEST_UNION: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_REQUEST_UNION: u8 = 2;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_REQUEST_UNION: [RequestUnion; 3] = [
  RequestUnion::NONE,
  RequestUnion::InitRequest,
  RequestUnion::FileRequest,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct RequestUnion(pub u8);
#[allow(non_upper_case_globals)]
impl RequestUnion {
  pub const NONE: Self = Self(0);
  pub const InitRequest: Self = Self(1);
  pub const FileRequest: Self = Self(2);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 2;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::InitRequest,
    Self::FileRequest,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::InitRequest => Some("InitRequest"),
      Self::FileRequest => Some("FileRequest"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for RequestUnion {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for RequestUnion {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for RequestUnion {
    type Output = RequestUnion;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for RequestUnion {
  type Scalar = u8;
  #[inline]
  fn to_little_endian(self) -> u8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u8) -> Self {
    let b = u8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for RequestUnion {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for RequestUnion {}
pub struct RequestUnionUnionTableOffset {}

// struct Location, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Location(pub [u8; 8]);
impl Default for Location { 
  fn default() -> Self { 
    Self([0; 8])
  }
}
impl core::fmt::Debug for Location {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("Location")
      .field("start_byte", &self.start_byte())
      .field("end_byte", &self.end_byte())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Location {}
impl<'a> flatbuffers::Follow<'a> for Location {
  type Inner = &'a Location;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Location>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Location {
  type Inner = &'a Location;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Location>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Location {
    type Output = Location;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const Location as *const u8, Self::size());
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for Location {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> Location {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    start_byte: u32,
    end_byte: u32,
  ) -> Self {
    let mut s = Self([0; 8]);
    s.set_start_byte(start_byte);
    s.set_end_byte(end_byte);
    s
  }

  pub fn start_byte(&self) -> u32 {
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

  pub fn set_start_byte(&mut self, x: u32) {
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

  pub fn end_byte(&self) -> u32 {
    let mut mem = core::mem::MaybeUninit::<<u32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_end_byte(&mut self, x: u32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<<u32 as EndianScalar>::Scalar>(),
      );
    }
  }

}

pub enum InitRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct InitRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for InitRequest<'a> {
  type Inner = InitRequest<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> InitRequest<'a> {
  pub const VT_LANG: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    InitRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args InitRequestArgs<'args>
  ) -> flatbuffers::WIPOffset<InitRequest<'bldr>> {
    let mut builder = InitRequestBuilder::new(_fbb);
    if let Some(x) = args.lang { builder.add_lang(x); }
    builder.finish()
  }


  #[inline]
  pub fn lang(&self) -> &'a str {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(InitRequest::VT_LANG, None).unwrap()}
  }
}

impl flatbuffers::Verifiable for InitRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("lang", Self::VT_LANG, true)?
     .finish();
    Ok(())
  }
}
pub struct InitRequestArgs<'a> {
    pub lang: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for InitRequestArgs<'a> {
  #[inline]
  fn default() -> Self {
    InitRequestArgs {
      lang: None, // required field
    }
  }
}

pub struct InitRequestBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> InitRequestBuilder<'a, 'b> {
  #[inline]
  pub fn add_lang(&mut self, lang: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(InitRequest::VT_LANG, lang);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> InitRequestBuilder<'a, 'b> {
    let start = _fbb.start_table();
    InitRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<InitRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, InitRequest::VT_LANG,"lang");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for InitRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("InitRequest");
      ds.field("lang", &self.lang());
      ds.finish()
  }
}
pub enum FileRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct FileRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for FileRequest<'a> {
  type Inner = FileRequest<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> FileRequest<'a> {
  pub const VT_PATH: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    FileRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args FileRequestArgs<'args>
  ) -> flatbuffers::WIPOffset<FileRequest<'bldr>> {
    let mut builder = FileRequestBuilder::new(_fbb);
    if let Some(x) = args.path { builder.add_path(x); }
    builder.finish()
  }


  #[inline]
  pub fn path(&self) -> &'a str {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(FileRequest::VT_PATH, None).unwrap()}
  }
}

impl flatbuffers::Verifiable for FileRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("path", Self::VT_PATH, true)?
     .finish();
    Ok(())
  }
}
pub struct FileRequestArgs<'a> {
    pub path: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for FileRequestArgs<'a> {
  #[inline]
  fn default() -> Self {
    FileRequestArgs {
      path: None, // required field
    }
  }
}

pub struct FileRequestBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> FileRequestBuilder<'a, 'b> {
  #[inline]
  pub fn add_path(&mut self, path: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(FileRequest::VT_PATH, path);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> FileRequestBuilder<'a, 'b> {
    let start = _fbb.start_table();
    FileRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<FileRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, FileRequest::VT_PATH,"path");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for FileRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("FileRequest");
      ds.field("path", &self.path());
      ds.finish()
  }
}
pub enum FileResponseOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct FileResponse<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for FileResponse<'a> {
  type Inner = FileResponse<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> FileResponse<'a> {
  pub const VT_TREE: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    FileResponse { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args FileResponseArgs<'args>
  ) -> flatbuffers::WIPOffset<FileResponse<'bldr>> {
    let mut builder = FileResponseBuilder::new(_fbb);
    if let Some(x) = args.tree { builder.add_tree(x); }
    builder.finish()
  }


  #[inline]
  pub fn tree(&self) -> Node<'a> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<Node>>(FileResponse::VT_TREE, None).unwrap()}
  }
}

impl flatbuffers::Verifiable for FileResponse<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<Node>>("tree", Self::VT_TREE, true)?
     .finish();
    Ok(())
  }
}
pub struct FileResponseArgs<'a> {
    pub tree: Option<flatbuffers::WIPOffset<Node<'a>>>,
}
impl<'a> Default for FileResponseArgs<'a> {
  #[inline]
  fn default() -> Self {
    FileResponseArgs {
      tree: None, // required field
    }
  }
}

pub struct FileResponseBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> FileResponseBuilder<'a, 'b> {
  #[inline]
  pub fn add_tree(&mut self, tree: flatbuffers::WIPOffset<Node<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Node>>(FileResponse::VT_TREE, tree);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> FileResponseBuilder<'a, 'b> {
    let start = _fbb.start_table();
    FileResponseBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<FileResponse<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, FileResponse::VT_TREE,"tree");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for FileResponse<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("FileResponse");
      ds.field("tree", &self.tree());
      ds.finish()
  }
}
pub enum RequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Request<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Request<'a> {
  type Inner = Request<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Request<'a> {
  pub const VT_REQUEST_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_REQUEST: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Request { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args RequestArgs
  ) -> flatbuffers::WIPOffset<Request<'bldr>> {
    let mut builder = RequestBuilder::new(_fbb);
    if let Some(x) = args.request { builder.add_request(x); }
    builder.add_request_type(args.request_type);
    builder.finish()
  }


  #[inline]
  pub fn request_type(&self) -> RequestUnion {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<RequestUnion>(Request::VT_REQUEST_TYPE, Some(RequestUnion::NONE)).unwrap()}
  }
  #[inline]
  pub fn request(&self) -> flatbuffers::Table<'a> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Request::VT_REQUEST, None).unwrap()}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn request_as_init_request(&self) -> Option<InitRequest<'a>> {
    if self.request_type() == RequestUnion::InitRequest {
      let u = self.request();
      // Safety:
      // Created from a valid Table for this object
      // Which contains a valid union in this slot
      Some(unsafe { InitRequest::init_from_table(u) })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn request_as_file_request(&self) -> Option<FileRequest<'a>> {
    if self.request_type() == RequestUnion::FileRequest {
      let u = self.request();
      // Safety:
      // Created from a valid Table for this object
      // Which contains a valid union in this slot
      Some(unsafe { FileRequest::init_from_table(u) })
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for Request<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_union::<RequestUnion, _>("request_type", Self::VT_REQUEST_TYPE, "request", Self::VT_REQUEST, true, |key, v, pos| {
        match key {
          RequestUnion::InitRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<InitRequest>>("RequestUnion::InitRequest", pos),
          RequestUnion::FileRequest => v.verify_union_variant::<flatbuffers::ForwardsUOffset<FileRequest>>("RequestUnion::FileRequest", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct RequestArgs {
    pub request_type: RequestUnion,
    pub request: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for RequestArgs {
  #[inline]
  fn default() -> Self {
    RequestArgs {
      request_type: RequestUnion::NONE,
      request: None, // required field
    }
  }
}

pub struct RequestBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RequestBuilder<'a, 'b> {
  #[inline]
  pub fn add_request_type(&mut self, request_type: RequestUnion) {
    self.fbb_.push_slot::<RequestUnion>(Request::VT_REQUEST_TYPE, request_type, RequestUnion::NONE);
  }
  #[inline]
  pub fn add_request(&mut self, request: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Request::VT_REQUEST, request);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RequestBuilder<'a, 'b> {
    let start = _fbb.start_table();
    RequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Request<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, Request::VT_REQUEST,"request");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Request<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Request");
      ds.field("request_type", &self.request_type());
      match self.request_type() {
        RequestUnion::InitRequest => {
          if let Some(x) = self.request_as_init_request() {
            ds.field("request", &x)
          } else {
            ds.field("request", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        RequestUnion::FileRequest => {
          if let Some(x) = self.request_as_file_request() {
            ds.field("request", &x)
          } else {
            ds.field("request", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("request", &x)
        },
      };
      ds.finish()
  }
}
pub enum NodeOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Node<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Node<'a> {
  type Inner = Node<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Node<'a> {
  pub const VT_KIND: flatbuffers::VOffsetT = 4;
  pub const VT_LOCATION: flatbuffers::VOffsetT = 6;
  pub const VT_CHILDREN: flatbuffers::VOffsetT = 8;
  pub const VT_NAMED: flatbuffers::VOffsetT = 10;
  pub const VT_TEXT: flatbuffers::VOffsetT = 12;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Node { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args NodeArgs<'args>
  ) -> flatbuffers::WIPOffset<Node<'bldr>> {
    let mut builder = NodeBuilder::new(_fbb);
    if let Some(x) = args.text { builder.add_text(x); }
    if let Some(x) = args.children { builder.add_children(x); }
    if let Some(x) = args.location { builder.add_location(x); }
    if let Some(x) = args.kind { builder.add_kind(x); }
    builder.add_named(args.named);
    builder.finish()
  }


  #[inline]
  pub fn kind(&self) -> &'a str {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Node::VT_KIND, None).unwrap()}
  }
  #[inline]
  pub fn location(&self) -> Option<&'a Location> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Location>(Node::VT_LOCATION, None)}
  }
  #[inline]
  pub fn children(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Node<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Node>>>>(Node::VT_CHILDREN, None)}
  }
  #[inline]
  pub fn named(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(Node::VT_NAMED, Some(false)).unwrap()}
  }
  #[inline]
  pub fn text(&self) -> Option<flatbuffers::Vector<'a, u16>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u16>>>(Node::VT_TEXT, None)}
  }
}

impl flatbuffers::Verifiable for Node<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("kind", Self::VT_KIND, true)?
     .visit_field::<Location>("location", Self::VT_LOCATION, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Node>>>>("children", Self::VT_CHILDREN, false)?
     .visit_field::<bool>("named", Self::VT_NAMED, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u16>>>("text", Self::VT_TEXT, false)?
     .finish();
    Ok(())
  }
}
pub struct NodeArgs<'a> {
    pub kind: Option<flatbuffers::WIPOffset<&'a str>>,
    pub location: Option<&'a Location>,
    pub children: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Node<'a>>>>>,
    pub named: bool,
    pub text: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u16>>>,
}
impl<'a> Default for NodeArgs<'a> {
  #[inline]
  fn default() -> Self {
    NodeArgs {
      kind: None, // required field
      location: None,
      children: None,
      named: false,
      text: None,
    }
  }
}

pub struct NodeBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> NodeBuilder<'a, 'b> {
  #[inline]
  pub fn add_kind(&mut self, kind: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Node::VT_KIND, kind);
  }
  #[inline]
  pub fn add_location(&mut self, location: &Location) {
    self.fbb_.push_slot_always::<&Location>(Node::VT_LOCATION, location);
  }
  #[inline]
  pub fn add_children(&mut self, children: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Node<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Node::VT_CHILDREN, children);
  }
  #[inline]
  pub fn add_named(&mut self, named: bool) {
    self.fbb_.push_slot::<bool>(Node::VT_NAMED, named, false);
  }
  #[inline]
  pub fn add_text(&mut self, text: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u16>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Node::VT_TEXT, text);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> NodeBuilder<'a, 'b> {
    let start = _fbb.start_table();
    NodeBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Node<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, Node::VT_KIND,"kind");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Node<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Node");
      ds.field("kind", &self.kind());
      ds.field("location", &self.location());
      ds.field("children", &self.children());
      ds.field("named", &self.named());
      ds.field("text", &self.text());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `Request`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_request_unchecked`.
pub fn root_as_request(buf: &[u8]) -> Result<Request, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Request>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Request` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_request_unchecked`.
pub fn size_prefixed_root_as_request(buf: &[u8]) -> Result<Request, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Request>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Request` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_request_unchecked`.
pub fn root_as_request_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Request<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Request<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Request` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_request_unchecked`.
pub fn size_prefixed_root_as_request_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Request<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Request<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Request and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Request`.
pub unsafe fn root_as_request_unchecked(buf: &[u8]) -> Request {
  flatbuffers::root_unchecked::<Request>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Request and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Request`.
pub unsafe fn size_prefixed_root_as_request_unchecked(buf: &[u8]) -> Request {
  flatbuffers::size_prefixed_root_unchecked::<Request>(buf)
}
#[inline]
pub fn finish_request_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Request<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_request_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Request<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod Interface
}  // pub mod ASTEd

