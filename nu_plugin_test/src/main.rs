use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_test::Test;

fn main() {
    serve_plugin(&mut Test, MsgPackSerializer)
}
