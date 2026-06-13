//! Shared building blocks used across sections.

use yew::prelude::*;

/// A standard section: a terminal-style `$ command` heading, a title, then content.
#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub id: &'static str,
    pub command: &'static str,
    pub title: &'static str,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <section id={props.id} class="section">
            <div class="section-head">
                <span class="prompt">{ "$" }</span>
                <span class="cmd">{ props.command }</span>
            </div>
            <h2 class="section-title">{ props.title }</h2>
            { props.children.clone() }
        </section>
    }
}

/// Render a list of `<li class="tag">` items. Wrap in `<ul class="tags">`.
pub fn tags(items: &'static [&'static str]) -> Html {
    items
        .iter()
        .map(|t| html! { <li class="tag">{ *t }</li> })
        .collect::<Html>()
}