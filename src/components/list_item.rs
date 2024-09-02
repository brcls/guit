use gpui::*;

#[derive(IntoElement)]
pub struct ListItem;

impl RenderOnce for ListItem {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        return div()
            .w_full()
            .flex()
            .text_sm()
            .px_4()
            .items_center()
            .rounded_xl()
            .h_10()
            .bg(rgb(0x151515))
            .border_1()
            .border_color(rgb(0x202020))
            .child("file.rs")
            .my_4()
            .hover(|style| style.bg(rgb(0x202020)).cursor_pointer());
    }
}
