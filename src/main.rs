use gpui::*;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .bg(rgb(0x202020))
            .size_full()
            .text_xl()
            .text_color(rgb(0xffffff))
            .children([
                div()
                    .w_full()
                    .flex()
                    .bg(rgb(0x181818))
                    .justify_between()
                    .items_center()
                    .p_2()
                    .gap_2()
                    .children([
                        div().w_full().rounded_xl().h_10().bg(rgb(0x202020)),
                        div().w_full().rounded_xl().h_10().bg(rgb(0x202020)),
                        div().w_full().rounded_xl().h_10().bg(rgb(0x202020)),
                        div().w_full().rounded_xl().h_10().bg(rgb(0x202020)),
                    ]),
                div()
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
                            .child(format!("Hello, {}!", &self.text)),
                        div()
                            .w_full()
                            .flex()
                            .text_sm()
                            .px_4()
                            .items_center()
                            .rounded_xl()
                            .h_10()
                            .bg(rgb(0x202020))
                            .child(format!("Hello, {}!", &self.text)),
                        div()
                            .w_full()
                            .flex()
                            .text_sm()
                            .px_4()
                            .items_center()
                            .rounded_xl()
                            .h_10()
                            .bg(rgb(0x202020))
                            .child(format!("Hello, {}!", &self.text)),
                        div()
                            .w_full()
                            .flex()
                            .text_sm()
                            .px_4()
                            .items_center()
                            .rounded_xl()
                            .h_10()
                            .bg(rgb(0x202020))
                            .child(format!("Hello, {}!", &self.text)),
                    ]),
            ])
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
            })
        })
        .unwrap();
    });
}
