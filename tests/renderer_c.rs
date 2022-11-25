use md4rust::renderer::prelude::*;

#[derive(Default)]
struct MyRenderer(String);

impl Md2HtmlC for MyRenderer {
    fn render_append(&mut self, text: &str) {
        self.0 += text;
    }
}

#[test]
fn renderer_c() -> MdResult {
    let mut renderer = MdRenderer::html_c(
        MyRenderer::default(),
        Md2HtmlFlags::default(),
    );

    renderer.render(
        r#"
```javascript Hello World
```
"#,
        &MdParserFlags::github(),
    )?;

    assert_eq!(
        renderer.userdata().0,
        "<pre><code class=\"language-javascript\"></code></pre>\n"
    );

    Ok(())
}
