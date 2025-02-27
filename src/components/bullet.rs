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

    // Generate list items for each content string
    let list_items: Vec<Html> = content
        .iter()
        .map(|&item| {
            html! {
                <li class="ml-2 flex items-start justify-left">
                    <div class="mt-1 mr-1">
                        <Icons icons={vec!["bullet".to_string()]} />
                    </div>
                    <div>
                        <p class="text-xl/9 font-medium text-slate-600 dark:text-slate-400">{ item }</p>
                    </div>
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
