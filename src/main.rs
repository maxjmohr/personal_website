use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <aside class="sidebar">
              <ul class="links">
                <li>
                  <span class="material-symbols-outlined">{"aboutme"}</span>
                  <a href="#">{"About Me"}</a>
                </li>
                <li>
                  <span class="material-symbols-outlined">{"profexper"}</span>
                  <a href="#">{"Professional experience"}</a>
                </li>
                <li>
                  <span class="material-symbols-outlined">{"educ"}</span>
                  <a href="#">{"Education"}</a>
                </li>
                <li>
                  <span class="material-symbols-outlined">{"projects"}</span>
                  <a href="#">{"Projects"}</a>
                </li>
                <li>
                  <span class="material-symbols-outlined">{"techskills"}</span>
                  <a href="#">{"Technical skills"}</a>
                </li>
                <li>
                  <span class="material-symbols-outlined">{"langs"}</span>
                  <a href="#">{"Languages"}</a>
                </li>
              </ul>
            </aside>
            <div id="aboutme" class="bg-blue-500 text-white p-4 mt-4 float-right" style="width: 90%; height: 100vh">
                <h2>{"About me"}</h2>
                <p>{"Hey, I'm Max."}</p>
                <p>{"I am currently studying data science."}</p>
            </div>
            <div id="profexper" class="bg-green-500 text-white p-4 mt-4 float-right" style="width: 90%; height: 100vh">
                <h2>{"Professional experience"}</h2>
                <p>{"Career steps go here"}</p>
            </div>
            <div class="bg-red-500 text-white p-4 mt-4 float-right" style="width: 90%; height: 100vh">
                {"This div uses Tailwind CSS classes!"}
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
