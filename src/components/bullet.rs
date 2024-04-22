use crate::components::icons::Icons;
use yew::prelude::*;

// Properties to get content and return as bullet for list
#[derive(Debug, PartialEq, Properties)]
pub struct BulletProps {
    pub content: String,
    #[prop_or_default]
    pub scale: Option<u8>,
}

#[function_component]
pub fn Bullet(props: &BulletProps) -> Html {
    // Split content string using semicolon as separator
    let content: Vec<&str> = props.content.split(';').map(|s| s.trim()).collect();

    // Scale if necessary
    let scale = props.scale.unwrap_or(100);

    // Generate list items for each content string
    let list_items: Vec<Html> = content
        .iter()
        .map(|&item| {
            html! {
                <li class="flex items-start space-x-1.5">
                    <Icons icons={vec!["bullet".to_string()]} scale={scale}/>
                    <span class="leading-tight lg:leading-snug">{ item }</span>
                </li>
            }
        })
        .collect();

    // If content found, render
    if !content.is_empty() {
        html! {
            <ul class="space-y-2 text-left">{ list_items }</ul>
        }
    } else {
        html! {<div />}
    }
}
