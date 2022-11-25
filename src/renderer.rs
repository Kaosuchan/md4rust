pub use crate::md4c::{MdParserFlags, MdResult};
use std::marker::PhantomData;

pub mod md2htmlflags;
pub mod md_html_c;
pub mod md_html_rs;

pub trait SpecifiedRenderer {
    type Userdata;
    fn render(
        &mut self,
        input: &str,
        parser_flags: &MdParserFlags,
    ) -> MdResult;

    fn userdata(&mut self) -> &mut Self::Userdata;
}

pub struct Renderer<Ud: Sized, I: SpecifiedRenderer<Userdata = Ud>>(
    pub I,
    PhantomData<Ud>,
);

impl<Ud, I> Renderer<Ud, I>
where
    Ud: Sized,
    I: SpecifiedRenderer<Userdata = Ud>,
{
    #[inline]
    pub fn render(
        &mut self,
        input: &str,
        parser_flags: &MdParserFlags,
    ) -> MdResult {
        self.0.render(input, parser_flags)
    }

    #[inline]
    pub fn userdata(&mut self) -> &mut Ud {
        self.0.userdata()
    }
}

pub mod prelude {
    pub use super::md2htmlflags::Md2HtmlFlags;
    pub use super::md_html_c::Md2HtmlUserdata as Md2HtmlC;
    pub use super::md_html_rs::Md2HtmlUserdata as Md2HtmlRs;
    pub use super::Renderer as MdRenderer;
    pub use crate::md4c::{MdParserFlags, MdResult};
}
