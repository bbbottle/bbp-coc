use extism_pdk::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use extism_pdk::*;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlElement, Window, Document};
use js_sys::Function;

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
            ui: PluginUI { counter: 0 }
        }
    }

    // Initialize the UI
    pub fn init_ui(&self) -> Result<(), Error> {
        // Get the window object
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("no document exists");

        // Create our UI container
        let container = document.create_element("div")?;
        container.set_id("extism-plugin-container");
        
        // Create counter display
        let counter_display = document.create_element("div")?;
        counter_display.set_id("counter-display");
        counter_display.set_text_content(Some(&format!("Count: {}", self.ui.counter)));

        // Create increment button
        let button = document.create_element("button")?;
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
        )?;
        
        // Keep the closure alive
        click_handler.forget();

        // Append elements
        container.append_child(&counter_display)?;
        container.append_child(&button)?;
        
        // Append to document body
        document.body()
            .expect("document should have a body")
            .append_child(&container)?;

        Ok(())
    }
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
