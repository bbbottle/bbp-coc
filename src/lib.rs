use extism_pdk::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use leptos::*;
use leptos::prelude::*;
use leptos::tachys::dom::document;
use leptos::wasm_bindgen::JsCast;

#[component]
pub fn Hello() -> impl IntoView {
    view! {
        <div>
            <h1 id="test">Hello World</h1>
        </div>
    }
}

#[plugin_fn]
pub fn test() -> FnResult<()>  {
    let target_html_element = document().get_element_by_id("test").unwrap().unchecked_into();
    let _ = mount_to(target_html_element, || view!{
        <Hello />
    });

    Ok(())
}

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
pub struct Stats {
    pub tag: String,
    pub name: String,
    pub league_icon: String,
    pub trophies: u32
}

#[host_fn("extism:host/user")]
extern "ExtismHost" {
   fn loading(show: String);
}

impl Stats {
    fn to_html(&self) -> String {
        format!("
            <div>{}(<code>{})</code></div>
            <div class='flex'>
                {}
                <img width=28 height=28 cross-origin='anonymous' src={} />
            </div>
        ", self.name, self.tag, self.trophies, self.league_icon)
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
pub fn coc() -> FnResult<String> {
    Ok("Work In Progress".to_string())
}

#[plugin_fn]
pub fn coc_bkp() -> FnResult<String> {
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
