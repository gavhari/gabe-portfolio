use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello from Yew!" }</h1>
            <p>{ "This portfolio is hosted on GitHub Pages." }</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
