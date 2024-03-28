mod share;
mod entities;
mod features;
mod widgets;
mod pages;






#[yew::function_component]
fn App() -> yew::Html {
    yew::html! {
        <div style={"min-height: 100vh"}>
            <div class={yew::classes!("container", "bg-light", "bg-gradient")}>
                <pages::IndexPage/>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}