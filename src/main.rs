mod components;
mod data;

use components::{About, Contact, Experience, Footer, Hero, Nav, Projects, Skills};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Nav />
            <main>
                <Hero />
                <About />
                <Projects />
                <Experience />
                <Skills />
                <Contact />
            </main>
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}