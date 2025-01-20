use extism_pdk::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use leptos::*;
#[component]
pub fn HelloButton() -> impl IntoView {
    view! {
        <div class="hello-container">
            <button 
                class="hello-btn"
            >
                "Hello World!"
            </button>
        </div>
    }
}


#[plugin_fn]
pub fn ui() -> FnResult<String> {
    
    // Create simple styles
    let styles = r#"
        <style>
            .hello-container {
                padding: 16px;
                font-family: -apple-system, system-ui, sans-serif;
            }
            .hello-btn {
                background: #0366d6;
                color: white;
                border: none;
                padding: 8px 16px;
                border-radius: 6px;
                cursor: pointer;
                font-size: 14px;
                transition: background 0.2s;
            }
            .hello-btn:hover {
                background: #0255b3;
            }
        </style>
    "#;

    // Create mount script

    // Render the component
    let html = leptos::ssr::render_to_string(|| view! { 
        <HelloButton /> 
    });

    Ok(format!(
        "{}{}",
        styles,
        html,
    ))
}

// ------------------old code------------------
#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
pub struct Stats {
    pub tag: String,
    pub name: String,
    pub league_icon: String,
    pub trophies: u32,
}

#[host_fn("extism:host/user")]
extern "ExtismHost" {
    fn loading(show: String);
}

impl Stats {
    fn to_html(&self) -> String {
        format!(
            "<div>{}(<code>{})</code></div>
            <div class='flex'>
                {}
                <img width=28 height=28 cross-origin='anonymous' src={} />
            </div>",
            self.name, self.tag, self.trophies, self.league_icon
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
