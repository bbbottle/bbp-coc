use extism_pdk::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
pub struct Stats {
    pub tag: String,
    pub name: String,
    pub leagueIcon: String,
    pub trophies: u32
}

impl Stats {
    fn to_html(&self) -> String {
        format!("
            <div>{}({})</div>
            <div>
                {}
                <img width=24 cross-origin='anonymous' src={} />
            </div>
        ", self.name, self.tag, self.trophies, self.leagueIcon)
    }
}

fn fetch_stats(api: &String) -> FnResult<Vec<Stats>> {
    let req = HttpRequest{
        url: (&api).to_string(),
        method: Some("GET".to_string()),
        headers: BTreeMap::new(),
    };

    let res = http::request::<()>(&req, None)?;
    Ok(res.json::<Vec<Stats>>()?)
}

#[plugin_fn]
pub fn ui() -> FnResult<String> {
    let api = String::from("https://api.bbki.ng/coc");
    let res = fetch_stats(&api);

    let content: Vec<String> = res?.iter()
            .map(|r| r.to_html())
            .collect();
    
    Ok(content.join("\r\n"))
}


// start with something simple
#[plugin_fn]
pub fn coc() -> FnResult<String> {
    Ok(format!("Work In Progress"))
}
