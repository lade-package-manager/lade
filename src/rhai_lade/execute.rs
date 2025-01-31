use rhai::Engine;

use crate::{crash, err, log};

use super::{envs, files, system};

pub fn execute_rhai(source: &str) {
    let mut engine = Engine::new();

    engine.register_fn("system", system::system);

    engine.register_fn("get_env", envs::get_env);

    engine.register_fn("set_env", envs::set_env);

    engine.register_fn("read_file", files::read_file);

    engine.register_fn("write_file", files::write_file);

    engine.run(source).unwrap_or_else(|e| {
        err!("Failed to execute rhai: {}", e);
        log!(
            format!("Failed to execute rhai: {}", e),
            "Failed to execute rhai"
        );
        crash!();
    });
}
