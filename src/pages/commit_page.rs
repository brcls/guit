use std::borrow::Borrow;

use gpui::*;

use crate::components::list_item::ListItem; // Importa o ListItem do módulo components

const FILE_SVG: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/images/file-regular.svg");
const CODE_BRANCH_SVG: &'static str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/images/code-branch-solid.svg");

pub struct List;

impl Render for List {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let scroll_handle = ScrollHandle::new();

        return uniform_list(
            cx.view().clone(),
            "ListOfCommits",
            1000,
            move |_this: &mut List,
                  _range: std::ops::Range<usize>,
                  _cx: &mut ViewContext<'_, List>| {
                let mut items = Vec::new();
                items.push(ListItem {
                    icon: svg().size_3().path(FILE_SVG).text_color(white()),
                });

                return items;
            },
        )
        .id("ListOfCommits")
        .size_full()
        .track_scroll(scroll_handle.borrow())
        .gap_2();
    }
}

#[derive(IntoElement)]
pub struct CommitPage;

impl RenderOnce for CommitPage {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let list = cx.new_view(|_cx: &mut ViewContext<'_, List>| List);

        return div()
            .size_full()
            .flex()
            .items_start()
            .justify_between()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .justify_between()
                    .w_2_5()
                    .max_h_5_6()
                    .h_5_6()
                    .gap_2()
                    .mt_4()
                    .ml_4()
                    .mr_2()
                    .child(
                        div()
                            .h_1_3()
                            .child(
                                div()
                                    .font_weight(FontWeight(900.0))
                                    .child("staged")
                                    .child(
                                        div()
                                            .child("10")
                                            .text_size(Pixels(10.0))
                                            .bg(rgb(0x202020))
                                            .size_7()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .rounded_full(),
                                    )
                                    .flex()
                                    .justify_between()
                                    .items_center()
                                    .m_2()
                                    .text_sm(),
                            )
                            .child(list.clone()),
                    )
                    .child(
                        div()
                            .h_1_3()
                            .child(
                                div()
                                    .font_weight(FontWeight(900.0))
                                    .child("changes")
                                    .child(
                                        div()
                                            .child("10")
                                            .text_size(Pixels(10.0))
                                            .bg(rgb(0x202020))
                                            .size_7()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .rounded_full(),
                                    )
                                    .flex()
                                    .justify_between()
                                    .items_center()
                                    .m_2()
                                    .text_sm(),
                            )
                            .child(list.clone()),
                    )
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .text_sm()
                            .px_4()
                            .gap_4()
                            .items_center()
                            .rounded_xl()
                            .h_10()
                            .bg(rgb(0x151515))
                            .border_1()
                            .border_color(rgb(0x202020))
                            .child(svg().size_3().path(CODE_BRANCH_SVG).text_color(white()))
                            .child("feature/test")
                            .hover(|style| style.bg(rgb(0x202020)).cursor_pointer()),
                    ),
            )
            .child(
                div()
                    .h_5_6()
                    .w_full()
                    .bg(rgb(0x181818))
                    .rounded_xl()
                    .my_4()
                    .mr_4()
                    .ml_2(),
            );
    }
}
