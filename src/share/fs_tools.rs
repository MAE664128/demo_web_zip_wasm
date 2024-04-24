//! Contains JS bindings for working with the file system through the browser.


/// File reading method.
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
                    format!("Error reading file {}", file_name),
                    format!("{}", err)
                ));
            }
        }
    })
}