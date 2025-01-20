// HTML content for the UI
const HTML_CONTENT: &str = r#"
<div id="extism-plugin-container">
    <h1>Hello, World!</h1>
    <button id="my-button">Click Me</button>
</div>
"#;

// JavaScript content for event handling
const JS_CONTENT: &str = r#"
document.getElementById('my-button').addEventListener('click', function() {
    console.log('Button clicked!');
});
"#;

// Generate the HTML and JavaScript content to inject into the specified target element
pub fn generate_ui() -> Result<String> {
    let content = format!(
        r#"
        <div id="test">
            {html_content}
            <script>
                {js_content}
            </script>
        </div>
        "#,
        html_content = HTML_CONTENT,
        js_content = JS_CONTENT
    );
    Ok(content)
}