mod sys {
    pub use crate::md4c_sys::md4c_html::*;
}

#[derive(Clone, Copy, Debug)]
pub struct Md2HtmlFlags(pub u32);

impl Md2HtmlFlags {
    const DEBUG: u32 = sys::MD_HTML_FLAG_DEBUG;
    const VERBATIM_ENTITIES: u32 =
        sys::MD_HTML_FLAG_VERBATIM_ENTITIES;
    const SKIP_UTF8_BOM: u32 = sys::MD_HTML_FLAG_SKIP_UTF8_BOM;
    const XHTML: u32 = sys::MD_HTML_FLAG_XHTML;

    #[inline]
    pub fn new() -> Self {
        Md2HtmlFlags(0)
    }

    #[inline]
    pub fn debug(mut self) -> Self {
        self.0 |= Self::DEBUG;
        self
    }

    #[inline]
    pub fn has_debug(self) -> bool {
        self.0 & Self::DEBUG != 0
    }

    #[inline]
    pub fn verbatim_entities(mut self) -> Self {
        self.0 |= Self::VERBATIM_ENTITIES;
        self
    }

    #[inline]
    pub fn has_verbatim_entities(self) -> bool {
        self.0 & Self::VERBATIM_ENTITIES != 0
    }

    #[inline]
    pub fn skip_utf8_bom(mut self) -> Self {
        self.0 |= Self::SKIP_UTF8_BOM;
        self
    }

    #[inline]
    pub fn has_skip_utf8_bom(self) -> bool {
        self.0 & Self::SKIP_UTF8_BOM != 0
    }

    #[inline]
    pub fn xhtml(mut self) -> Self {
        self.0 |= Self::XHTML;
        self
    }

    #[inline]
    pub fn has_xhtml(self) -> bool {
        self.0 & Self::XHTML != 0
    }
}

impl Default for Md2HtmlFlags {
    fn default() -> Self {
        Self::new()
    }
}
