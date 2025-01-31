use rhai::Engine;

use crate::{crash, err, log};

pub fn execute_rhai(source: &str){
    let engine = Engine::new();


    engine.run(source).unwrap_or_else(|e| {
        err!("Failed to execute rhai: {}", e);
        log!(format!("Failed to execute rhai: {}", e), "Failed to execute rhai");
        crash!();
    });


}
