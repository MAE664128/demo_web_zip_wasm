use yew::TargetCast;


pub enum AddFileRowsMsg {
    AddList(gloo_file::FileList),
    Pass,
}

#[derive(yew::Properties, PartialEq)]
pub struct AddFileRowsProps {
    /// Callback when files have been selected by the user.
    pub on_files_selection: yew::Callback<gloo_file::FileList>,
}


pub struct AddFileRowsComponent;

impl yew::Component for AddFileRowsComponent {
    type Message = AddFileRowsMsg;
    type Properties = AddFileRowsProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AddFileRowsMsg::AddList(fl) => {
                ctx.props().on_files_selection.emit(fl);
                false
            }
            AddFileRowsMsg::Pass => { false }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {


        let ondrop_selected_files_callback = &ctx.link().callback(|event: web_sys::DragEvent| {
            event.prevent_default();
            if let Some(dt) = event.data_transfer() {
                if let Some(fl) = dt.files() {
                    AddFileRowsMsg::AddList(fl.into())
                } else { AddFileRowsMsg::Pass }
            } else { AddFileRowsMsg::Pass }
        });
        let onchange_selected_files_callback = &ctx.link().callback(move |e: web_sys::Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Some(fl) = input.files() {
                AddFileRowsMsg::AddList(fl.into())
            } else { AddFileRowsMsg::Pass }
        });
        yew::html! {
            <div id="wrapper" class={yew::classes!("row", "border")}>
                <div
                    class={
                        yew::classes!("col", "m-2","p-2", "d-flex", "flex-column", "justify-content-center", "align-items-center")
                    }
                    style="border: 2px #000 dashed;"
                    id="drop-container"
                    ondrop={ondrop_selected_files_callback}
                    ondragover={yew::Callback::from(|event: web_sys::DragEvent| {
                        event.prevent_default();
                    })}
                    ondragenter={yew::Callback::from(|event: web_sys::DragEvent| {
                        event.prevent_default();
                    })}
                >
                    <div class={yew::classes!("fs-1", "mb-3")}>
                        <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="currentColor" class="bi bi-cloud-upload" viewBox="0 0 16 16">
                          <path fill-rule="evenodd" d="M4.406 1.342A5.53 5.53 0 0 1 8 0c2.69 0 4.923 2 5.166 4.579C14.758 4.804 16 6.137 16 7.773 16 9.569 14.502 11 12.687 11H10a.5.5 0 0 1 0-1h2.688C13.979 10 15 8.988 15 7.773c0-1.216-1.02-2.228-2.313-2.228h-.5v-.5C12.188 2.825 10.328 1 8 1a4.53 4.53 0 0 0-2.941 1.1c-.757.652-1.153 1.438-1.153 2.055v.448l-.445.049C2.064 4.805 1 5.952 1 7.318 1 8.785 2.23 10 3.781 10H6a.5.5 0 0 1 0 1H3.781C1.708 11 0 9.366 0 7.318c0-1.763 1.266-3.223 2.942-3.593.143-.863.698-1.723 1.464-2.383"/>
                          <path fill-rule="evenodd" d="M7.646 4.146a.5.5 0 0 1 .708 0l3 3a.5.5 0 0 1-.708.708L8.5 5.707V14.5a.5.5 0 0 1-1 0V5.707L5.354 7.854a.5.5 0 1 1-.708-.708z"/>
                        </svg>
                    </div>

                    <label for="file-upload">
                        <p>{"Drag and drop your files here or click to select them."}</p>
                    </label>
                    <p>{"or"}</p>
                    <span class="btn btn-outline-dark"
                          style="position: relative; overflow: hidden;"
                    >
                        {"Select file(s)"}
                        <input
                            style="position: absolute; top: 0; right: 0; min-width: 100%; min-height: 100%; font-size: 100px; text-align: right; filter: alpha(opacity=0); opacity: 0; outline: none; cursor: inherit; display: block;"
                            id="file-upload"
                            type="file"
                            multiple={true}
                            onchange={onchange_selected_files_callback}
                        />
                    </span>
                </div>
            </div>
        }
    }
}