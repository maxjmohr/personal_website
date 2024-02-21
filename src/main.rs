use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <nav>
                <a href="#aboutme">{"About Me"}</a>
                <a href="#profexper">{"Professional experience"}</a>
            </nav>
            <div id="aboutme" class="section" style="height: 100vh; background-color: lightblue;">
                <h2>{"About me"}</h2>
                <p>{"Hey, I'm Max."}</p>
                <p>{"I am currently studying data science."}</p>
            </div>
            <div id="profexper" class="section" style="height: 100vh; background-color: lightgreen;">
                <h2>{"Professional experience"}</h2>
                <p>{"Career steps go here"}</p>
            </div>
            <div class="bg-red-500 text-white p-4 mt-4" style="width: 90%;">
                {"This div uses Tailwind CSS classes!"}
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
