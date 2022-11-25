use super::hex_val;
use super::Md2HtmlRenderer as Renderer;
use crate::md4c::*;

pub trait Md2HtmlUserdata: Sized {
    fn render_append(&mut self, text: &str);

    #[inline]
    fn render_verbatim(&mut self, data: &[u8]) {
        let text = unsafe { std::str::from_utf8_unchecked(data) };
        self.render_append(text);
    }

    // enter block

    fn enter_doc(renderer: &mut Renderer<Self>) -> MdResult {
        // noop
        Ok(())
    }

    fn enter_quote(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("<blockquote>\n");
        Ok(())
    }

    fn enter_ul(
        renderer: &mut Renderer<Self>,
        is_tight: bool,
        mark: MdChar,
    ) -> MdResult {
        renderer.render_append("<ul>\n");
        Ok(())
    }

    fn enter_ol(
        renderer: &mut Renderer<Self>,
        start: u32,
        is_tight: bool,
        mark_delimiter: MdChar,
    ) -> MdResult {
        if start == 1 {
            renderer.render_append("<ol>\n");
        } else {
            renderer
                .render_append(&format!("<ol start=\"{}\">\n", start));
        }
        Ok(())
    }

    fn enter_li(
        renderer: &mut Renderer<Self>,
        is_task: bool,
        task_mark: MdChar,
        task_mark_offset: MdOffset,
    ) -> MdResult {
        if is_task {
            renderer.render_append("<li class=\"task-list-item\"><input type=\"checkbox\" class=\"task-list-item-checkbox\" disabled");
            if task_mark as u8 == b'x' || task_mark as u8 == b'X' {
                renderer.render_append(" checked");
            }
            renderer.render_append(">");
        } else {
            renderer.render_append("<li>");
        }
        Ok(())
    }

    fn enter_hr(renderer: &mut Renderer<Self>) -> MdResult {
        if renderer.flags.has_xhtml() {
            renderer.render_append("<hr />\n");
        } else {
            renderer.render_append("<hr>\n")
        }
        Ok(())
    }

    fn enter_h(
        renderer: &mut Renderer<Self>,
        level: u32,
    ) -> MdResult {
        static HEAD: &[&str; 6] =
            &["<h1>", "<h2>", "<h3>", "<h4>", "<h5>", "<h6>"];
        renderer.render_append(HEAD[level as usize - 1]);
        Ok(())
    }

    fn enter_code(
        renderer: &mut Renderer<Self>,
        info: MdAttribute,
        lang: MdAttribute,
        fence_char: MdChar,
    ) -> MdResult {
        renderer.render_append("<pre><code");

        if !lang.is_empty() {
            renderer.render_append(" class=\"language-");
            renderer.render_attribute(
                lang,
                Renderer::render_html_escaped,
            );
            renderer.render_append("\"");
        }

        renderer.render_append(">");
        Ok(())
    }

    fn enter_html(renderer: &mut Renderer<Self>) -> MdResult {
        // noop
        Ok(())
    }

    fn enter_p(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("<p>");
        Ok(())
    }

    fn enter_table(
        renderer: &mut Renderer<Self>,
        col_count: u32,
        head_row_count: u32,
        body_row_count: u32,
    ) -> MdResult {
        renderer.render_append("<table>\n");
        Ok(())
    }

    fn enter_thead(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("<thead>\n");
        Ok(())
    }

    fn enter_tbody(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("<tbody>\n");
        Ok(())
    }

    fn enter_tr(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("<tr>\n");
        Ok(())
    }

    fn enter_th(
        renderer: &mut Renderer<Self>,
        align: MdAlign,
    ) -> MdResult {
        renderer.render_append("<th");

        match align {
            MdAlign::Left => {
                renderer.render_append(" align=\"left\">")
            }
            MdAlign::Center => {
                renderer.render_append(" align=\"center\">")
            }
            MdAlign::Right => {
                renderer.render_append(" align=\"right\">")
            }
            MdAlign::Default => renderer.render_append(">"),
        }

        Ok(())
    }

    fn enter_td(
        renderer: &mut Renderer<Self>,
        align: MdAlign,
    ) -> MdResult {
        renderer.render_append("<td");

        match align {
            MdAlign::Left => {
                renderer.render_append(" align=\"left\">")
            }
            MdAlign::Center => {
                renderer.render_append(" align=\"center\">")
            }
            MdAlign::Right => {
                renderer.render_append(" align=\"right\">")
            }
            MdAlign::Default => renderer.render_append(">"),
        }

        Ok(())
    }

    // leave block

    fn leave_doc(renderer: &mut Renderer<Self>) -> MdResult {
        // noop
        Ok(())
    }

    fn leave_quote(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("</blockquote>\n");
        Ok(())
    }

    fn leave_ul(
        renderer: &mut Renderer<Self>,
        is_tight: bool,
        mark: MdChar,
    ) -> MdResult {
        renderer.render_append("</ul>\n");
        Ok(())
    }

    fn leave_ol(
        renderer: &mut Renderer<Self>,
        start: u32,
        is_tight: bool,
        mark_delimiter: MdChar,
    ) -> MdResult {
        renderer.render_append("</ol>\n");
        Ok(())
    }

    fn leave_li(
        renderer: &mut Renderer<Self>,
        is_task: bool,
        task_mark: MdChar,
        task_mark_offset: MdOffset,
    ) -> MdResult {
        renderer.render_append("</li>\n");
        Ok(())
    }

    fn leave_hr(renderer: &mut Renderer<Self>) -> MdResult {
        // noop
        Ok(())
    }

    fn leave_h(
        renderer: &mut Renderer<Self>,
        level: u32,
    ) -> MdResult {
        static HEAD: &[&str; 6] = &[
            "</h1>\n", "</h2>\n", "</h3>\n", "</h4>\n", "</h5>\n",
            "</h6>\n",
        ];
        renderer.render_append(HEAD[(level as usize) - 1]);
        Ok(())
    }

    fn leave_code(
        renderer: &mut Renderer<Self>,
        info: MdAttribute,
        lang: MdAttribute,
        fence_char: MdChar,
    ) -> MdResult {
        renderer.render_append("</code></pre>\n");
        Ok(())
    }

    fn leave_html(renderer: &mut Renderer<Self>) -> MdResult {
        // noop
        Ok(())
    }

    fn leave_p(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("</p>\n");
        Ok(())
    }

    fn leave_table(
        renderer: &mut Renderer<Self>,
        col_count: u32,
        head_row_count: u32,
        body_row_count: u32,
    ) -> MdResult {
        renderer.render_append("</table>\n");
        Ok(())
    }

    fn leave_thead(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("</thead>\n");
        Ok(())
    }

    fn leave_tbody(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("</tbody>\n");
        Ok(())
    }

    fn leave_tr(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("</tr>\n");
        Ok(())
    }

    fn leave_th(
        renderer: &mut Renderer<Self>,
        align: MdAlign,
    ) -> MdResult {
        renderer.render_append("</th>\n");
        Ok(())
    }

    fn leave_td(
        renderer: &mut Renderer<Self>,
        align: MdAlign,
    ) -> MdResult {
        renderer.render_append("</td>\n");
        Ok(())
    }

    // enter span

    fn enter_em(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("<em>");
        Ok(())
    }

    fn enter_strong(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("<strong>");
        Ok(())
    }

    fn enter_a(
        renderer: &mut Renderer<Self>,
        href: MdAttribute,
        title: MdAttribute,
    ) -> MdResult {
        renderer.render_append("<a href=\"");
        renderer.render_attribute(href, Renderer::render_url_escaped);

        if !title.is_empty() {
            renderer.render_append("\" title=\"");
            renderer.render_attribute(
                title,
                Renderer::render_html_escaped,
            );
        }

        renderer.render_append("\">");

        Ok(())
    }

    fn enter_img(
        renderer: &mut Renderer<Self>,
        src: MdAttribute,
        title: MdAttribute,
    ) -> MdResult {
        renderer.render_append("<img src=\"");

        renderer.render_attribute(src, Renderer::render_url_escaped);

        renderer.render_append("\" alt=\"");
        Ok(())
    }

    fn enter_ilcode(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("<code>");
        Ok(())
    }

    fn enter_del(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("<del>");
        Ok(())
    }

    fn enter_latexmath(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("<x-equation>");
        Ok(())
    }

    fn enter_latexmath_display(
        renderer: &mut Renderer<Self>,
    ) -> MdResult {
        renderer.render_append("<x-equation type=\"display\">");
        Ok(())
    }

    fn enter_wikilink(
        renderer: &mut Renderer<Self>,
        target: MdAttribute,
    ) -> MdResult {
        renderer.render_append("<x-wikilink data-target=\"");
        renderer
            .render_attribute(target, Renderer::render_html_escaped);

        renderer.render_append("\">");
        Ok(())
    }

    fn enter_u(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("<u>");
        Ok(())
    }

    fn leave_em(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("</em>");
        Ok(())
    }

    fn leave_strong(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("</strong>");
        Ok(())
    }

    fn leave_a(
        renderer: &mut Renderer<Self>,
        href: MdAttribute,
        title: MdAttribute,
    ) -> MdResult {
        renderer.render_append("</a>");
        Ok(())
    }

    fn leave_img(
        renderer: &mut Renderer<Self>,
        src: MdAttribute,
        title: MdAttribute,
    ) -> MdResult {
        if !title.is_empty() {
            renderer.render_append("\" title=\"");
            renderer.render_attribute(
                title,
                Renderer::render_html_escaped,
            );
        }

        renderer.render_append(if renderer.flags.has_xhtml() {
            "\" />"
        } else {
            "\">"
        });

        Ok(())
    }

    fn leave_ilcode(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("</code>");
        Ok(())
    }

    fn leave_del(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("</del>");
        Ok(())
    }

    fn leave_latexmath(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("</x-equation>");
        Ok(())
    }

    fn leave_latexmath_display(
        renderer: &mut Renderer<Self>,
    ) -> MdResult {
        renderer.render_append("</x-equation>");
        Ok(())
    }

    fn leave_wikilink(
        renderer: &mut Renderer<Self>,
        target: MdAttribute,
    ) -> MdResult {
        renderer.render_append("</x-wikilink>");
        Ok(())
    }

    fn leave_u(renderer: &mut Renderer<Self>) -> MdResult {
        renderer.render_append("</u>");
        Ok(())
    }

    fn text_normal(
        renderer: &mut Renderer<Self>,
        text: &str,
    ) -> MdResult {
        renderer.render_html_escaped(text.as_bytes());
        Ok(())
    }

    fn text_nullchar(
        renderer: &mut Renderer<Self>,
        text: &str,
    ) -> MdResult {
        renderer
            .render_utf8_codepoint(0x0000, Renderer::render_verbatim);
        Ok(())
    }

    fn text_br(
        renderer: &mut Renderer<Self>,
        text: &str,
    ) -> MdResult {
        renderer.render_append(
            if renderer.image_nesting_level == 0 {
                if renderer.flags.has_xhtml() {
                    "<br />\n"
                } else {
                    "<br>\n"
                }
            } else {
                " "
            },
        );
        Ok(())
    }

    fn text_softbr(
        renderer: &mut Renderer<Self>,
        text: &str,
    ) -> MdResult {
        renderer.render_append(
            if renderer.image_nesting_level == 0 {
                "\n"
            } else {
                " "
            },
        );
        Ok(())
    }

    fn text_entity(
        renderer: &mut Renderer<Self>,
        text: &str,
    ) -> MdResult {
        renderer.render_entity(
            text.as_bytes(),
            Renderer::render_html_escaped,
        );
        Ok(())
    }

    fn text_code(
        renderer: &mut Renderer<Self>,
        text: &str,
    ) -> MdResult {
        renderer.text_normal(text)
    }

    fn text_html(
        renderer: &mut Renderer<Self>,
        text: &str,
    ) -> MdResult {
        renderer.render_append(text);
        Ok(())
    }

    fn text_latexmath(
        renderer: &mut Renderer<Self>,
        text: &str,
    ) -> MdResult {
        renderer.text_normal(text)
    }

    fn debug_log(renderer: &mut Renderer<Self>, msg: &str) {
        // Do nothing
    }
}
