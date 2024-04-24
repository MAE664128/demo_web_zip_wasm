pub struct IndexPage {}

impl yew::Component for IndexPage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }


    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        yew::html! {
            <>
                <div class={yew::classes!("row", "text-center", "border")}>
                    <h3>{"Create Zip archive"}</h3>
                    <p>{"A simple example program for creating ZIP archives running in the browser using WebAssembly."}</p>
                    <p>{"GitHub: "}<a href={"https://github.com/MAE664128/demo_web_zip_wasm"} target="_blank">{"mae664128/demo_web_zip_wasm"}</a></p>
                </div>
                <crate::widgets::file_selection_block::FileSelectionBlockComponent />
            </>
        }
    }
}