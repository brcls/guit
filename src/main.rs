use gpui::*;

struct HelloWorld {
    selected_tab: Model<String>,
}

impl Render for HelloWorld {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let selected_tab = self.selected_tab.read(cx);

        let tab_content = match selected_tab.as_str() {
            "commit" => div()
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
                ]),
            "branches" => div().child("Branches content goes here"),
            "history" => div().child("History content goes here"),
            "flow" => div().child("Flow content goes here"),
            _ => div().child("Select a tab"),
        };

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
                        div()
                            .w_full()
                            .flex()
                            .text_sm()
                            .px_4()
                            .items_center()
                            .justify_center()
                            .rounded_xl()
                            .h_10()
                            .bg(rgb(0x202020))
                            .child("commit")
                            .hover(|style| style.bg(rgb(0x282828)).cursor_pointer())
                            .on_mouse_down(MouseButton::Left, {
                                let model = self.selected_tab.clone();
                                move |_, cx: &mut WindowContext| {
                                    cx.update_model(&model, |tab, _| {
                                        *tab = "commit".to_string();
                                    });
                                }
                            }),
                        div()
                            .w_full()
                            .flex()
                            .text_sm()
                            .px_4()
                            .items_center()
                            .justify_center()
                            .rounded_xl()
                            .h_10()
                            .bg(rgb(0x202020))
                            .child("branches")
                            .hover(|style| style.bg(rgb(0x282828)).cursor_pointer())
                            .on_mouse_down(MouseButton::Left, {
                                let model = self.selected_tab.clone();
                                move |_, cx: &mut WindowContext| {
                                    cx.update_model(&model, |tab, _| {
                                        *tab = "branches".to_string();
                                    });
                                }
                            }),
                        div()
                            .w_full()
                            .flex()
                            .text_sm()
                            .px_4()
                            .items_center()
                            .justify_center()
                            .rounded_xl()
                            .h_10()
                            .bg(rgb(0x202020))
                            .child("history")
                            .hover(|style| style.bg(rgb(0x282828)).cursor_pointer())
                            .on_mouse_down(MouseButton::Left, {
                                let model = self.selected_tab.clone();
                                move |_, cx: &mut WindowContext| {
                                    cx.update_model(&model, |tab, _| {
                                        *tab = "history".to_string();
                                    });
                                }
                            }),
                        div()
                            .w_full()
                            .flex()
                            .text_sm()
                            .px_4()
                            .items_center()
                            .justify_center()
                            .rounded_xl()
                            .h_10()
                            .bg(rgb(0x202020))
                            .child("flow")
                            .hover(|style| style.bg(rgb(0x282828)).cursor_pointer())
                            .on_mouse_down(MouseButton::Left, {
                                let model = self.selected_tab.clone();
                                move |_, cx: &mut WindowContext| {
                                    cx.update_model(&model, |tab, _| {
                                        *tab = "flow".to_string();
                                    });
                                }
                            }),
                    ]),
                tab_content,
            ])
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| HelloWorld {
                selected_tab: cx.new_model(|_| "commit".to_string()),
            })
        })
        .unwrap();
    });
}
