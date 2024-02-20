use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Max Mohr" }</h1>
            <div>
                <h2>{ "Hey, I'm Max." }</h2>
                <p>{ "I am currently studying data science." }</p>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
