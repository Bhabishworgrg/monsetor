use x11rb::rust_connection::RustConnection;
use x11rb::connection::Connection;
use x11rb::protocol::randr;

fn main() {
    let (conn, screen_num) = RustConnection::connect(None)
                                .expect("Connection failed");
    let screen = &conn.setup().roots[screen_num];

    let monitors = randr::get_monitors(&conn, screen.root, true)
                        .expect("Unable to get monitors")
                        .reply()
                        .expect("Unable to get monitors");

    for monitor in monitors.monitors {
        for &output in &monitor.outputs {
            let info = randr::get_output_info(&conn, output, 0)
                            .expect("Unable to get output info")
                            .reply()
                            .expect("Unable to get output info");
            let name = String::from_utf8_lossy(&info.name);
            println!("{name}");
        }
    }
}
