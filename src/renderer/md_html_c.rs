use super::{
    md2htmlflags::Md2HtmlFlags, MdResult, Renderer, SpecifiedRenderer,
};
use std::{
    marker::PhantomData, num::NonZeroI32, os::raw::*,
    ptr::slice_from_raw_parts,
};

mod sys {
    pub use crate::md4c_sys::md4c_html::*;
}

use crate::md4c::MdParserFlags;

use self::sys::md_html;

unsafe extern "C" fn cb_process_output<T: Md2HtmlUserdata>(
    text: *const sys::MD_CHAR,
    length: sys::MD_SIZE,
    userdata: *mut c_void,
) {
    let renderer = userdata as *mut Md2HtmlRenderer<T>;
    let userdata = &mut renderer.as_mut().unwrap_unchecked().userdata;
    let text = text as *const u8;
    let text = slice_from_raw_parts(text, length as usize);
    let text = text.as_ref().unwrap_unchecked();
    let text = std::str::from_utf8_unchecked(text);
    userdata.render_append(text);
}

pub trait Md2HtmlUserdata: Sized {
    fn render_append(&mut self, text: &str);
}

pub struct Md2HtmlRenderer<T> {
    renderer_flags: Md2HtmlFlags,
    pub userdata: T,
}

impl<T> Md2HtmlRenderer<T>
where
    T: Md2HtmlUserdata,
{
    pub fn new(userdata: T, renderer_flags: Md2HtmlFlags) -> Self {
        Self {
            renderer_flags,
            userdata,
        }
    }

    pub fn render(
        &mut self,
        input: &str,
        parser_flags: &MdParserFlags,
    ) -> MdResult {
        let input_size = input.len() as sys::MD_SIZE;
        let input = input.as_ptr() as *const sys::MD_CHAR;
        let userdata = self as *mut Self as *mut c_void;
        let parser_flags = parser_flags.0;
        let renderer_flags = self.renderer_flags.0;

        let res;

        unsafe {
            res = sys::md_html(
                input,
                input_size,
                Some(cb_process_output::<T>),
                userdata,
                parser_flags,
                renderer_flags,
            )
        };

        if (res == 0) {
            Ok(())
        } else {
            unsafe { Err(NonZeroI32::new_unchecked(res)) }
        }
    }
}

impl<T> SpecifiedRenderer for Md2HtmlRenderer<T>
where
    T: Md2HtmlUserdata,
{
    type Userdata = T;

    #[inline]
    fn render(
        &mut self,
        input: &str,
        parser_flags: &MdParserFlags,
    ) -> MdResult {
        self.render(input, parser_flags)
    }

    #[inline]
    fn userdata(&mut self) -> &mut Self::Userdata {
        &mut self.userdata
    }

    #[inline]
    fn unwrap(self) -> Self::Userdata {
        self.userdata
    }
}

impl<T> Renderer<T, Md2HtmlRenderer<T>>
where
    T: Md2HtmlUserdata,
{
    pub fn html_c(userdata: T, renderer_flags: Md2HtmlFlags) -> Self {
        Self(
            Md2HtmlRenderer {
                renderer_flags,
                userdata,
            },
            PhantomData,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    struct MyRenderer {
        result: String,
    }

    impl Md2HtmlUserdata for MyRenderer {
        fn render_append(&mut self, text: &str) {
            self.result += text;
        }
    }

    #[test]
    fn test1() {
        let mut myrenderer = MyRenderer {
            result: "".to_owned(),
        };
        let parser_flags = MdParserFlags::github();
        let mut renderer =
            Md2HtmlRenderer::new(myrenderer, Md2HtmlFlags::new());

        renderer.render(
            r#"
### aaa ###
~~del~~Yayayay
```cpp
#include<iostream>

int main() {
    int a, b;
    std::cin >> a >> b;
    std::cout << a + b << std::endl;
    return 0;
}
```
        "#,
            &parser_flags,
        );

        assert_eq!(
            &(renderer.userdata.result),
            r#"<h3>aaa</h3>
<p><del>del</del>Yayayay</p>
<pre><code class="language-cpp">#include&lt;iostream&gt;

int main() {
    int a, b;
    std::cin &gt;&gt; a &gt;&gt; b;
    std::cout &lt;&lt; a + b &lt;&lt; std::endl;
    return 0;
}
</code></pre>
"#
        );
    }
}
