use gpui::*;

#[derive(IntoElement)]

pub struct BranchesPage;

impl RenderOnce for BranchesPage {
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
            .bg(rgb(0x181818));
    }
}
