use nwg::ListView;
use crate::models::Logs;
use crate::views::TabView;

pub struct LogsTab {}

impl TabView for LogsTab {
    fn load_data(logs: &Logs, dv: &ListView) {
        dv.clear();

        for log in logs.records().iter().rev() {
            dv.insert_items_row(None, &log.to_slice());
        }
    }
}