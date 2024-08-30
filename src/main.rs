mod components;
mod pages;

use branches_page::BranchesPage;
use commit_page::CommitPage;
use components::header::Header;
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
            .child(Header {
                selected_tab: self.selected_tab.clone(),
            })
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
