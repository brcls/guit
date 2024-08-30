mod components;
mod pages;

use branches_page::BranchesPage;
use commit_page::CommitPage;
use components::header::Header;
use flow_page::FlowPage;
use gpui::*;
use history_pages::HistoryPage;
use pages::*;

enum Page {
    Commit(CommitPage),
    Branches(BranchesPage),
    History(HistoryPage),
    Flow(FlowPage),
}

impl RenderOnce for Page {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        match self {
            Page::Commit(page) => page.render(cx).into_any_element(),
            Page::Branches(page) => page.render(cx).into_any_element(),
            Page::History(page) => page.render(cx).into_any_element(),
            Page::Flow(page) => page.render(cx).into_any_element(),
        }
    }
}

struct Main {
    selected_tab: Model<String>,
}

impl Render for Main {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let selected_tab = self.selected_tab.read(cx);

        let tab_content = match selected_tab.as_str() {
            "commit" => Page::Commit(CommitPage),
            "branches" => Page::Branches(BranchesPage),
            "history" => Page::History(HistoryPage),
            "flow" => Page::Flow(FlowPage),
            _ => Page::Commit(CommitPage),
        };

        div()
            .bg(rgb(0x202020))
            .size_full()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(Header {
                selected_tab: self.selected_tab.clone(),
            })
            .child(tab_content.render(cx))
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
