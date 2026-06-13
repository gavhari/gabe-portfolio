use crate::components::ui::{tags, Section};
use crate::data::EXPERIENCE;
use yew::prelude::*;

#[function_component(Experience)]
pub fn experience() -> Html {
    let items = EXPERIENCE
        .iter()
        .map(|j| {
            let bullets = j
                .bullets
                .iter()
                .map(|b| html! { <li>{ *b }</li> })
                .collect::<Html>();

            html! {
                <article class="job">
                    <div class="job-head">
                        <div class="job-title">
                            <span class="job-role">{ j.role }</span>
                            <span class="job-faint">{ " @ " }</span>
                            <span class="job-co">{ j.company }</span>
                        </div>
                        <div class="job-meta">{ j.period }{ " · " }{ j.location }</div>
                    </div>
                    <ul class="job-bullets">{ bullets }</ul>
                    <ul class="tags">{ tags(j.tech) }</ul>
                </article>
            }
        })
        .collect::<Html>();

    html! {
        <Section id="experience" command="cat experience.log" title="Experience">
            <div class="timeline">{ items }</div>
        </Section>
    }
}
