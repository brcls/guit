mod components;
mod pages;

use branches_page::BranchesPage;
use commit_page::CommitPage;
use components::header::Header;
use flow_page::FlowPage;
use gpui::*;
use history_page::HistoryPage;
use pages::*;

enum Page {
    Commit(CommitPage),
    Branches(BranchesPage),
    History(HistoryPage),
    Flow(FlowPage),
}

impl RenderOnce for Page {
    fn render(self, _: &mut WindowContext) -> impl IntoElement {
        match self {
            Page::Commit(page) => page.into_any_element(),
            Page::Branches(page) => page.into_any_element(),
            Page::History(page) => page.into_any_element(),
            Page::Flow(page) => page.into_any_element(),
        }
    }
}

struct Tabs {
    selected_tab: Model<String>,
}

impl RenderOnce for Tabs {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let selected_tab = self.selected_tab.read(cx).as_str();
        let commit = Page::Commit(CommitPage);
        let branches = Page::Branches(BranchesPage);
        let history = Page::History(HistoryPage);
        let flow = Page::Flow(FlowPage);

        let tab_content = match selected_tab {
            "commit" => commit,
            "branches" => branches,
            "history" => history,
            "flow" => flow,
            _ => Page::Commit(CommitPage),
        };

        return tab_content.render(cx);
    }
}

struct Main {
    selected_tab: Model<String>,
}

impl Render for Main {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let header = Header {
            selected_tab: self.selected_tab.clone(),
        };
        let tabs = Tabs {
            selected_tab: self.selected_tab.clone(),
        };

        div()
            .bg(rgb(0x202020))
            .size_full()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(header)
            .child(tabs.render(cx))
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
