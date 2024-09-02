use gpui::*;
#[derive(IntoElement)]

pub struct HeaderButton {
    pub selected_tab: Model<String>,
    text: String,
    icon: Svg,
    id: ElementId,
}

impl RenderOnce for HeaderButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        return div()
            .id(self.id)
            .size_10()
            .flex()
            .text_sm()
            .items_center()
            .justify_center()
            .rounded_xl()
            .bg(rgb(0x151515))
            .child(self.icon)
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

const GEAR_SVG: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/images/gear.svg");
const CODE_COMMIT_SVG: &'static str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/images/code-commit-solid.svg");
const CODE_BRANCH_SVG: &'static str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/images/code-branch-solid.svg");
const TIMELINE_SVG: &'static str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/images/timeline-solid.svg");

impl RenderOnce for Header {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let changes_button = HeaderButton {
            selected_tab: self.selected_tab.clone(),
            text: "changes".to_string(),
            icon: svg().size_3().path(CODE_COMMIT_SVG).text_color(white()),
            id: ElementId::Name("changes_button".into()),
        };

        let history_button = HeaderButton {
            selected_tab: self.selected_tab.clone(),
            text: "history".to_string(),
            icon: svg().size_3().path(TIMELINE_SVG).text_color(white()),
            id: ElementId::Name("history_button".into()),
        };

        let flow_button = HeaderButton {
            selected_tab: self.selected_tab.clone(),
            text: "flow".to_string(),
            icon: svg().size_3().path(CODE_BRANCH_SVG).text_color(white()),
            id: ElementId::Name("flow_button".into()),
        };

        let settings_button = HeaderButton {
            selected_tab: self.selected_tab.clone(),
            text: "settings".to_string(),
            icon: svg().size_3().path(GEAR_SVG).text_color(white()),
            id: ElementId::Name("settings_button".into()),
        };

        return div()
            .w_full()
            .flex()
            .justify_between()
            .items_center()
            .px_4()
            .py_2()
            .gap_2()
            .child(div().font_weight(FontWeight(900.0)).child("guit"))
            .child(div().flex().gap_2().children([
                changes_button,
                flow_button,
                history_button,
                settings_button,
            ]))
            .relative();
    }
}
