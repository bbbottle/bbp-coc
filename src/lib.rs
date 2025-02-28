use extism_pdk::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use chrono::{DateTime, Utc}; // 0.4.10
use axohtml::{html, text};
use axohtml::dom::DOMTree;

const HTML_TPL: &str = include_str!("./ui/index.html");


#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
pub struct Stats {
    pub tag: String,
    pub name: String,
    pub created_at: String,
    pub trophies: u32,
}

#[host_fn("extism:host/user")]
extern "ExtismHost" {
    fn loading(show: String);
}

impl Stats {
    fn get_time(&self) -> String {
        let date = self.created_at.parse::<DateTime<Utc>>().unwrap();
        date.format("%Y-%m-%d %H:%M:%S").to_string()
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

fn render_html(body: String) -> String {
    HTML_TPL.replace("{{content}}", &body)
}

fn render_stats_table(stats: Vec<Stats>) -> String {
    let doc_str: DOMTree<String> = html!(
        <table>
            <thead>
                <tr>
                    <th>"Trophies"</th>
                    <th>"Created At"</th>
                </tr>
            </thead>
            <tbody>
            {stats.iter().map(|stat| html!(
                <tr>
                    <td>{ text!(stat.trophies.to_string())}</td>
                    <td><small>{ text!(stat.get_time()) }</small></td>
                </tr>
            ))}
            </tbody>
        </table>
    );

    doc_str.to_string()
}

#[plugin_fn]
pub fn ui() -> FnResult<String> {
    let api = String::from("https://api.bbki.ng/coc");
    let _res = fetch_stats(&api);
    if _res.is_err() {
        return Ok("Failed to fetch data".to_string());
    }

    let table = render_stats_table(_res?);
    Ok(render_html(table))
}

#[plugin_fn]
pub fn coc() -> FnResult<String> {
    unsafe {
        let _ = loading("true".to_string());
    };

    let api = String::from("https://api.bbki.ng/coc");
    let _res = fetch_stats(&api);
    if _res.is_err() {
        return Ok("Failed to fetch data".to_string());
    }

    let table = render_stats_table(_res?);

    unsafe {
        let _ = loading("false".to_string());
    };

    Ok(table)
}
