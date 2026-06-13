use crate::data::{PROFILE, SOCIALS};
use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {
    let socials = SOCIALS
        .iter()
        .map(|s| {
            html! {
                <a class="chip" href={s.href} target="_blank" rel="noopener noreferrer">
                    { s.label }
                </a>
            }
        })
        .collect::<Html>();

    let resume = PROFILE
        .resume_url
        .map(|u| {
            html! {
                <a class="btn btn-ghost" href={u} target="_blank" rel="noopener noreferrer">
                    { "download résumé" }
                </a>
            }
        })
        .unwrap_or_default();

    html! {
        <section id="top" class="hero">
            <div class="term">
                <div class="term-bar">
                    <span class="dot red"></span>
                    <span class="dot amber"></span>
                    <span class="dot green"></span>
                    <span class="term-title">{ "gabe@portfolio: ~" }</span>
                </div>
                <div class="term-body">
                    <p class="line"><span class="prompt">{ "$" }</span>{ " whoami" }</p>
                    <p class="out name">
                        { PROFILE.name }{ " — " }<span class="role">{ PROFILE.role }</span>
                    </p>
                    <p class="line"><span class="prompt">{ "$" }</span>{ " cat tagline.txt" }</p>
                    <p class="out">{ PROFILE.tagline }</p>
                    <p class="line"><span class="prompt">{ "$" }</span>{ " ./socials --list" }</p>
                    <div class="out chips">{ socials }</div>
                    <p class="line">
                        <span class="prompt">{ "$" }</span>
                        <span class="cursor" aria-hidden="true"></span>
                    </p>
                </div>
            </div>
            <div class="hero-cta">
                <a class="btn btn-primary" href="#projects">{ "view projects ↓" }</a>
                { resume }
            </div>
        </section>
    }
}
