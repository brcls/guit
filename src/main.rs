mod pages;
use branches_page::BranchesPage;
use commit_page::CommitPage;
use flow_page::FlowPage;
use gpui::*;
use history_pages::HistoryPage;
use pages::*;

struct Main {
    selected_tab: Model<String>,
}

impl Render for Main {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let selected_tab = self.selected_tab.read(cx);

        let tab_content = match selected_tab.as_str() {
            "commit" => CommitPage.into_any_element(),
            "branches" => BranchesPage.into_any_element(),
            "history" => HistoryPage.into_any_element(),
            "flow" => FlowPage.into_any_element(),
            _ => CommitPage.into_any_element(),
        };

        div()
            .bg(rgb(0x202020))
            .size_full()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(
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
            )
            .child(tab_content)
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| Main {
                selected_tab: cx.new_model(|_| "commit".to_string()),
            })
        })
        .unwrap();
    });
}
