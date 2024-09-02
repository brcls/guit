use std::process::Child;

use gpui::*;

use crate::components::list_item::ListItem; // Importa o ListItem do módulo components

pub struct List;

impl Render for List {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        return uniform_list(
            cx.view().clone(),
            "ListOfCommits",
            1000,
            move |_this: &mut List,
                  _range: std::ops::Range<usize>,
                  _cx: &mut ViewContext<'_, List>| {
                let mut items = Vec::new();
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                items.push(ListItem);
                return items;
            },
        )
        .id("ListOfCommits")
        .size_full()
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
                    .h_5_6()
                    .w_2_5()
                    .rounded_xl()
                    .gap_2()
                    .flex()
                    .flex_col()
                    .my_4()
                    .ml_4()
                    .mr_2()
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
                            .mx_2()
                            .text_sm(),
                    )
                    .child(list),
            )
            .child(
                div()
                    .h_5_6()
                    .w_full()
                    .bg(rgb(0x202020))
                    .rounded_xl()
                    .my_4()
                    .mr_4()
                    .ml_2(),
            );
    }
}
