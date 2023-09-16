use nu_plugin::{serve_plugin, MsgPackSerializer};
user nu_plugin_zk4log::Zk4log;

fn main() {
    serve_pugin(&mut Zk4log {}, MsgPackSerializer {})
}