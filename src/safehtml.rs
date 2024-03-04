use yew::{function_component, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap();

    // Set the inner HTML
    div.set_inner_html(&props.html);

    // Apply the provided class if it exists
    if let Some(class) = &props.class {
        div.set_attribute("class", class)
            .expect("Failed to set class attribute");
    }

    Html::VRef(div.into())
}
