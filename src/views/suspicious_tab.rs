use nwg::ListView;
use crate::models::Logs;
use crate::views::TabView;

pub struct SuspiciousTab { }

impl TabView for SuspiciousTab {
    fn load_data(logs: &Logs, dv: &ListView) {
        dv.clear();

        for log in logs.records().iter().rev() {
            let path = log.path();
            let user_agent = log.user_agent();
            let method = log.method();

            if path.contains(".") &&
                !path.contains("ads.txt") &&
                !path.contains("robots.txt") &&
                !path.contains("sitemap.xml") &&
                !path.contains("forum.js") &&
                !path.contains("forum-pl.js") &&
                !path.contains(".woff") &&
                !path.contains(".css") &&
                !path.contains(".jpg") &&
                !path.contains(".svg") &&
                !path.contains(".png") ||
                user_agent.len() < 3 ||
                method != "GET" &&
                method != "POST" {
                dv.insert_items_row(None, &log.to_slice());
            }
        }
    }
}