use gpui::*;

#[derive(IntoElement)]

pub struct CommitPage;

impl RenderOnce for CommitPage {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        return div()
            .h_full()
            .m_2()
            .rounded_xl()
            .w_1_3()
            .p_2()
            .gap_2()
            .flex()
            .flex_col()
            .bg(rgb(0x181818))
            .children([
                div()
                    .w_full()
                    .flex()
                    .text_sm()
                    .px_4()
                    .items_center()
                    .rounded_xl()
                    .h_10()
                    .bg(rgb(0x202020))
                    .child("file.rs")
                    .hover(|style| style.bg(rgb(0x282828)).cursor_pointer()),
                div()
                    .w_full()
                    .flex()
                    .text_sm()
                    .px_4()
                    .items_center()
                    .rounded_xl()
                    .h_10()
                    .bg(rgb(0x202020))
                    .child("file.rs")
                    .hover(|style| style.bg(rgb(0x282828)).cursor_pointer()),
                div()
                    .w_full()
                    .flex()
                    .text_sm()
                    .px_4()
                    .items_center()
                    .rounded_xl()
                    .h_10()
                    .bg(rgb(0x202020))
                    .child("file.rs")
                    .hover(|style| style.bg(rgb(0x282828)).cursor_pointer()),
                div()
                    .w_full()
                    .flex()
                    .text_sm()
                    .px_4()
                    .items_center()
                    .rounded_xl()
                    .h_10()
                    .bg(rgb(0x202020))
                    .child("file.rs")
                    .hover(|style| style.bg(rgb(0x282828)).cursor_pointer()),
            ]);
    }
}
