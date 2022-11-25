use crate::renderer::prelude::*;

pub use crate::md4c::MdParserFlags;

pub fn ootb_md(src: &str, flags: &MdParserFlags) -> String {
    #[derive(Default)]
    struct MyRenderer(String);
    impl Md2HtmlC for MyRenderer {
        fn render_append(&mut self, text: &str) {
            self.0 += text;
        }
    }
    let mut renderer = MdRenderer::html_c(
        MyRenderer::default(),
        Md2HtmlFlags::new().verbatim_entities().skip_utf8_bom(),
    );
    renderer.render(src, flags).unwrap();
    renderer.userdata().0.to_owned()
}

pub fn github_md(src: &str) -> String {
    ootb_md(src, &MdParserFlags::github())
}

pub fn commonmark(src: &str) -> String {
    ootb_md(src, &MdParserFlags::commonmark())
}
