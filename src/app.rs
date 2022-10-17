use nwd::NwgUi;
use crate::models::Logs;
use crate::views::{ErrorsTab, LogsTab, SuspiciousTab, TabView};

#[derive(Default, NwgUi)]
pub struct App {
    #[nwg_resource(source_file: Some("icon.ico"))]
    icon: nwg::Icon,

    #[nwg_control(size: (1200, 700), title: "Apache Log Viewer", accept_files: true, center: true, icon: Some(&data.icon))]
    #[nwg_events(OnWindowClose: [App::exit], OnInit: [App::setup_view], OnFileDrop: [App::file_drop(SELF, EVT_DATA)], OnResize: [App::resize], OnWindowMaximize: [App::resize], OnWindowMinimize: [App::resize])]
    window: nwg::Window,

    #[nwg_resource(family: "Segoe UI", size: 15)]
    segoe: nwg::Font,

    #[nwg_control(parent: window, text: "&File")]
    menu_bar: nwg::Menu,

    #[nwg_control(parent: menu_bar, text: "&Open")]
    #[nwg_events(OnMenuItemSelected: [App::open_file])]
    menu_open: nwg::MenuItem,

    #[nwg_control(parent: menu_bar, text: "E&xit")]
    #[nwg_events(OnMenuItemSelected: [App::exit])]
    menu_exit: nwg::MenuItem,

    #[nwg_control(parent: window, font: Some(&data.segoe))]
    tabs_container: nwg::TabsContainer,

    #[nwg_control(parent: tabs_container, text: "Logs")]
    tab_all: nwg::Tab,

    #[nwg_control(parent: tabs_container, text: "Errors")]
    tab_errors: nwg::Tab,

    #[nwg_control(parent: tabs_container, text: "Suspicious")]
    tab_suspicious: nwg::Tab,

    #[nwg_control(item_count: 10, size: (500, 350), list_style: nwg::ListViewStyle::Detailed, focus: true,
    ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT, parent: tab_all)]
    logs_view: nwg::ListView,

    #[nwg_control(item_count: 10, size: (500, 350), list_style: nwg::ListViewStyle::Detailed, focus: true,
    ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT, parent: tab_errors)]
    errors_view: nwg::ListView,

    #[nwg_control(item_count: 10, size: (500, 350), list_style: nwg::ListViewStyle::Detailed, focus: true,
    ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT, parent: tab_suspicious)]
    suspicious_view: nwg::ListView,
}

impl App {
    fn setup_view(&self) {
        LogsTab::setup_view(&self.logs_view);
        ErrorsTab::setup_view(&self.errors_view);
        SuspiciousTab::setup_view(&self.suspicious_view);
        self.resize();
    }

    fn load_data(&self, path: &str) {
        let logs = Logs::load_file(path);
        if logs.is_ok() {
            let logs = logs.unwrap();
            LogsTab::load_data(&logs, &self.logs_view);
            ErrorsTab::load_data(&logs, &self.errors_view);
            SuspiciousTab::load_data(&logs, &self.suspicious_view);
        } else {
            nwg::modal_error_message(&self.window, "Error", logs.err().unwrap().as_str());
        }
    }

    fn open_file(&self) {
        let mut file_dialog = Default::default();
        nwg::FileDialog::builder()
            .title("Open Log File")
            .action(nwg::FileDialogAction::Open)
            .multiselect(false)
            .filters("Log files(*.txt;*.log)|All files (*.*)")
            .build(&mut file_dialog)
            .unwrap();

        file_dialog.run(Some(&self.window));

        let result = file_dialog.get_selected_item();
        if result.is_ok() {
            let path = result.unwrap();
            self.load_data(path.to_str().unwrap());
        }
    }

    fn file_drop(&self, data: &nwg::EventData) {
        let drop = data.on_file_drop();
        let file = &drop.files()[0];
        self.load_data(file);
    }

    fn resize(&self) {
        let (w, h) = self.window.size();
        self.tabs_container.set_size(w, h - 2);
        self.logs_view.set_size(w - 13, h - 41);
        self.errors_view.set_size(w - 13, h - 41);
        self.suspicious_view.set_size(w - 13, h - 41);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}