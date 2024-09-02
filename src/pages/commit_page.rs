use std::{borrow::Borrow, process::Child};

use gpui::*;

use crate::components::list_item::ListItem; // Importa o ListItem do módulo components

const FILE_SVG: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/images/file-regular.svg");

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
        let list = cx.new_view(|cx: &mut ViewContext<'_, List>| List);

        return div()
            .size_full()
            .flex()
            .items_start()
            .justify_between()
            .child(
                div()
                    .w_2_5()
                    .h_5_6()
                    .gap_2()
                    .my_4()
                    .ml_4()
                    .mr_2()
                    .child(
                        div()
                            .h_1_2()
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
                            .h_1_2()
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
