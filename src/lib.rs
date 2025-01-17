use extism_pdk::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
pub struct Stats {
    pub tag: String,
    pub name: String,
    pub leagueIcon: String,
}

impl Stats {
    fn to_html(&self) -> String {
        format!("
            <li>{}</li>
            <li>{}</li>
        ", self.name, self.tag)
    }
}

fn fetch_stats(api: &String) -> FnResult<Stats> {
    let req = HttpRequest{
        url: (&api).to_string(),
        method: Some("GET".to_string()),
        headers: BTreeMap::new(),
    };

    let res = http::request::<()>(&req, None)?;
    Ok(res.json::<Stats>()?)
}

#[plugin_fn]
pub fn ui() -> FnResult<String> {
    let api = String::from("https://api.bbki.ng/coc");
    let res = fetch_stats(&api);
    
    Ok(res?.to_html())
}


// start with something simple
#[plugin_fn]
pub fn coc() -> FnResult<String> {
    Ok(format!("Work In Progress"))
}
