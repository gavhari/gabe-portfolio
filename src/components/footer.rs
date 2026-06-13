use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <span>{ "built with " }<span class="accent">{ "Rust + Yew + WebAssembly" }</span></span>
            <span class="sep">{ "·" }</span>
            <a href="https://github.com/gavhari/gabe-portfolio" target="_blank" rel="noopener noreferrer">
                { "source ↗" }
            </a>
            <span class="sep">{ "·" }</span>
            <span>{ "© 2026 Gabe Vahari" }</span>
        </footer>
    }
}
