use crate::share;

/// Information about a file on the local computer selected by the user.
#[derive(yew::Properties, PartialEq, Clone)]
pub struct InfoAboutSelectedFile {
    /// User-specified file name on disk.
    pub(crate) file_name: yew::virtual_dom::AttrValue,
    /// A string containing the date the file was last modified.
    pub(crate) last_modified: yew::virtual_dom::AttrValue,
    /// File type
    pub(crate) file_type: yew::virtual_dom::AttrValue,
    /// A string containing the file size.
    pub(crate) file_size: yew::virtual_dom::AttrValue,
    /// File size in bytes.
    pub(crate) raw_size: u64,
    pub(crate) js_file_obj: gloo_file::File,
}


impl InfoAboutSelectedFile {
    pub fn from_js_file(file: gloo_file::File) -> Self {
        let last_modified: chrono::DateTime<chrono::Utc> = file.last_modified_time().into();
        let last_modified = last_modified.to_rfc3339()[0..10].to_string();
        let file_type = file.raw_mime_type();
        let file_name = file.name();
        let file_size = share::size_to_string(file.size() as f64);


        Self {
            file_name: yew::virtual_dom::AttrValue::from(file_name.clone()),
            last_modified: yew::virtual_dom::AttrValue::from(last_modified),
            file_type: yew::virtual_dom::AttrValue::from(file_type.clone()),
            file_size: yew::virtual_dom::AttrValue::from(file_size),
            raw_size: file.size(),
            js_file_obj: file,
        }
    }
}

