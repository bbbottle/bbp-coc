use extism_pdk::*;
use serde::{Deserialize, Serialize};

// start with something simple
#[plugin_fn]
pub fn coc() -> FnResult<String> {
    Ok(format!("WIP"))
}
