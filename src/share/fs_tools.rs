//! Содержит обвязки к JS для работы с файловой системой через браузер.


/// Метод чтения файлов.
pub fn read_file(
    current_ind_file: usize,
    file_name: yew::AttrValue,
    file: &gloo_file::File,
    callback_loaded_file: yew::Callback<(usize, yew::AttrValue, Vec<u8>)>,
    callback_filed: yew::Callback<(String, String)>,
) -> gloo_file::callbacks::FileReader {
    gloo_file::callbacks::read_as_bytes(file, move |res| {
        match res {
            Ok(res) => { callback_loaded_file.emit((current_ind_file, file_name, res)); }
            Err(err) => {
                callback_filed.emit((
                    format!("Ошибка чтения файла {}", file_name),
                    format!("{}", err)
                ));
            }
        }
    })
}