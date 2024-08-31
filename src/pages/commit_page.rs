use gpui::*;

use crate::components::list_item::ListItem; // Importa o ListItem do módulo components

#[derive(IntoElement)]
pub struct CommitPage;

impl RenderOnce for CommitPage {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let item = ListItem.into_any_element();
        let item1 = ListItem.into_any_element();
        let item2 = ListItem.into_any_element();

        return div()
            .size_full()
            .w_1_3()
            .child(
                div()
                    .h_auto()
                    .m_2()
                    .rounded_xl()
                    .p_2()
                    .gap_2()
                    .flex()
                    .flex_col()
                    .bg(rgb(0x181818))
                    .children([item, item2]),
            )
            .child(
                div()
                    .h_auto()
                    .m_2()
                    .rounded_xl()
                    .p_2()
                    .gap_2()
                    .flex()
                    .flex_col()
                    .bg(rgb(0x181818))
                    .children([item1]),
            );
    }
}
