use super::*;

impl<T> MdEnterBlockCallback for Md2HtmlRenderer<T>
where
    T: Md2HtmlUserdata,
{
    fn enter_doc(&mut self) -> MdResult {
        T::enter_doc(self)
    }

    fn enter_quote(&mut self) -> MdResult {
        T::enter_quote(self)
    }

    fn enter_ul(&mut self, is_tight: bool, mark: MdChar) -> MdResult {
        T::enter_ul(self, is_tight, mark)
    }

    fn enter_ol(
        &mut self,
        start: u32,
        is_tight: bool,
        mark_delimiter: MdChar,
    ) -> MdResult {
        T::enter_ol(self, start, is_tight, mark_delimiter)
    }

    fn enter_li(
        &mut self,
        is_task: bool,
        task_mark: MdChar,
        task_mark_offset: MdOffset,
    ) -> MdResult {
        T::enter_li(self, is_task, task_mark, task_mark_offset)
    }

    fn enter_hr(&mut self) -> MdResult {
        T::enter_hr(self)
    }

    fn enter_h(&mut self, level: u32) -> MdResult {
        T::enter_h(self, level)
    }

    fn enter_code(
        &mut self,
        info: MdAttribute,
        lang: MdAttribute,
        fence_char: MdChar,
    ) -> MdResult {
        T::enter_code(self, info, lang, fence_char)
    }

    fn enter_html(&mut self) -> MdResult {
        T::enter_html(self)
    }

    fn enter_p(&mut self) -> MdResult {
        T::enter_p(self)
    }

    fn enter_table(
        &mut self,
        col_count: u32,
        head_row_count: u32,
        body_row_count: u32,
    ) -> MdResult {
        T::enter_table(
            self,
            col_count,
            head_row_count,
            body_row_count,
        )
    }

    fn enter_thead(&mut self) -> MdResult {
        T::enter_thead(self)
    }

    fn enter_tbody(&mut self) -> MdResult {
        T::enter_tbody(self)
    }

    fn enter_tr(&mut self) -> MdResult {
        T::enter_tr(self)
    }

    fn enter_th(&mut self, align: MdAlign) -> MdResult {
        T::enter_th(self, align)
    }

    fn enter_td(&mut self, align: MdAlign) -> MdResult {
        T::enter_td(self, align)
    }
}

impl<T> MdLeaveBlockCallback for Md2HtmlRenderer<T>
where
    T: Md2HtmlUserdata,
{
    fn leave_doc(&mut self) -> MdResult {
        T::leave_doc(self)
    }

    fn leave_quote(&mut self) -> MdResult {
        T::leave_quote(self)
    }

    fn leave_ul(&mut self, is_tight: bool, mark: MdChar) -> MdResult {
        T::leave_ul(self, is_tight, mark)
    }

    fn leave_ol(
        &mut self,
        start: u32,
        is_tight: bool,
        mark_delimiter: MdChar,
    ) -> MdResult {
        T::leave_ol(self, start, is_tight, mark_delimiter)
    }

    fn leave_li(
        &mut self,
        is_task: bool,
        task_mark: MdChar,
        task_mark_offset: MdOffset,
    ) -> MdResult {
        T::leave_li(self, is_task, task_mark, task_mark_offset)
    }

    fn leave_hr(&mut self) -> MdResult {
        T::leave_hr(self)
    }

    fn leave_h(&mut self, level: u32) -> MdResult {
        T::leave_h(self, level)
    }

    fn leave_code(
        &mut self,
        info: MdAttribute,
        lang: MdAttribute,
        fence_char: MdChar,
    ) -> MdResult {
        T::leave_code(self, info, lang, fence_char)
    }

    fn leave_html(&mut self) -> MdResult {
        T::leave_html(self)
    }

    fn leave_p(&mut self) -> MdResult {
        T::leave_p(self)
    }

    fn leave_table(
        &mut self,
        col_count: u32,
        head_row_count: u32,
        body_row_count: u32,
    ) -> MdResult {
        T::leave_table(
            self,
            col_count,
            head_row_count,
            body_row_count,
        )
    }

    fn leave_thead(&mut self) -> MdResult {
        T::leave_thead(self)
    }

    fn leave_tbody(&mut self) -> MdResult {
        T::leave_tbody(self)
    }

    fn leave_tr(&mut self) -> MdResult {
        T::leave_tr(self)
    }

    fn leave_th(&mut self, align: MdAlign) -> MdResult {
        T::leave_th(self, align)
    }

    fn leave_td(&mut self, align: MdAlign) -> MdResult {
        T::leave_td(self, align)
    }
}

impl<T> MdEnterSpanCallback for Md2HtmlRenderer<T>
where
    T: Md2HtmlUserdata,
{
    fn enter_span(
        &mut self,
        spantype: MdSpantype,
        detail: *mut std::os::raw::c_void,
    ) -> MdResult {
        if self.image_nesting_level > 0 {
            Ok(())
        } else {
            self.enter_span_fork(spantype, detail)
        }
    }

    fn enter_em(&mut self) -> MdResult {
        T::enter_em(self)
    }

    fn enter_strong(&mut self) -> MdResult {
        T::enter_strong(self)
    }

    fn enter_a(
        &mut self,
        href: MdAttribute,
        title: MdAttribute,
    ) -> MdResult {
        T::enter_a(self, href, title)
    }

    fn enter_img(
        &mut self,
        src: MdAttribute,
        title: MdAttribute,
    ) -> MdResult {
       let res = T::enter_img(self, src, title);
       self.image_nesting_level += 1;
       res
    }

    fn enter_ilcode(&mut self) -> MdResult {
        T::enter_ilcode(self)
    }

    fn enter_del(&mut self) -> MdResult {
        T::enter_del(self)
    }

    fn enter_latexmath(&mut self) -> MdResult {
        T::enter_latexmath(self)
    }

    fn enter_latexmath_display(&mut self) -> MdResult {
        T::enter_latexmath_display(self)
    }

    fn enter_wikilink(&mut self, target: MdAttribute) -> MdResult {
        T::enter_wikilink(self, target)
    }

    fn enter_u(&mut self) -> MdResult {
        T::enter_u(self)
    }
}

impl<T> MdLeaveSpanCallback for Md2HtmlRenderer<T>
where
    T: Md2HtmlUserdata,
{
    fn leave_span(
        &mut self,
        spantype: MdSpantype,
        detail: *mut std::os::raw::c_void,
    ) -> MdResult {
        if self.image_nesting_level > 0 {
            if self.image_nesting_level == 1
                && spantype == MdSpantype::Img
            {
                self.leave_span_fork(spantype, detail)
            } else {
                Ok(())
            }
        } else if spantype != MdSpantype::Img {
            self.leave_span_fork(spantype, detail)
        } else {
            Ok(())
        }
    }

    fn leave_em(&mut self) -> MdResult {
        T::leave_em(self)
    }

    fn leave_strong(&mut self) -> MdResult {
        T::leave_strong(self)
    }

    fn leave_a(
        &mut self,
        href: MdAttribute,
        title: MdAttribute,
    ) -> MdResult {
        T::leave_a(self, href, title)
    }

    fn leave_img(
        &mut self,
        src: MdAttribute,
        title: MdAttribute,
    ) -> MdResult {
        self.image_nesting_level -= 1;
        T::leave_img(self, src, title)
    }

    fn leave_ilcode(&mut self) -> MdResult {
        T::leave_ilcode(self)
    }

    fn leave_del(&mut self) -> MdResult {
        T::leave_del(self)
    }

    fn leave_latexmath(&mut self) -> MdResult {
        T::leave_latexmath(self)
    }

    fn leave_latexmath_display(&mut self) -> MdResult {
        T::leave_latexmath_display(self)
    }

    fn leave_wikilink(&mut self, target: MdAttribute) -> MdResult {
        T::leave_wikilink(self, target)
    }

    fn leave_u(&mut self) -> MdResult {
        T::leave_u(self)
    }
}

impl<T> MdParser for Md2HtmlRenderer<T>
where
    T: Md2HtmlUserdata,
{
    fn debug_log(&mut self, msg: &str) {
        T::debug_log(self, msg)
    }
}

impl<T> MdTextCallback for Md2HtmlRenderer<T>
where
    T: Md2HtmlUserdata,
{
    fn text_normal(&mut self, text: &str) -> MdResult {
        T::text_normal(self, text)
    }

    fn text_nullchar(&mut self, text: &str) -> MdResult {
        T::text_nullchar(self, text)
    }

    fn text_br(&mut self, text: &str) -> MdResult {
        T::text_br(self, text)
    }

    fn text_softbr(&mut self, text: &str) -> MdResult {
        T::text_softbr(self, text)
    }

    fn text_entity(&mut self, text: &str) -> MdResult {
        T::text_entity(self, text)
    }

    fn text_code(&mut self, text: &str) -> MdResult {
        T::text_code(self, text)
    }

    fn text_html(&mut self, text: &str) -> MdResult {
        T::text_html(self, text)
    }

    fn text_latexmath(&mut self, text: &str) -> MdResult {
        T::text_latexmath(self, text)
    }
}

impl<T> Md2HtmlRenderer<T>
where
    T: Md2HtmlUserdata,
{
    pub fn new(userdata: T, flags: Md2HtmlFlags) -> Self {
        Self {
            image_nesting_level: 0,
            flags,
            userdata,
        }
    }

    fn skip_bom(&mut self, input: &mut &str) {
        if self.flags.has_skip_utf8_bom() {
            let data = input.as_bytes();
            static BOM: &[u8; 3] = &[0xef, 0xbb, 0xbf];
            if data.len() > BOM.len() && &data[0..3] == BOM {
                *input = &input[3..];
            }
        }
    }

    pub fn render(
        &mut self,
        mut input: &str,
        parser_flags: &MdParserFlags,
    ) -> MdResult {
        self.skip_bom(&mut input);
        self.parse(input, parser_flags)
    }

    pub fn render_with_logger(
        &mut self,
        mut input: &str,
        parser_flags: &MdParserFlags,
    ) -> MdResult {
        self.skip_bom(&mut input);
        self.parse_with_logger(input, parser_flags)
    }
}
