use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_nopen::Nopen;

fn main() {
    serve_plugin(&mut Nopen {}, MsgPackSerializer {})
}
