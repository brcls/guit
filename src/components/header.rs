use gpui::*;

#[derive(IntoElement)]
pub struct HeaderButton {
    pub selected_tab: Model<String>,
    text: String,
    icon: Svg,
    id: ElementId,
}

impl HeaderButton {
    pub fn new(selected_tab: Model<String>, text: String, icon: Svg, id: ElementId) -> Self {
        Self {
            selected_tab: selected_tab,
            text: text,
            icon: icon,
            id: id,
        }
    }
}

impl RenderOnce for HeaderButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        return div()
            .id(self.id)
            .min_w_10()
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
pub struct RepositoryButton {
    pub selected_tab: Model<String>,
    text: String,
    icon: Svg,
    id: ElementId,
}

impl RepositoryButton {
    pub fn new(selected_tab: Model<String>, text: String, icon: Svg, id: ElementId) -> Self {
        Self {
            selected_tab: selected_tab,
            text: text,
            icon: icon,
            id: id,
        }
    }
}

impl RenderOnce for RepositoryButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        return div()
            .id(self.id)
            .px_4()
            .gap_2()
            .h_10()
            .w_full()
            .flex()
            .text_xs()
            .items_center()
            .justify_start()
            .rounded_xl()
            .bg(rgb(0x151515))
            .child(self.icon)
            .text_color(rgb(0xAAAAAA))
            .child(self.text.clone())
            .border_1()
            .border_color(rgb(0x202020))
            .hover(|style| style.bg(rgb(0x202020)).cursor_pointer())
            .border_1()
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
const BOOK_SVG: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/images/book-solid.svg");

impl RenderOnce for Header {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let changes_button = HeaderButton::new(
            self.selected_tab.clone(),
            "changes".to_string(),
            svg().size_3().path(CODE_COMMIT_SVG).text_color(white()),
            ElementId::Name("changes_button".into()),
        );

        let history_button = HeaderButton::new(
            self.selected_tab.clone(),
            "history".to_string(),
            svg().size_3().path(TIMELINE_SVG).text_color(white()),
            ElementId::Name("history_button".into()),
        );

        let flow_button = HeaderButton::new(
            self.selected_tab.clone(),
            "flow".to_string(),
            svg().size_3().path(CODE_BRANCH_SVG).text_color(white()),
            ElementId::Name("flow_button".into()),
        );

        let settings_button = HeaderButton::new(
            self.selected_tab.clone(),
            "settings".to_string(),
            svg().size_3().path(GEAR_SVG).text_color(white()),
            ElementId::Name("settings_button".into()),
        );

        let repository_button = RepositoryButton::new(
            self.selected_tab.clone(),
            "repository: guit-project".to_string(),
            svg().size_3().path(BOOK_SVG).text_color(white()),
            ElementId::Name("repository_button".into()),
        );

        return div()
            .w_full()
            .flex()
            .justify_between()
            .items_center()
            .px_4()
            .py_2()
            .gap_2()
            .child(div().w_2_5().font_weight(FontWeight(900.0)).child("guit"))
            .child(
                div()
                    .w_full()
                    .flex()
                    .gap_2()
                    .child(repository_button)
                    .children([changes_button, flow_button, history_button, settings_button]),
            )
            .relative();
    }
}
