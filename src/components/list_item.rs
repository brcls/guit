use gpui::*;

#[derive(IntoElement)]
pub struct ListItem;

impl RenderOnce for ListItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        return div()
            .w_full()
            .flex()
            .text_sm()
            .px_4()
            .items_center()
            .rounded_xl()
            .h_10()
            .bg(rgb(0x202020))
            .child("file.rs")
            .hover(|style| style.bg(rgb(0x282828)).cursor_pointer());
    }
}
