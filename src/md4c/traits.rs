use super::*;

fn intoref<T>(detail: *mut c_void) -> &'static T {
    let detail = detail as *const T;
    unsafe { detail.as_ref().unwrap_unchecked() }
}

pub trait MdEnterBlockCallback {
    #[allow(unused)]
    fn enter_block(
        &mut self,
        blocktype: MdBlocktype,
        detail: *mut c_void,
    ) -> MdResult {
        /* noop */
        self.enter_block_fork(blocktype, detail)
    }
    fn enter_block_fork(
        &mut self,
        blocktype: MdBlocktype,
        detail: *mut c_void,
    ) -> MdResult {
        match blocktype {
            MdBlocktype::Doc => self.enter_doc(),
            MdBlocktype::Quote => self.enter_quote(),
            MdBlocktype::Ul => {
                let detail: &sys::MD_BLOCK_UL_DETAIL =
                    intoref(detail);
                let is_tight = detail.is_tight != 0;
                let mark = detail.mark as MdChar;
                self.enter_ul(is_tight, mark)
            }
            MdBlocktype::Ol => {
                let detail: &sys::MD_BLOCK_OL_DETAIL =
                    intoref(detail);
                let start = detail.start as u32;
                let is_tight = detail.is_tight != 0;
                let mark_delimiter = detail.mark_delimiter as MdChar;
                self.enter_ol(start, is_tight, mark_delimiter)
            }
            MdBlocktype::Li => {
                let detail: &sys::MD_BLOCK_LI_DETAIL =
                    intoref(detail);
                let is_task = detail.is_task != 0;
                let task_mark = detail.task_mark as MdChar;
                let task_mark_offset = detail.task_mark_offset as u32;
                self.enter_li(is_task, task_mark, task_mark_offset)
            }
            MdBlocktype::Hr => self.enter_hr(),
            MdBlocktype::H => {
                let detail: &sys::MD_BLOCK_H_DETAIL = intoref(detail);
                let level = detail.level as u32;
                self.enter_h(level)
            }
            MdBlocktype::Code => {
                let detail: &sys::MD_BLOCK_CODE_DETAIL =
                    intoref(detail);
                let info = detail.info.into();
                let lang = detail.lang.into();
                let fence_char = detail.fence_char as MdChar;
                self.enter_code(info, lang, fence_char)
            }
            MdBlocktype::Html => self.enter_html(),
            MdBlocktype::P => self.enter_p(),
            MdBlocktype::Table => {
                let detail: &sys::MD_BLOCK_TABLE_DETAIL =
                    intoref(detail);
                let col_count = detail.col_count as u32;
                let head_row_count = detail.head_row_count as u32;
                let body_row_count = detail.body_row_count as u32;
                self.enter_table(
                    col_count,
                    head_row_count,
                    body_row_count,
                )
            }
            MdBlocktype::Thead => self.enter_thead(),
            MdBlocktype::Tbody => self.enter_tbody(),
            MdBlocktype::Tr => self.enter_tr(),
            MdBlocktype::Th => {
                let detail: &sys::MD_BLOCK_TD_DETAIL =
                    intoref(detail);
                let align = detail.align.into();
                self.enter_th(align)
            }
            MdBlocktype::Td => {
                let detail: &sys::MD_BLOCK_TD_DETAIL =
                    intoref(detail);
                let align = detail.align.into();
                self.enter_td(align)
            }
        }
    }

    fn enter_doc(&mut self) -> MdResult;
    fn enter_quote(&mut self) -> MdResult;
    fn enter_ul(&mut self, is_tight: bool, mark: MdChar) -> MdResult;
    fn enter_ol(
        &mut self,
        start: u32,
        is_tight: bool,
        mark_delimiter: MdChar,
    ) -> MdResult;
    fn enter_li(
        &mut self,
        is_task: bool,
        task_mark: MdChar,
        task_mark_offset: MdOffset,
    ) -> MdResult;
    fn enter_hr(&mut self) -> MdResult;
    fn enter_h(&mut self, level: u32) -> MdResult;
    fn enter_code(
        &mut self,
        info: MdAttribute,
        lang: MdAttribute,
        fence_char: MdChar,
    ) -> MdResult;
    fn enter_html(&mut self) -> MdResult;
    fn enter_p(&mut self) -> MdResult;
    fn enter_table(
        &mut self,
        col_count: u32,
        head_row_count: u32,
        body_row_count: u32,
    ) -> MdResult;
    fn enter_thead(&mut self) -> MdResult;
    fn enter_tbody(&mut self) -> MdResult;
    fn enter_tr(&mut self) -> MdResult;
    fn enter_th(&mut self, align: MdAlign) -> MdResult;
    fn enter_td(&mut self, align: MdAlign) -> MdResult;
}

pub trait MdLeaveBlockCallback {
    #[allow(unused)]
    fn leave_block(
        &mut self,
        blocktype: MdBlocktype,
        detail: *mut c_void,
    ) -> MdResult {
        /* noop */
        self.leave_block_fork(blocktype, detail)
    }
    fn leave_block_fork(
        &mut self,
        blocktype: MdBlocktype,
        detail: *mut c_void,
    ) -> MdResult {
        match blocktype {
            MdBlocktype::Doc => self.leave_doc(),
            MdBlocktype::Quote => self.leave_quote(),
            MdBlocktype::Ul => {
                let detail: &sys::MD_BLOCK_UL_DETAIL =
                    intoref(detail);
                let is_tight = detail.is_tight != 0;
                let mark = detail.mark as MdChar;
                self.leave_ul(is_tight, mark)
            }
            MdBlocktype::Ol => {
                let detail: &sys::MD_BLOCK_OL_DETAIL =
                    intoref(detail);
                let start = detail.start as u32;
                let is_tight = detail.is_tight != 0;
                let mark_delimiter = detail.mark_delimiter as MdChar;
                self.leave_ol(start, is_tight, mark_delimiter)
            }
            MdBlocktype::Li => {
                let detail: &sys::MD_BLOCK_LI_DETAIL =
                    intoref(detail);
                let is_task = detail.is_task != 0;
                let task_mark = detail.task_mark as MdChar;
                let task_mark_offset = detail.task_mark_offset as u32;
                self.leave_li(is_task, task_mark, task_mark_offset)
            }
            MdBlocktype::Hr => self.leave_hr(),
            MdBlocktype::H => {
                let detail: &sys::MD_BLOCK_H_DETAIL = intoref(detail);
                let level = detail.level as u32;
                self.leave_h(level)
            }
            MdBlocktype::Code => {
                let detail: &sys::MD_BLOCK_CODE_DETAIL =
                    intoref(detail);
                let info = detail.info.into();
                let lang = detail.lang.into();
                let fence_char = detail.fence_char as MdChar;
                self.leave_code(info, lang, fence_char)
            }
            MdBlocktype::Html => self.leave_html(),
            MdBlocktype::P => self.leave_p(),
            MdBlocktype::Table => {
                let detail: &sys::MD_BLOCK_TABLE_DETAIL =
                    intoref(detail);
                let col_count = detail.col_count as u32;
                let head_row_count = detail.head_row_count as u32;
                let body_row_count = detail.body_row_count as u32;
                self.leave_table(
                    col_count,
                    head_row_count,
                    body_row_count,
                )
            }
            MdBlocktype::Thead => self.leave_thead(),
            MdBlocktype::Tbody => self.leave_tbody(),
            MdBlocktype::Tr => self.leave_tr(),
            MdBlocktype::Th => {
                let detail: &sys::MD_BLOCK_TD_DETAIL =
                    intoref(detail);
                let align = detail.align.into();
                self.leave_th(align)
            }
            MdBlocktype::Td => {
                let detail: &sys::MD_BLOCK_TD_DETAIL =
                    intoref(detail);
                let align = detail.align.into();
                self.leave_td(align)
            }
        }
    }

    fn leave_doc(&mut self) -> MdResult;
    fn leave_quote(&mut self) -> MdResult;
    fn leave_ul(&mut self, is_tight: bool, mark: MdChar) -> MdResult;
    fn leave_ol(
        &mut self,
        start: u32,
        is_tight: bool,
        mark_delimiter: MdChar,
    ) -> MdResult;
    fn leave_li(
        &mut self,
        is_task: bool,
        task_mark: MdChar,
        task_mark_offset: MdOffset,
    ) -> MdResult;
    fn leave_hr(&mut self) -> MdResult;
    fn leave_h(&mut self, level: u32) -> MdResult;
    fn leave_code(
        &mut self,
        info: MdAttribute,
        lang: MdAttribute,
        fence_char: MdChar,
    ) -> MdResult;
    fn leave_html(&mut self) -> MdResult;
    fn leave_p(&mut self) -> MdResult;
    fn leave_table(
        &mut self,
        col_count: u32,
        head_row_count: u32,
        body_row_count: u32,
    ) -> MdResult;
    fn leave_thead(&mut self) -> MdResult;
    fn leave_tbody(&mut self) -> MdResult;
    fn leave_tr(&mut self) -> MdResult;
    fn leave_th(&mut self, align: MdAlign) -> MdResult;
    fn leave_td(&mut self, align: MdAlign) -> MdResult;
}

pub trait MdEnterSpanCallback {
    #[allow(unused)]
    fn enter_span(
        &mut self,
        spantype: MdSpantype,
        detail: *mut c_void,
    ) -> MdResult {
        /* noop */
        self.enter_span_fork(spantype, detail)
    }
    fn enter_span_fork(
        &mut self,
        spantype: MdSpantype,
        detail: *mut c_void,
    ) -> MdResult {
        match spantype {
            MdSpantype::Em => self.enter_em(),
            MdSpantype::Strong => self.enter_strong(),
            MdSpantype::A => {
                let detail: &sys::MD_SPAN_A_DETAIL = intoref(detail);
                let href = detail.href.into();
                let title = detail.title.into();
                self.enter_a(href, title)
            }
            MdSpantype::Img => {
                let detail: &sys::MD_SPAN_IMG_DETAIL =
                    intoref(detail);
                let src = detail.src.into();
                let title = detail.title.into();
                self.enter_img(src, title)
            }
            MdSpantype::Code => self.enter_ilcode(),
            MdSpantype::Del => self.enter_del(),
            MdSpantype::Latexmath => self.enter_latexmath(),
            MdSpantype::LatexmathDisplay => {
                self.enter_latexmath_display()
            }
            MdSpantype::Wikilink => {
                let detail: &sys::MD_SPAN_WIKILINK_DETAIL =
                    intoref(detail);
                let target = detail.target.into();
                self.enter_wikilink(target)
            }
            MdSpantype::U => self.enter_u(),
        }
    }

    fn enter_em(&mut self) -> MdResult;
    fn enter_strong(&mut self) -> MdResult;
    fn enter_a(
        &mut self,
        href: MdAttribute,
        title: MdAttribute,
    ) -> MdResult;
    fn enter_img(
        &mut self,
        src: MdAttribute,
        title: MdAttribute,
    ) -> MdResult;
    fn enter_ilcode(&mut self) -> MdResult;
    fn enter_del(&mut self) -> MdResult;
    fn enter_latexmath(&mut self) -> MdResult;
    fn enter_latexmath_display(&mut self) -> MdResult;
    fn enter_wikilink(&mut self, target: MdAttribute) -> MdResult;
    fn enter_u(&mut self) -> MdResult;
}

pub trait MdLeaveSpanCallback {
    #[allow(unused)]
    fn leave_span(
        &mut self,
        spantype: MdSpantype,
        detail: *mut c_void,
    ) -> MdResult {
        /* noop */
        self.leave_span_fork(spantype, detail)
    }
    fn leave_span_fork(
        &mut self,
        spantype: MdSpantype,
        detail: *mut c_void,
    ) -> MdResult {
        match spantype {
            MdSpantype::Em => self.leave_em(),
            MdSpantype::Strong => self.leave_strong(),
            MdSpantype::A => {
                let detail: &sys::MD_SPAN_A_DETAIL = intoref(detail);
                let href = detail.href.into();
                let title = detail.title.into();
                self.leave_a(href, title)
            }
            MdSpantype::Img => {
                let detail: &sys::MD_SPAN_IMG_DETAIL =
                    intoref(detail);
                let src = detail.src.into();
                let title = detail.title.into();
                self.leave_img(src, title)
            }
            MdSpantype::Code => self.leave_ilcode(),
            MdSpantype::Del => self.leave_del(),
            MdSpantype::Latexmath => self.leave_latexmath(),
            MdSpantype::LatexmathDisplay => {
                self.leave_latexmath_display()
            }
            MdSpantype::Wikilink => {
                let detail: &sys::MD_SPAN_WIKILINK_DETAIL =
                    intoref(detail);
                let target = detail.target.into();
                self.leave_wikilink(target)
            }
            MdSpantype::U => self.leave_u(),
        }
    }

    fn leave_em(&mut self) -> MdResult;
    fn leave_strong(&mut self) -> MdResult;
    fn leave_a(
        &mut self,
        href: MdAttribute,
        title: MdAttribute,
    ) -> MdResult;
    fn leave_img(
        &mut self,
        src: MdAttribute,
        title: MdAttribute,
    ) -> MdResult;
    fn leave_ilcode(&mut self) -> MdResult;
    fn leave_del(&mut self) -> MdResult;
    fn leave_latexmath(&mut self) -> MdResult;
    fn leave_latexmath_display(&mut self) -> MdResult;
    fn leave_wikilink(&mut self, target: MdAttribute) -> MdResult;
    fn leave_u(&mut self) -> MdResult;
}

pub trait MdTextCallback {
    #[allow(unused)]
    fn text(&mut self, texttype: MdTexttype, text: &str) -> MdResult {
        /* noop */
        self.text_fork(text, texttype)
    }
    fn text_fork(
        &mut self,
        text: &str,
        texttype: MdTexttype,
    ) -> MdResult {
        match texttype {
            MdTexttype::Normal => self.text_normal(text),
            MdTexttype::Nullchar => self.text_nullchar(text),
            MdTexttype::Br => self.text_br(text),
            MdTexttype::Softbr => self.text_softbr(text),
            MdTexttype::Entity => self.text_entity(text),
            MdTexttype::Code => self.text_code(text),
            MdTexttype::Html => self.text_html(text),
            MdTexttype::Latexmath => self.text_latexmath(text),
        }
    }

    fn text_normal(&mut self, text: &str) -> MdResult;
    fn text_nullchar(&mut self, text: &str) -> MdResult;
    fn text_br(&mut self, text: &str) -> MdResult;
    fn text_softbr(&mut self, text: &str) -> MdResult;
    fn text_entity(&mut self, text: &str) -> MdResult;
    fn text_code(&mut self, text: &str) -> MdResult;
    fn text_html(&mut self, text: &str) -> MdResult;
    fn text_latexmath(&mut self, text: &str) -> MdResult;
}
