use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_copy::Copy;

fn main() {
        serve_plugin(&mut Copy {}, MsgPackSerializer {})
}
