use crate::components::ui::Section;
use crate::data::PROFILE;
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    let paras = PROFILE
        .about
        .iter()
        .map(|p| html! { <p>{ *p }</p> })
        .collect::<Html>();

    let facts = PROFILE
        .facts
        .iter()
        .map(|(k, v)| {
            html! {
                <li>
                    <span class="fact-k">{ *k }</span>
                    <span class="fact-v">{ *v }</span>
                </li>
            }
        })
        .collect::<Html>();

    html! {
        <Section id="about" command="cat about.md" title="About">
            <div class="about-grid">
                <div class="prose">{ paras }</div>
                <aside class="facts">
                    <div class="facts-head">{ "// quick facts" }</div>
                    <ul>{ facts }</ul>
                </aside>
            </div>
        </Section>
    }
}
