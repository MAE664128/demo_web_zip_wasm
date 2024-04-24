use yew::Context;
use crate::{entities, features, share};

pub enum FileSelectionBlockMsg {
    /// New file list received.
    NewFileList(gloo_file::FileList),
    RemoveFile(usize),
    /// Message about the need to block the interface
    NeedToBlock,
    /// Message with the index of the compressed file.
    SuccessCompressFile(usize),
}

/// Component - an area for adding and compressing files.
pub struct FileSelectionBlockComponent {
    files: std::collections::HashMap<usize, std::rc::Rc<entities::file::model::InfoAboutSelectedFile>>,
    need_to_block_action: bool,
    list_success_ind: Vec<usize>,
}

impl yew::Component for FileSelectionBlockComponent {
    type Message = FileSelectionBlockMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            files: std::collections::HashMap::new(),
            need_to_block_action: false,
            list_success_ind: vec![],
        }
    }



    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FileSelectionBlockMsg::SuccessCompressFile(ind) => {
                self.list_success_ind.push(ind);
                true
            }
            FileSelectionBlockMsg::NeedToBlock => {
                self.need_to_block_action = true;
                true
            }
            FileSelectionBlockMsg::NewFileList(file_list) => {
                let count = self.files.len();
                for ind in 0..file_list.len() {
                    if let Some(file) = file_list.get(ind) {
                        self.files.insert(
                            count + ind,
                            std::rc::Rc::from(
                                entities::file::model::InfoAboutSelectedFile::from_js_file(file.clone())
                            ),
                        );
                    }
                }
                true
            }
            FileSelectionBlockMsg::RemoveFile(ind) => {
                self.files.remove(&ind);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {

        let on_files_selection = &ctx.link().callback(FileSelectionBlockMsg::NewFileList);
        let on_file_remove = &ctx.link().callback(FileSelectionBlockMsg::RemoveFile);
        let on_start_compress = &ctx.link().callback(|_| FileSelectionBlockMsg::NeedToBlock);
        let on_add_success_compress_file = &ctx.link().callback(FileSelectionBlockMsg::SuccessCompressFile);
        let size = self.files.values().map(|info_file| {
            info_file.raw_size as f64
        }).reduce(|acc, e| acc + e).map(share::size_to_string).unwrap_or("???".to_string());

        yew::html! {
            <>
                if !self.need_to_block_action {
                    <features::file::add_files::AddFileRowsComponent {on_files_selection} />
                } else {
                    <div class={yew::classes!("row", "border")}>

                    </div>
                }
                <div class={yew::classes!("row", "px-2", "pt-2", "border-start", "border-end")}>
                    <div class={yew::classes!("list-group", "list-group-numbered", "overflow-auto", "pe-0")} style="max-height: 25vh;">
                        { self.files.iter().map(|(ind, info_file)| {
                            Self::view_file_row(
                                *ind,
                                info_file,
                                on_file_remove,
                                self.need_to_block_action,
                                self.list_success_ind.contains(ind)
                            )
                        }).collect::<yew::Html>() }
                    </div>
                </div>
                if !self.files.is_empty() {
                <div class={yew::classes!("row", "border", "rounded-bottom", "border-top-0", "py-2")}>
                    <div class={yew::classes!("col", "col-4")}>{format!("Total size: {}",size)}</div>
                    <div class={yew::classes!("col", "col-8", "d-flex", "justify-content-end")}>
                        <features::file::compress_files::CompressionFilesComponent files={self.files.clone()} on_start_compress={on_start_compress} on_add_success_compress_file={on_add_success_compress_file}/>
                    </div>

                </div>
                }

            </>
        }
    }
}

impl FileSelectionBlockComponent {
    fn view_file_row(
        ind: usize,
        file: &entities::file::model::InfoAboutSelectedFile,
        on_file_remove: &yew::Callback<usize>,
        need_to_block_action: bool,
        is_success_compress: bool,
    ) -> yew::Html {

        let classes = if is_success_compress {
            yew::classes!("list-group-item", "p-0", "text-success")
        } else {
            yew::classes!("list-group-item", "p-0")
        };

        yew::html! {
            <div class={classes} aria-current="true">
                <div class={yew::classes!("position-relative", "p-2")}>
                    <entities::file::ui::SelectedFileFileRowComponent info_about_selected_file={(*file).clone()} />
                    if !need_to_block_action {
                        <features::file::delete_file::DeleteFileRowComponent ind={ind} on_clicked={on_file_remove} />
                    }
                </div>
            </div>

        }
    }
}