use std::ffi::{CStr, CString};
use std::num::NonZeroI32;
use std::os::raw::*;
use std::ptr::slice_from_raw_parts;
use std::str::FromStr;
use std::{slice, vec};

mod sys {
    pub use crate::md4c_sys::md4c::*;
}

mod traits;
pub use traits::*;

pub type MdChar = sys::MD_CHAR;
pub type MdSize = sys::MD_SIZE;
pub type MdOffset = sys::MD_OFFSET;

pub type MdResult = Result<(), NonZeroI32>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MdBlocktype {
    Doc,
    Quote,
    Ul,
    Ol,
    Li,
    Hr,
    H,
    Code,
    Html,
    P,
    Table,
    Thead,
    Tbody,
    Tr,
    Th,
    Td,
}

impl From<sys::MD_BLOCKTYPE> for MdBlocktype {
    #[inline]
    fn from(raw_blocktype: sys::MD_BLOCKTYPE) -> Self {
        match raw_blocktype {
            sys::MD_BLOCKTYPE_MD_BLOCK_DOC => Self::Doc,
            sys::MD_BLOCKTYPE_MD_BLOCK_QUOTE => Self::Quote,
            sys::MD_BLOCKTYPE_MD_BLOCK_UL => Self::Ul,
            sys::MD_BLOCKTYPE_MD_BLOCK_OL => Self::Ol,
            sys::MD_BLOCKTYPE_MD_BLOCK_LI => Self::Li,
            sys::MD_BLOCKTYPE_MD_BLOCK_HR => Self::Hr,
            sys::MD_BLOCKTYPE_MD_BLOCK_H => Self::H,
            sys::MD_BLOCKTYPE_MD_BLOCK_CODE => Self::Code,
            sys::MD_BLOCKTYPE_MD_BLOCK_HTML => Self::Html,
            sys::MD_BLOCKTYPE_MD_BLOCK_P => Self::P,
            sys::MD_BLOCKTYPE_MD_BLOCK_TABLE => Self::Table,
            sys::MD_BLOCKTYPE_MD_BLOCK_THEAD => Self::Thead,
            sys::MD_BLOCKTYPE_MD_BLOCK_TBODY => Self::Tbody,
            sys::MD_BLOCKTYPE_MD_BLOCK_TR => Self::Tr,
            sys::MD_BLOCKTYPE_MD_BLOCK_TH => Self::Th,
            sys::MD_BLOCKTYPE_MD_BLOCK_TD => Self::Td,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MdSpantype {
    Em,
    Strong,
    A,
    Img,
    Code,
    Del,
    Latexmath,
    LatexmathDisplay,
    Wikilink,
    U,
}

impl From<sys::MD_SPANTYPE> for MdSpantype {
    #[inline]
    fn from(raw_spantype: sys::MD_SPANTYPE) -> Self {
        match raw_spantype {
            sys::MD_SPANTYPE_MD_SPAN_EM => Self::Em,
            sys::MD_SPANTYPE_MD_SPAN_STRONG => Self::Strong,
            sys::MD_SPANTYPE_MD_SPAN_A => Self::A,
            sys::MD_SPANTYPE_MD_SPAN_IMG => Self::Img,
            sys::MD_SPANTYPE_MD_SPAN_CODE => Self::Code,
            sys::MD_SPANTYPE_MD_SPAN_DEL => Self::Del,
            sys::MD_SPANTYPE_MD_SPAN_LATEXMATH => Self::Latexmath,
            sys::MD_SPANTYPE_MD_SPAN_LATEXMATH_DISPLAY => {
                Self::LatexmathDisplay
            }
            sys::MD_SPANTYPE_MD_SPAN_WIKILINK => Self::Wikilink,
            sys::MD_SPANTYPE_MD_SPAN_U => Self::U,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MdTexttype {
    Normal,
    Nullchar,
    Br,
    Softbr,
    Entity,
    Code,
    Html,
    Latexmath,
}

impl From<sys::MD_TEXTTYPE> for MdTexttype {
    #[inline]
    fn from(raw_texttype: sys::MD_TEXTTYPE) -> Self {
        match raw_texttype {
            sys::MD_TEXTTYPE_MD_TEXT_NORMAL => Self::Normal,
            sys::MD_TEXTTYPE_MD_TEXT_NULLCHAR => Self::Nullchar,
            sys::MD_TEXTTYPE_MD_TEXT_BR => Self::Br,
            sys::MD_TEXTTYPE_MD_TEXT_SOFTBR => Self::Softbr,
            sys::MD_TEXTTYPE_MD_TEXT_ENTITY => Self::Entity,
            sys::MD_TEXTTYPE_MD_TEXT_CODE => Self::Code,
            sys::MD_TEXTTYPE_MD_TEXT_HTML => Self::Html,
            sys::MD_TEXTTYPE_MD_TEXT_LATEXMATH => Self::Latexmath,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MdAlign {
    Default,
    Left,
    Center,
    Right,
}

impl From<sys::MD_ALIGN> for MdAlign {
    #[inline]
    fn from(raw_align: sys::MD_ALIGN) -> Self {
        match raw_align {
            sys::MD_ALIGN_MD_ALIGN_DEFAULT => Self::Default,
            sys::MD_ALIGN_MD_ALIGN_LEFT => Self::Left,
            sys::MD_ALIGN_MD_ALIGN_CENTER => Self::Center,
            sys::MD_ALIGN_MD_ALIGN_RIGHT => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct MdAttribute(Box<sys::MD_ATTRIBUTE>);

pub struct MdAttributeIterator {
    raw_attribute: Box<sys::MD_ATTRIBUTE>,
    las: *const MdChar,
    i: isize,
}

impl MdAttribute {
    pub fn as_str(self) -> Option<&'static str> {
        let text = unsafe { (*self.0).text as *const u8 };
        if text.is_null() {
            return None;
        }
        let size = unsafe { (*self.0).size as usize };
        let res = slice_from_raw_parts(text, size);
        let res = unsafe { res.as_ref().unwrap_unchecked() };
        unsafe { Some(std::str::from_utf8_unchecked(res)) }
    }

    pub fn is_empty(&self) -> bool {
        let text = self.0.text as *const u8;
        text.is_null()
    }
}

impl Clone for MdAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl From<sys::MD_ATTRIBUTE> for MdAttribute {
    fn from(raw_attribute: sys::MD_ATTRIBUTE) -> Self {
        // let sl = slice_from_raw_parts(raw_attribute.text as *const u8, raw_attribute.size as usize);
        // let sl = unsafe { sl.as_ref().unwrap_unchecked() };
        // let sl = unsafe { std::str::from_utf8_unchecked(sl) };
        // println!("{sl}");
        Self(Box::new(raw_attribute))
    }
}

impl IntoIterator for MdAttribute {
    type Item = (&'static str, MdTexttype);
    type IntoIter = MdAttributeIterator;

    fn into_iter(self) -> Self::IntoIter {
        let las = unsafe { self.0.text };
        MdAttributeIterator {
            raw_attribute: self.0,
            las,
            i: 0,
        }
    }
}

impl Iterator for MdAttributeIterator {
    type Item = (&'static str, MdTexttype);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let size = (*self.raw_attribute).size;
            let text = (*self.raw_attribute).text;
            if text.is_null() {
                return None;
            }
            if self.las == text.offset(size as isize) {
                return None;
            }

            let tp =
                *((*self.raw_attribute).substr_types.offset(self.i));
            let tp = MdTexttype::from(tp);

            self.i += 1;

            let nl = *((*self.raw_attribute)
                .substr_offsets
                .offset(self.i));
            let nl = text.offset(nl as isize);
            let strlen = nl.offset_from(self.las) as usize;

            let s =
                slice_from_raw_parts(self.las as *const u8, strlen);
            let s = s.as_ref().unwrap_unchecked();
            let s = std::str::from_utf8_unchecked(s);

            self.las = nl;

            Some((s, tp))
        }
    }
}

#[test]
fn from_raw_attribute() {
    use sys::*;
    let text = CString::new("xfoo &quot; bar\x71\x7e").unwrap();
    let text = unsafe { text.as_ptr().offset(1) } as *const MD_CHAR;
    let size: MD_SIZE = 14;
    let substr_types: *const MD_TEXTTYPE = [
        MD_TEXTTYPE_MD_TEXT_NORMAL,
        MD_TEXTTYPE_MD_TEXT_ENTITY,
        MD_TEXTTYPE_MD_TEXT_NORMAL,
    ]
    .as_ptr();
    let substr_offsets: *const MD_OFFSET = [0, 4, 10, 14].as_ptr();
    let raw_attribute = MD_ATTRIBUTE {
        text,
        size,
        substr_types,
        substr_offsets,
    };
    let attribute = MdAttribute::from(raw_attribute);
    let s: &str = attribute.as_str().unwrap();

    assert_eq!(s, "foo &quot; bar");

    let attribute = MdAttribute::from(raw_attribute);
    let mut iter = attribute.into_iter();

    assert_eq!(iter.next(), Some(("foo ", MdTexttype::Normal)));
    assert_eq!(iter.next(), Some(("&quot;", MdTexttype::Entity)));
    assert_eq!(iter.next(), Some((" bar", MdTexttype::Normal)));
    assert_eq!(iter.next(), None);
}

#[test]
fn from_empty_attribute() {
    use sys::*;
    let text = 0 as *const i8;
    let size = 0 as MdSize;
    let substr_types = 0 as *const MD_TEXTTYPE;
    let substr_offsets: *const MD_OFFSET = [0].as_ptr();
    let raw_attribute = MD_ATTRIBUTE {
        text,
        size,
        substr_types,
        substr_offsets,
    };
    let attribute = MdAttribute::from(raw_attribute);
    assert_eq!(attribute.as_str(), None);

    let attribute = MdAttribute::from(raw_attribute);
    let mut iter = attribute.into_iter();

    assert_eq!(iter.next(), None);
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MdParserFlags(pub u32);

impl MdParserFlags {
    #[inline]
    pub fn new() -> Self {
        MdParserFlags(0)
    }

    /// Commonmark, currently with no flags
    #[inline]
    pub fn commonmark() -> Self {
        MdParserFlags(sys::MD_DIALECT_COMMONMARK)
    }

    /// Github favored Markdown, with four flags:
    /// 1. Permissive Autolinks: Recognize URLs, e-mails as autolinks even without '<', '>', and enable WWW autolinks (even without ant scheme prefix).
    /// 2. Tables: Enable tables extension.
    /// 3. Strike: Through: Enable ~~strikethrough~~ extension.
    /// 4. Tasklists: Enable task list extension.
    #[inline]
    pub fn github() -> Self {
        MdParserFlags(sys::MD_DIALECT_GITHUB)
    }

    /// In normal text(MD_TEXT_NORMAL), collapse non-trivial whitespace into single ' '
    pub fn collapse_whitespace(mut self) -> Self {
        self.0 |= sys::MD_FLAG_COLLAPSEWHITESPACE;
        self
    }

    /// Do not require space in ATX headers ( ###header )
    #[inline]
    pub fn permissive_atx_headers(mut self) -> Self {
        self.0 |= sys::MD_FLAG_PERMISSIVEATXHEADERS;
        self
    }

    /// Recognize URLs as autolinks even without '<', '>'
    #[inline]
    pub fn permissive_url_autolinks(mut self) -> Self {
        self.0 |= sys::MD_FLAG_PERMISSIVEURLAUTOLINKS;
        self
    }

    /// Recognize e-mails as autolinks even without '<', '>' and 'mailto:'
    #[inline]
    pub fn no_indented_codeblocks(mut self) -> Self {
        self.0 |= sys::MD_FLAG_NOINDENTEDCODEBLOCKS;
        self
    }

    /// Disable indented code blocks. (Only fenced code works.)
    #[inline]
    pub fn no_html_blocks(mut self) -> Self {
        self.0 |= sys::MD_FLAG_NOHTMLBLOCKS;
        self
    }

    /// Disable raw HTML blocks.
    #[inline]
    pub fn no_html_spans(mut self) -> Self {
        self.0 |= sys::MD_FLAG_NOHTMLSPANS;
        self
    }

    /// Enable tables extension.
    #[inline]
    pub fn tables(mut self) -> Self {
        self.0 |= sys::MD_FLAG_TABLES;
        self
    }

    /// Enable strikethrough extension.
    #[inline]
    pub fn strike_through(mut self) -> Self {
        self.0 |= sys::MD_FLAG_STRIKETHROUGH;
        self
    }

    /// Enable WWW autolinks (even without any scheme prefix, if they begin with 'www.')
    #[inline]
    pub fn permissive_www_autolinks(mut self) -> Self {
        self.0 |= sys::MD_FLAG_PERMISSIVEWWWAUTOLINKS;
        self
    }

    /// Enable task list extension.
    #[inline]
    pub fn permissive_tasklists(mut self) -> Self {
        self.0 |= sys::MD_FLAG_TASKLISTS;
        self
    }

    /// Enable $ and $$ containing LaTeX equations.
    #[inline]
    pub fn permissive_latexmath_spans(mut self) -> Self {
        self.0 |= sys::MD_FLAG_LATEXMATHSPANS;
        self
    }

    /// Enable wiki links extension.
    #[inline]
    pub fn permissive_wikilinks(mut self) -> Self {
        self.0 |= sys::MD_FLAG_WIKILINKS;
        self
    }

    /// Enable underline extension (and disables '_' for normal emphasis).
    #[inline]
    pub fn permissive_underline(mut self) -> Self {
        self.0 |= sys::MD_FLAG_UNDERLINE;
        self
    }

    /// Enable all autolinks, including URL, e-mail and WWW.
    #[inline]
    pub fn permissive_autolinks(mut self) -> Self {
        self.0 |= sys::MD_FLAG_PERMISSIVEAUTOLINKS;
        self
    }

    /// Disable all raw HTML, including span and block.
    #[inline]
    pub fn no_html(mut self) -> Self {
        self.0 |= sys::MD_FLAG_NOHTML;
        self
    }

    /// Remove flag(s).
    #[inline]
    pub fn remove(mut self, flag: &MdParserFlags) -> Self {
        self.0 &= !flag.0;
        self
    }
}

impl From<u32> for MdParserFlags {
    #[inline]
    fn from(flags: u32) -> Self {
        MdParserFlags(flags)
    }
}

unsafe extern "C" fn cb_enter_block<T: MdParser>(
    blocktype: sys::MD_BLOCKTYPE,
    detail: *mut c_void,
    userdata: *mut c_void,
) -> c_int {
    let userdata = userdata as *mut T;
    let userdata = userdata.as_mut().unwrap_unchecked();
    match userdata.enter_block(blocktype.into(), detail) {
        Ok(()) => 0,
        Err(errcode) => errcode.into(),
    }
}

unsafe extern "C" fn cb_leave_block<T: MdParser>(
    blocktype: sys::MD_BLOCKTYPE,
    detail: *mut c_void,
    userdata: *mut c_void,
) -> c_int {
    let userdata = userdata as *mut T;
    let userdata = userdata.as_mut().unwrap_unchecked();
    match userdata.leave_block(blocktype.into(), detail) {
        Ok(()) => 0,
        Err(errcode) => errcode.into(),
    }
}

unsafe extern "C" fn cb_enter_span<T: MdParser>(
    spantype: sys::MD_SPANTYPE,
    detail: *mut c_void,
    userdata: *mut c_void,
) -> c_int {
    let userdata = userdata as *mut T;
    let userdata = userdata.as_mut().unwrap_unchecked();
    match userdata.enter_span(spantype.into(), detail) {
        Ok(()) => 0,
        Err(errcode) => errcode.into(),
    }
}

unsafe extern "C" fn cb_leave_span<T: MdParser>(
    spantype: sys::MD_SPANTYPE,
    detail: *mut c_void,
    userdata: *mut c_void,
) -> c_int {
    let userdata = userdata as *mut T;
    let userdata = userdata.as_mut().unwrap_unchecked();
    match userdata.leave_span(spantype.into(), detail) {
        Ok(()) => 0,
        Err(errcode) => errcode.into(),
    }
}

unsafe extern "C" fn cb_text<T: MdParser>(
    texttype: sys::MD_TEXTTYPE,
    text: *const MdChar,
    size: MdSize,
    userdata: *mut c_void,
) -> c_int {
    let text = slice_from_raw_parts(text as *const u8, size as usize);
    let text = text.as_ref().unwrap_unchecked();
    let text = std::str::from_utf8_unchecked(text);
    let userdata = userdata as *mut T;
    let userdata = userdata.as_mut().unwrap_unchecked();
    match userdata.text(texttype.into(), text) {
        Ok(()) => 0,
        Err(errcode) => errcode.into(),
    }
}

unsafe extern "C" fn cb_debug_log<T: MdParser>(
    msg: *const c_char,
    userdata: *mut c_void,
) {
    let userdata = userdata as *mut T;
    let userdata = userdata.as_mut().unwrap_unchecked();
    let msg = CStr::from_ptr(msg);
    let msg = msg.to_str().unwrap_unchecked();
    userdata.debug_log(msg)
}

fn parse_work<T: MdParser>(
    parser: &mut T,
    src: &str,
    flags: &MdParserFlags,
    debug_log: Option<
        unsafe extern "C" fn(*const c_char, *mut c_void),
    >,
) -> MdResult {
    let abi_version = 0;
    let flags = flags.0;

    let syntax = None;

    let raw_parser = &mut sys::MD_PARSER {
        abi_version,
        flags,
        enter_block: Some(cb_enter_block::<T>),
        leave_block: Some(cb_leave_block::<T>),
        enter_span: Some(cb_enter_span::<T>),
        leave_span: Some(cb_leave_span::<T>),
        text: Some(cb_text::<T>),
        debug_log,
        syntax,
    };

    let size = src.len() as MdSize;
    let text = src.as_ptr() as *const MdChar;
    let userdata = parser as *mut T as *mut c_void;
    let res =
        unsafe { sys::md_parse(text, size, raw_parser, userdata) };
    if res == 0 {
        Ok(())
    } else {
        unsafe { Err(NonZeroI32::new_unchecked(res)) }
    }
}

pub trait MdParser:
    MdEnterBlockCallback
    + MdEnterSpanCallback
    + MdLeaveBlockCallback
    + MdLeaveSpanCallback
    + MdTextCallback
    + Sized
{
    fn debug_log(&mut self, msg: &str) {
        // Do nothing
    }

    fn parse(
        &mut self,
        src: &str,
        flags: &MdParserFlags,
    ) -> MdResult {
        parse_work(self, src, flags, None)
    }

    fn parse_with_logger(
        &mut self,
        src: &str,
        flags: &MdParserFlags,
    ) -> MdResult {
        parse_work(self, src, flags, Some(cb_debug_log::<Self>))
    }
}

pub mod prelude {
    pub use super::MdAlign;
    pub use super::MdAttribute;
    pub use super::MdChar;
    pub use super::MdResult;
    pub use super::MdSize;

    pub use super::MdBlocktype;
    pub use super::MdSpantype;
    pub use super::MdTexttype;

    pub use super::MdEnterBlockCallback;
    pub use super::MdEnterSpanCallback;
    pub use super::MdLeaveBlockCallback;
    pub use super::MdLeaveSpanCallback;
    pub use super::MdParser;
    pub use super::MdTextCallback;
}
