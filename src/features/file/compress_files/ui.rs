use yew::TargetCast;
use crate::entities::file::model::InfoAboutSelectedFile;
use crate::features::file::compress_files;
use crate::features::file::compress_files::model::CompressingState;

pub enum CompressionFilesMsg {
    StartCompression,
    /// Файл прочитан с диска.
    LoadedFile(usize, yew::AttrValue, Vec<u8>),
    ProgressUpdateCompression {
        total_size: usize,
    },
    SuccessfulCompression(Vec<u8>),
    FailedCompression((String, String)),
    /// Пароль из поля ввода.
    EditPassword(String),
}

#[derive(yew::Properties, PartialEq)]
pub struct CompressionFilesProps {
    /// Обратный вызов, когда компрессия была выполнена успешно.
    pub on_start_compress: yew::Callback<()>,
    /// Обратный вызов с индексом успешно компрессированного файла.
    pub on_add_success_compress_file: yew::Callback<usize>,
    pub files: std::collections::HashMap<usize, std::rc::Rc<InfoAboutSelectedFile>>,
}

pub struct CompressionFilesComponent {
    password: String,
    is_blocked: bool,
    compressor: compress_files::model::CompressionFiles,
    file_reading_tasks: std::collections::HashMap<yew::AttrValue, gloo_file::callbacks::FileReader>,
    number_of_successfully_processed_files: usize,
    total_size: usize,
    err_msg: (String, String),
    blob_result: Option<gloo_file::Blob>,
}

impl CompressionFilesComponent {
    fn get_obj_url_with_result(&self) -> Option<String> {
        let blob = self.blob_result.as_ref()?;
        let download_url = web_sys::Url::create_object_url_with_blob(blob.as_ref()).ok()?;
        Some(download_url)
    }
}

impl yew::Component for CompressionFilesComponent {
    type Message = CompressionFilesMsg;
    type Properties = CompressionFilesProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let password = String::new();
        Self {
            password: password.clone(),
            is_blocked: false,
            compressor: compress_files::model::CompressionFiles::new(password),
            file_reading_tasks: std::collections::HashMap::new(),
            number_of_successfully_processed_files: 0,
            total_size: 0,
            err_msg: ("".to_string(), "".to_string()),
            blob_result: None,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CompressionFilesMsg::EditPassword(edit_value) => {
                self.password = edit_value.trim().to_string();
                true
            }

            CompressionFilesMsg::StartCompression => {
                ctx.props().on_start_compress.emit(());

                // Разблокируем работу компрессора.
                self.is_blocked = false;
                // Обнуляем компрессор.
                self.compressor = compress_files::model::CompressionFiles::new(self.password.clone());

                let callback_loaded_file = ctx.link()
                    .callback(
                        |(current_ind_file, file_name, byte_data)| CompressionFilesMsg::LoadedFile(current_ind_file, file_name, byte_data)
                    );
                let callback_filed = ctx.link().callback(CompressionFilesMsg::FailedCompression);
                self.compressor.change_state_on_in_process();

                for (current_ind_file, file) in ctx.props().files.iter() {
                    let file = file.clone();
                    if self.is_blocked { break; }
                    let file_name = file.file_name.clone();
                    let task = crate::share::fs_tools::read_file(
                        *current_ind_file,
                        file_name.clone(),
                        &file.js_file_obj,
                        callback_loaded_file.clone(),
                        callback_filed.clone(),
                    );
                    self.file_reading_tasks.insert(file_name, task);
                }
                true
            }
            CompressionFilesMsg::LoadedFile(current_ind_file, file_name, data) => {
                // Файл успешно прочитан с диска в память.
                // Добавляем файл в компрессор
                if self.is_blocked { return true; }
                if self.compressor.need_to_wait {
                    ctx.link().send_message(
                        CompressionFilesMsg::LoadedFile(current_ind_file, file_name, data)
                    );
                    return true;
                }
                match self.compressor.add_file_in_zip(
                    &file_name,
                    &data,
                ) {
                    Ok(total_size) => {
                        ctx.props().on_add_success_compress_file.emit(current_ind_file);
                        ctx.link().send_message(
                            CompressionFilesMsg::ProgressUpdateCompression {
                                total_size
                            }
                        );
                    }
                    Err((title, detail)) => {
                        ctx.link().send_message(
                            CompressionFilesMsg::FailedCompression((title, detail))
                        );
                    }
                };
                self.file_reading_tasks.remove(&file_name);
                true
            }
            CompressionFilesMsg::ProgressUpdateCompression { total_size } => {
                self.total_size = self.total_size.saturating_add(total_size);
                self.number_of_successfully_processed_files = self.number_of_successfully_processed_files.saturating_add(1);
                let flag1 = self.file_reading_tasks.is_empty();
                let flag2 = self.number_of_successfully_processed_files >= ctx.props().files.len();
                if flag1 && flag2 {
                    match self.compressor.finish() {
                        Ok(res) => {
                            ctx.link().send_message(
                                CompressionFilesMsg::SuccessfulCompression(res)
                            );
                        }
                        Err(err) => {
                            ctx.link().send_message(
                                CompressionFilesMsg::FailedCompression(err)
                            );
                        }
                    }
                }
                true
            }
            CompressionFilesMsg::FailedCompression(err) => {
                self.is_blocked = true;
                self.err_msg = err;
                self.file_reading_tasks = std::collections::HashMap::new();
                true
            }
            CompressionFilesMsg::SuccessfulCompression(data) => {
                self.total_size = data.len();
                self.blob_result = Some(gloo_file::Blob::new(data.as_slice()));
                true
            }
        }
    }
    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let edit = move |input: web_sys::HtmlInputElement| {
            let value = input.value();
            input.set_value("");
            CompressionFilesMsg::EditPassword(value)
        };

        let onblur = &ctx.link().callback(move |e: web_sys::FocusEvent| edit(e.target_unchecked_into()));

        let onkeypress = &ctx.link().batch_callback(move |e: web_sys::KeyboardEvent| {
            (e.key() == "Enter").then(|| edit(e.target_unchecked_into()))
        });

        let start_onclick = &ctx.link().callback(|_| CompressionFilesMsg::StartCompression);
        match &self.compressor.state {
            CompressingState::WaitStart => {
                if ctx.props().files.is_empty() {
                    yew::html! {
                        <p>{"Выберете файлы..."}</p>
                    }
                } else {
                    yew::html! {
                        <div class="input-group mb-3">
                            <input
                                class={yew::classes!("form-control")}
                                type="text"
                                placeholder="Укажите пароль" aria-label="Укажите пароль"
                                aria-describedby="button-compress"
                                value={self.password.clone()}
                                {onblur}
                                {onkeypress}
                            />
                            <button
                                id="button-compress"
                                type="button"
                                class={yew::classes!("btn", "btn-outline-dark")}
                                data-bs-toggle="tooltip"
                                data-bs-placement="bottom"
                                title="Запустить процесс сжатия файлов."
                                onclick={start_onclick}
                            >
                                {"Упаковать"}
                            </button>
                        </div>
                    }
                }
            }
            CompressingState::InProcess => {
                let progress = self.number_of_successfully_processed_files as f32;
                let progress = progress / (ctx.props().files.len() as f32);
                let progress = (progress * 100_f32) as u32;
                let total_size = crate::share::size_to_string(self.total_size as f64);

                yew::html! {
                    <div class={yew::classes!("row", "flex-fill")}>
                        <div class={yew::classes!("col")}>
                            <div class={yew::classes!("progress")}>
                              <div
                                class={yew::classes!("progress-bar")}
                                role={"progressbar" }
                                style={format!("width: {}%", progress)}
                                aria-valuenow={format!("{}", progress)}
                                aria-valuemin={format!("{}", progress)}
                                aria-valuemax={"100"}></div>
                            </div>
                        </div>
                        <div class={yew::classes!("col")}>
                            <div>{format!("Упаковано: {total_size}")}</div>
                        </div>
                    </div>
                }
            }
            CompressingState::Done => {
                let total_size = crate::share::size_to_string(self.total_size as f64);
                if let Some(href) = self.get_obj_url_with_result() {
                    yew::html! {
                    <a
                        class={yew::classes!("btn", "btn-outline-dark")}
                        href={href}
                        download={"compressed.zip"}
                    >
                        {format!("Скачать архив: {total_size}")}
                    </a>
                }
                } else {
                    yew::html! {
                    <p>{"Ой, файл не доступен"}</p>
                }
                }
            }
            CompressingState::Fail => {
                let msg = self.err_msg.0.as_str();
                let detail = self.err_msg.1.as_str();
                yew::html! {
                    <div
                    class={yew::classes!("alert", "alert-danger")}
                    data-bs-toggle={"tooltip"} data-bs-placement={"top"}
                    title={detail.to_string()}
                    >{format!("Ошибка: {msg}")}
                    </div>
                }
            }
        }
    }
}