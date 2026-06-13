use crate::components::ui::{tags, Section};
use crate::data::PROJECTS;
use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    let cards = PROJECTS
        .iter()
        .map(|p| {
            let repo = p
                .repo
                .map(|r| html! {
                    <a class="proj-link" href={r} target="_blank" rel="noopener noreferrer">
                        { "code ↗" }
                    </a>
                })
                .unwrap_or_default();

            let demo = p
                .demo
                .map(|d| html! {
                    <a class="proj-link" href={d} target="_blank" rel="noopener noreferrer">
                        { "live ↗" }
                    </a>
                })
                .unwrap_or_default();

            html! {
                <article class="proj">
                    <div class="proj-top">
                        <span class="proj-path">
                            { "~/projects/" }<span class="proj-name">{ p.name }</span>
                        </span>
                        <span class="proj-year">{ p.year }</span>
                    </div>
                    <p class="proj-desc">{ p.description }</p>
                    <ul class="tags">{ tags(p.tech) }</ul>
                    <div class="proj-links">{ repo }{ demo }</div>
                </article>
            }
        })
        .collect::<Html>();

    html! {
        <Section id="projects" command="ls -la projects/" title="Projects">
            <div class="proj-grid">{ cards }</div>
        </Section>
    }
}
