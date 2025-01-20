use leptos::*;

#[component]
pub fn HelloButton() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| {
        set_count.update(|n| *n + 1);
    };

    view! {
        <div class="hello-container">
            <button 
                class="hello-btn"
                on:click=on_click
            >
                "Hello World! (clicked " {count} " times)"
            </button>
        </div>
    }
}