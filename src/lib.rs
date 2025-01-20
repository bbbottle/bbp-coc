use extism_pdk::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use js_sys::Error as JsError;
use std::collections::BTreeMap;
use web_sys::{Element, HtmlElement, Window, Document};

// Structure to hold UI state
struct PluginUI {
    counter: i32,
}

pub struct Plugin {
    ui: PluginUI,
}

impl Plugin {
    pub fn new() -> Self {
        Self {
            ui: PluginUI { counter: 0 },
        }
    }

    // Initialize the UI
    pub fn init_ui(&self) -> Result<(), extism_pdk::Error> {
        // Get the window object
        let window = web_sys::window().ok_or_else(|| extism_pdk::Error::Other("no global window exists".into()))?;
        let document = window.document().ok_or_else(|| extism_pdk::Error::Other("no document exists".into()))?;

        // Create our UI container
        let container = document.create_element("div").map_err(js_to_extism_error)?;
        container.set_id("extism-plugin-container");

        // Create counter display
        let counter_display = document.create_element("div").map_err(js_to_extism_error)?;
        counter_display.set_id("counter-display");
        counter_display.set_text_content(Some(&format!("Count: {}", self.ui.counter)));

        // Create increment button
        let button = document.create_element("button").map_err(js_to_extism_error)?;
        button.set_inner_html("Increment");

        // Add click event listener
        let click_handler = Closure::wrap(Box::new(move || {
            if let Some(display) = document.get_element_by_id("counter-display") {
                let current = display.text_content()
                    .unwrap_or_default()
                    .replace("Count: ", "")
                    .parse::<i32>()
                    .unwrap_or(0);
                display.set_text_content(Some(&format!("Count: {}", current + 1)));
            }
        }) as Box<dyn FnMut()>);

        button.add_event_listener_with_callback(
            "click",
            click_handler.as_ref().unchecked_ref(),
        ).map_err(js_to_extism_error)?;

        // Keep the closure alive
        click_handler.forget();

        // Append elements
        container.append_child(&counter_display).map_err(js_to_extism_error)?;
        container.append_child(&button).map_err(js_to_extism_error)?;

        // Append to document body
        document.body()
            .ok_or_else(|| extism_pdk::Error::Other("document should have a body".into()))?
            .append_child(&container)
            .map_err(js_to_extism_error)?;

        Ok(())
    }
}

fn js_to_extism_error(js_value: JsValue) -> extism_pdk::Error {
    extism_pdk::Error::Other(js_value.as_string().unwrap_or_else(|| "Unknown error".to_string()))
}

// Export the plugin
#[plugin_fn]
pub fn init() -> FnResult<()> {
    let plugin = Plugin::new();
    plugin.init_ui()?;
    Ok(())
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
