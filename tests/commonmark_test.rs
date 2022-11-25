use md4rust::renderer::prelude::*;
use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize)]
struct Test {
    markdown: String,
    html: String,
    example: u32,
}

#[derive(Deserialize)]
struct Tests(Vec<Test>);

fn commonmark_test_rs() -> Vec<u32>{
    #[derive(Default)]
    struct MyRenderer(String);

    impl Md2HtmlRs for MyRenderer {
        fn render_append(&mut self, text: &str) {
            self.0 += text;
        }
    }

    let mut renderer = MdRenderer::html_rs(
        MyRenderer::default(),
        Md2HtmlFlags::default().skip_utf8_bom().xhtml(),
    );

    let parser_flags = MdParserFlags::commonmark();

    let test_file = File::open("./commonmark0.30.json").unwrap();
    let tests: Tests = serde_json::from_reader(test_file).unwrap();
    let mut fails = Vec::<u32>::new();
    for test in tests.0.iter() {
        renderer.userdata().0 = String::new();
        renderer.render(&test.markdown, &parser_flags).unwrap();
        if &renderer.userdata().0 != &test.html {
            fails.extend([test.example].iter());
        }
    }
    // println!("Fails: {:?}", fails);
    fails
}

fn commonmark_test_c() -> Vec<u32>{
    #[derive(Default)]
    struct MyRenderer(String);

    impl Md2HtmlC for MyRenderer {
        fn render_append(&mut self, text: &str) {
            self.0 += text;
        }
    }

    let mut renderer = MdRenderer::html_c(
        MyRenderer::default(),
        Md2HtmlFlags::default().skip_utf8_bom().xhtml(),
    );

    let parser_flags = MdParserFlags::commonmark();

    let test_file = File::open("./commonmark0.30.json").unwrap();
    let tests: Tests = serde_json::from_reader(test_file).unwrap();
    let mut fails = Vec::<u32>::new();
    for test in tests.0.iter() {
        // println!("Testing example {}", test.example);
        // if test.example != 573 { continue; }
        renderer.userdata().0 = String::new();
        renderer.render(&test.markdown, &parser_flags).unwrap();
        if &renderer.userdata().0 != &test.html {
            fails.extend([test.example].iter());
        }
    }
    fails
}

#[test]
fn compare() {
    let fail_c = commonmark_test_c();
    let fail_rs = commonmark_test_rs();
    // let p = std::cmp::min(fail_c.len(), fail_rs.len());
    // for i in 0..p {
    //     assert_eq!(fail_c[i], fail_rs[i]);
    // }
    assert_eq!(fail_c, fail_rs);
}

