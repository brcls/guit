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
        .overflow_scroll();
    }
}

#[derive(IntoElement)]
pub struct CommitPage;

impl RenderOnce for CommitPage {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let list = cx.new_view(|cx: &mut ViewContext<'_, List>| List);

        return div().h_128().w_1_3().child(
            div()
                .h_128()
                .m_2()
                .rounded_xl()
                .p_2()
                .gap_2()
                .flex()
                .flex_col()
                .bg(rgb(0x202020))
                .child(list)
                .overflow_y_hidden(),
        );
    }
}
