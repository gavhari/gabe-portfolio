use yew::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <header class="nav">
            <a class="brand" href="#top" aria-label="Home">
                <span class="brand-user">{ "gabe" }</span>
                <span class="brand-faint">{ "@" }</span>
                <span class="brand-host">{ "portfolio" }</span>
                <span class="brand-faint">{ ":~$" }</span>
            </a>
            <nav class="nav-links" aria-label="Primary">
                <a href="#about">{ "about" }</a>
                <a href="#projects">{ "projects" }</a>
                <a href="#experience">{ "work" }</a>
                <a href="#skills">{ "skills" }</a>
                <a href="#contact">{ "contact" }</a>
            </nav>
        </header>
    }
}
