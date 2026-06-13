use crate::components::ui::Section;
use crate::data::{PROFILE, SOCIALS};
use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    let socials = SOCIALS
        .iter()
        .map(|s| {
            html! {
                <a class="btn btn-ghost" href={s.href} target="_blank" rel="noopener noreferrer">
                    { s.label }{ " ↗" }
                </a>
            }
        })
        .collect::<Html>();

    html! {
        <Section id="contact" command="./contact.sh" title="Contact">
            <div class="contact">
                <p class="contact-lead">{ PROFILE.status }{ "." }</p>
                <a class="contact-email" href={ format!("mailto:{}", PROFILE.email) }>
                    { PROFILE.email }
                </a>
                <div class="contact-btns">{ socials }</div>
            </div>
        </Section>
    }
}
