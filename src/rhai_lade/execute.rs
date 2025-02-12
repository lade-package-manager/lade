use rhai::Engine;
use rhai::Shared;
use std::cell::RefCell;
use std::error::Error;

use super::path;
use super::{
    envs,
    files::{self, RFile},
    system,
};

pub fn execute_rhai(source: &str) -> Result<(), Box<dyn Error>> {
    let mut engine = Engine::new();

    engine.register_fn("system", system::system_rhai);

    engine.register_fn("get_env", envs::get_env);

    engine.register_fn("set_env", envs::set_env);

    // register `RFile` structs
    engine
        .register_type::<files::RFile>()
        .register_fn("open_file", files::open_file_share)
        .register_fn(
            "write",
            |file: &mut Shared<RefCell<RFile>>, content: &str| {
                file.borrow_mut().write(content);
            },
        )
        .register_fn("clear", |file: &mut Shared<RefCell<RFile>>| {
            file.borrow_mut().clear();
        })
        .register_fn(
            "read_to_string",
            |file: &mut Shared<RefCell<RFile>>| -> String { RFile::read_to_string(file) },
        );

    // register `RPath` structs
    engine
        .register_type::<path::RPath>()
        .register_fn("path", path::path)
        .register_fn("to_string", path::RPath::to_string)
        .register_fn("file_name", path::RPath::file_name)
        .register_fn("exists", path::RPath::exists)
        .register_fn("read_file", path::RPath::read_file);

    engine.run(source)?;
    Ok(())
}
