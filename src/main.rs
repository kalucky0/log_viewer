extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use log_viewer::App;
use nwg::NativeUi;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = App::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}