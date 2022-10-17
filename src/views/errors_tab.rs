use nwg::ListView;
use crate::models::Logs;
use crate::views::TabView;

pub struct ErrorsTab { }

impl TabView for ErrorsTab {
    fn load_data(logs: &Logs, dv: &ListView) {
        dv.clear();

        for log in logs.records().iter().rev() {
            let status = log.status();
            if !status.starts_with('2') {
                dv.insert_items_row(None, &log.to_slice());
            }
        }
    }
}