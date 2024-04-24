use yew::{Context, Html};

#[derive(yew::Properties, PartialEq)]
pub struct DeleteFileRowProps {
    pub ind: usize,
    pub on_clicked: yew::Callback<usize>,
}
pub struct DeleteFileRowComponent;


impl yew::Component for DeleteFileRowComponent {
    type Message = ();
    type Properties = DeleteFileRowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current_ind = ctx.props().ind;
        let onclick = ctx.props().on_clicked.reform(move |_| current_ind);
        yew::html! {
            <button
                type="button"
                class={yew::classes!(
                    "position-absolute", "btn",
                    "top-0", "end-0",
                    "badge", "bg-secondary", "p-2"
                )}
                data-bs-toggle="tooltip"
                data-bs-placement="bottom"
                title="Remove current file from list"
                style="border-radius: unset;"
                {onclick}
            >
                {"✘"}
                <span class="visually-hidden">{"Delete"}</span>
            </button>
        }
    }
}