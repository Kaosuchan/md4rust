use md4rust::renderer::prelude::*;

#[derive(Default)]
struct MyData(String);

impl Md2HtmlRs for MyData {
    fn render_append(&mut self, text: &str) {
        self.0 += text;
    }
}

#[test]
fn renderer_rs() {
    let mut renderer = MdRenderer::html_rs(
        MyData::default(),
        Md2HtmlFlags::default(),
    );

    let input = "## Hello World ##\n";

    renderer.render(input, &MdParserFlags::github()).unwrap();

    // println!("{}", renderer.userdata().0);
    assert_eq!(renderer.userdata().0, "<h2>Hello World</h2>\n");
}
