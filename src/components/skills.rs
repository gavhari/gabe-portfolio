use crate::components::ui::{tags, Section};
use crate::data::SKILLS;
use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    let groups = SKILLS
        .iter()
        .map(|g| {
            html! {
                <div class="skill-group">
                    <div class="skill-cat">{ g.category }{ "/" }</div>
                    <ul class="tags">{ tags(g.items) }</ul>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <Section id="skills" command="cat skills.json" title="Skills">
            <div class="skills">{ groups }</div>
        </Section>
    }
}
