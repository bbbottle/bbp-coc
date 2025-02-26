use extism_pdk::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
pub struct Stats {
    pub tag: String,
    pub name: String,
    pub created_at: string,
    pub trophies: u32,
}

#[host_fn("extism:host/user")]
extern "ExtismHost" {
    fn loading(show: String);
}

impl Stats {
    fn to_html(&self) -> String {
        format!(
            "<div>{}(<code>{}</code>)</div>",
            self.created_at, self.trophies
        )
    }
}

fn fetch_stats(api: &String) -> FnResult<Vec<Stats>> {
    let req = HttpRequest{
        url: api.to_string(),
        method: Some("GET".to_string()),
        headers: BTreeMap::new(),
    };

    let res = http::request::<()>(&req, None)?;
    Ok(res.json::<Vec<Stats>>()?)
}

#[plugin_fn]
pub fn coc() -> FnResult<String> {
    unsafe {
        let _ = loading("true".to_string());
    };
    let api = String::from("https://api.bbki.ng/coc");
    let res = fetch_stats(&api);

    let content: Vec<String> = res?.iter()
        .map(|r| r.to_html())
        .collect();
    unsafe {
        let _ = loading("false".to_string());
    };
    Ok(content.join("\r\n"))
}
