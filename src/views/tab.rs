use nwg::ListView;
use crate::models::Logs;

pub trait TabView {
    fn setup_view(dv: &ListView) {
        dv.insert_column("IP");
        dv.insert_column(nwg::InsertListViewColumn {
            index: Some(1),
            fmt: None,
            width: Some(160),
            text: Some("Timestamp".into()),
        });
        dv.insert_column(nwg::InsertListViewColumn {
            index: Some(2),
            fmt: None,
            width: Some(55),
            text: Some("Method".into()),
        });
        dv.insert_column(nwg::InsertListViewColumn {
            index: Some(3),
            fmt: None,
            width: Some(260),
            text: Some("Path".into()),
        });
        dv.insert_column(nwg::InsertListViewColumn {
            index: Some(4),
            fmt: None,
            width: Some(45),
            text: Some("Status".into()),
        });
        dv.insert_column(nwg::InsertListViewColumn {
            index: Some(5),
            fmt: None,
            width: Some(200),
            text: Some("Referer".into()),
        });
        dv.insert_column(nwg::InsertListViewColumn {
            index: Some(6),
            fmt: None,
            width: Some(350),
            text: Some("User Agent".into()),
        });
        dv.set_headers_enabled(true);
    }

    fn load_data(logs: &Logs, dv: &ListView);
}