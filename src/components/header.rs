use gpui::*;
#[derive(IntoElement)]

pub struct HeaderButton {
    pub selected_tab: Model<String>,
    text: String,
    id: ElementId,
}

impl RenderOnce for HeaderButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        return div()
            .id(self.id)
            .w_full()
            .flex()
            .text_sm()
            .px_4()
            .items_center()
            .justify_center()
            .rounded_xl()
            .h_10()
            .bg(rgb(0x151515))
            .child(self.text.clone())
            .border_1()
            .border_color(rgb(0x202020))
            .hover(|style| style.bg(rgb(0x202020)).cursor_pointer())
            .border_1()
            .focusable()
            .focus(|style| style.bg(rgb(0x202020)).cursor_pointer())
            .on_click({
                let model = self.selected_tab.clone();
                move |_, cx: &mut WindowContext| {
                    cx.update_model(&model, |tab, _| {
                        *tab = self.text.clone();
                    });
                }
            });
    }
}

#[derive(IntoElement)]
pub struct Header {
    pub selected_tab: Model<String>,
}

impl RenderOnce for Header {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let changes_button = HeaderButton {
            selected_tab: self.selected_tab.clone(),
            text: "changes".to_string(),
            id: ElementId::Name("changes_button".into()),
        };

        let history_button = HeaderButton {
            selected_tab: self.selected_tab.clone(),
            text: "history".to_string(),
            id: ElementId::Name("history_button".into()),
        };
        let settings_button = HeaderButton {
            selected_tab: self.selected_tab.clone(),
            text: "settings".to_string(),
            id: ElementId::Name("settings_button".into()),
        };

        return div()
            .w_full()
            .flex()
            .justify_between()
            .items_center()
            .p_2()
            .gap_2()
            .child(div().font_weight(FontWeight(900.0)).child("guit").mx_4())
            .child(div().flex().gap_2().w_1_3().children([
                changes_button,
                history_button,
                settings_button,
            ]))
            .relative();
    }
}
