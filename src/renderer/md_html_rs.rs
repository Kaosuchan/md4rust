use std::marker::PhantomData;

use super::md2htmlflags::Md2HtmlFlags;
use super::{Renderer, SpecifiedRenderer};
use crate::md4c::*;

mod userdata;
pub use userdata::Md2HtmlUserdata;

pub use crate::md4c::{MdParserFlags, MdResult};

mod escape_map;
use escape_map::{need_html_esc, need_url_esc};

mod utils;
use utils::*;

mod autoimpl;

pub struct Md2HtmlRenderer<T> {
    pub image_nesting_level: usize,
    pub flags: Md2HtmlFlags,
    pub userdata: T,
}

#[allow(clippy::needless_range_loop)]
#[allow(clippy::identity_op)]
impl<T> Md2HtmlRenderer<T>
where
    T: Md2HtmlUserdata,
{
    #[inline]
    fn render_append(&mut self, text: &str) {
        self.userdata.render_append(text)
    }

    #[inline]
    fn render_verbatim(&mut self, data: &[u8]) {
        self.userdata.render_verbatim(data)
    }

    fn render_html_escaped(&mut self, data: &[u8]) {
        let mut beg: usize = 0;
        let mut off: usize = 0;
        let size = data.len();

        loop {
            while off + 3 < size
                && !need_html_esc(data[off + 0])
                && !need_html_esc(data[off + 1])
                && !need_html_esc(data[off + 2])
                && !need_html_esc(data[off + 3])
            {
                off += 4;
            }

            while off < size && !need_html_esc(data[off]) {
                off += 1;
            }

            if off > beg {
                self.render_verbatim(&data[beg..off]);
            }

            if off < size {
                match data[off] {
                    b'&' => self.render_verbatim(b"&amp;"),
                    b'<' => self.render_verbatim(b"&lt;"),
                    b'>' => self.render_verbatim(b"&gt;"),
                    b'"' => self.render_verbatim(b"&quot;"),
                    _ => (), /* noop */
                }
                off += 1;
            } else {
                break;
            }

            beg = off;
        }
    }

    fn render_url_escaped(&mut self, data: &[u8]) {
        static HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";

        let mut beg: usize = 0;
        let mut off: usize = 0;
        let size = data.len();

        loop {
            while off < size && !need_url_esc(data[off]) {
                off += 1;
            }

            if off > beg {
                self.render_verbatim(&data[beg..off]);
            }

            if off < size {
                match data[off] {
                    b'&' => self.render_verbatim(b"&amp;"),
                    _ => self.render_verbatim(&[
                        b'%',
                        HEX_CHARS[((data[off] as usize) >> 4) & 0xf],
                        HEX_CHARS[((data[off] as usize) >> 0) & 0xf],
                    ]),
                }
                off += 1;
            } else {
                break;
            }

            beg = off;
        }
    }

    fn render_utf8_codepoint(
        &mut self,
        codepoint: u32,
        fn_append: fn(&mut Self, &[u8]),
    ) {
        static UTF8_REPLACEMENR_CHAR: &[u8; 3] = &[0xef, 0xbf, 0xbd];

        if codepoint == 0 || codepoint > 0x10ffff {
            fn_append(self, UTF8_REPLACEMENR_CHAR);
            return ;
        }

        if codepoint <= 0x7f {
            fn_append(self, &[codepoint as u8])
        } else if codepoint <= 0x7ff {
            fn_append(
                self,
                &[
                    0xc0 | ((codepoint >> 6) & 0x1f) as u8,
                    0x80 + ((codepoint >> 0) & 0x3f) as u8,
                ],
            )
        } else if codepoint <= 0xffff {
            fn_append(
                self,
                &[
                    0xe0 | ((codepoint >> 12) & 0x0f) as u8,
                    0x80 + ((codepoint >> 6) & 0x3f) as u8,
                    0x80 + ((codepoint >> 0) & 0x3f) as u8,
                ],
            )
        } else {
            fn_append(
                self,
                &[
                    0xf0 | ((codepoint >> 18) & 0x07) as u8,
                    0x80 + ((codepoint >> 12) & 0x3f) as u8,
                    0x80 + ((codepoint >> 6) & 0x3f) as u8,
                    0x80 + ((codepoint >> 0) & 0x3f) as u8,
                ],
            )
        }
    }

    fn render_entity(
        &mut self,
        text: &[u8],
        fn_append: fn(&mut Self, &[u8]),
    ) {
        if self.flags.has_verbatim_entities() {
            self.render_verbatim(text);
            return;
        }

        let size = text.len();

        if size > 3 && text[1] == b'#' {
            let mut codepoint: u32 = 0;

            if text[2] == b'x' || text[2] == b'X' {
                for i in 3..size - 1 {
                    codepoint =
                        16 * codepoint + hex_val(text[i]) as u32;
                }
            } else {
                for i in 2..size - 1 {
                    codepoint =
                        10 * codepoint + (text[i] - b'0') as u32;
                }
            }

            self.render_utf8_codepoint(codepoint, fn_append);
            return;
        } else {
            use crate::entity::entity_lookup_raw;
            if let Some(ent) = entity_lookup_raw(text) {
                for codepoint in ent {
                    self.render_utf8_codepoint(
                        codepoint.to_owned(),
                        fn_append,
                    );
                }
                return;
            }
        }

        fn_append(self, text);
    }

    fn render_attribute(
        &mut self,
        attribute: MdAttribute,
        fn_append: fn(&mut Self, &[u8]),
    ) {
        for (s, t) in attribute.into_iter() {
            let text = s.as_bytes();
            match t {
                MdTexttype::Nullchar => {
                    self.render_utf8_codepoint(0x0000, fn_append)
                }
                MdTexttype::Entity => {
                    self.render_entity(text, fn_append)
                }
                _ => fn_append(self, text),
            }
        }
    }
}

impl<T> SpecifiedRenderer for Md2HtmlRenderer<T>
where
    T: Md2HtmlUserdata,
{
    type Userdata = T;

    fn render(
        &mut self,
        mut input: &str,
        parser_flags: &MdParserFlags,
    ) -> MdResult {
        self.render(input, parser_flags)
    }

    fn userdata(&mut self) -> &mut Self::Userdata {
        &mut self.userdata
    }
}

impl<T> Renderer<T, Md2HtmlRenderer<T>>
where
    T: Md2HtmlUserdata,
{
    pub fn html_rs(userdata: T, flags: Md2HtmlFlags) -> Self {
        Self(
            Md2HtmlRenderer {
                image_nesting_level: 0,
                flags,
                userdata,
            },
            PhantomData,
        )
    }

    pub fn render_with_logger(
        &mut self,
        mut input: &str,
        parser_flags: &MdParserFlags,
    ) -> MdResult {
        self.0.render_with_logger(input, parser_flags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = "<a href=\"/bar\\/)\">\n";
        #[derive(Default)]
        struct MyRenderer(String);
        impl Md2HtmlUserdata for MyRenderer {
            fn render_append(&mut self, text: &str) {
                self.0 += text;
            }
        }

        let mut renderer = Md2HtmlRenderer::new(
            MyRenderer::default(),
            Md2HtmlFlags::new()
        );

        renderer.render(input, &MdParserFlags::commonmark());

        println!("{}", renderer.userdata.0);
    }
}