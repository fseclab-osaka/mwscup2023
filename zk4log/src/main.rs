use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_zk4log::Zk4log;

fn main() {
    serve_plugin(&mut Zk4log {}, MsgPackSerializer {})
}
