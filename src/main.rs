mod components;
mod pages;

use std::borrow::Borrow;

use branches_page::BranchesPage;
use commit_page::CommitPage;
use components::header::{self, Header};
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
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        match self {
            Page::Commit(page) => page.render(cx).into_any_element(),
            Page::Branches(page) => page.render(cx).into_any_element(),
            Page::History(page) => page.render(cx).into_any_element(),
            Page::Flow(page) => page.render(cx).into_any_element(),
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
            "commit" => commit.render(cx),
            "branches" => branches.render(cx),
            "history" => history.render(cx),
            "flow" => flow.render(cx),
            _ => Page::Commit(CommitPage).render(cx),
        };

        return tab_content;
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
