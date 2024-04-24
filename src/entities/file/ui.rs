use yew::{Context, Html};
use crate::entities::file;

#[derive(yew::Properties, PartialEq, Clone)]
pub struct SelectedFileFileRowProps {
    pub(crate) info_about_selected_file: file::model::InfoAboutSelectedFile,
}

/// Component - A string containing information about the selected file.
pub struct SelectedFileFileRowComponent;

impl yew::Component for SelectedFileFileRowComponent {
    type Message = ();
    type Properties = SelectedFileFileRowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let file_name = &ctx.props().info_about_selected_file.file_name;
        let last_modified = &ctx.props().info_about_selected_file.last_modified;
        let file_size = &ctx.props().info_about_selected_file.file_size;
        yew::html! {
            <>
                <div class={yew::classes!("d-flex", "w-100", "justify-content-between", "align-items-center")}>
                    <h5 class={yew::classes!("mb-1", "text-truncate")}>{file_name}</h5>
                </div>
                <div class={yew::classes!("d-flex", "w-100", "justify-content-between", "align-items-center")}>
                    <small>{last_modified}</small>
                    <small>{file_size}</small>
                </div>

            </>
        }
    }
}