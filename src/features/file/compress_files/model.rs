use std::io::Write;
use zip::unstable::write::FileOptionsExt;

#[derive(PartialEq, Copy, Clone)]
pub(crate) enum TypeEncryption {
    ZipCrypto,
    Aes256,
}

impl From<String> for TypeEncryption {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Aes256" => { Self::Aes256 }
            _ => { Self::ZipCrypto }
        }
    }
}

/// State of the compression process.
#[derive(PartialEq)]
pub(crate) enum CompressingState {
    /// Waiting for the operation to start.
    WaitStart,
    /// In progress.
    InProcess,
    /// Process completed.
    Done,
    /// The process did not complete successfully.
    Fail,
}

pub(crate) struct CompressionFiles {
    password: String,
    type_encryption: TypeEncryption,
    zip_writer: Option<zip::ZipWriter<std::io::Cursor<Vec<u8>>>>,
    pub state: CompressingState,
    pub need_to_wait: bool,
}


impl CompressionFiles {
    pub fn new(password: String, type_encryption: TypeEncryption) -> Self {
        Self {
            password,
            type_encryption,
            zip_writer: Some(zip::ZipWriter::new(std::io::Cursor::new(vec![]))),
            state: CompressingState::WaitStart,
            need_to_wait: false,
        }
    }

    pub fn change_state_on_in_process(&mut self) {
        self.state = CompressingState::InProcess
    }
    pub fn change_state_on_in_fail(&mut self) {
        self.state = CompressingState::Fail
    }
    fn block(&mut self) {
        self.need_to_wait = true;
    }
    fn unblock(&mut self) {
        self.need_to_wait = false;
    }

    /// Add the file to our zip_writer.
    pub fn add_file_in_zip(
        &mut self,
        ind: usize,
        file_name: &yew::AttrValue,
        file_data: &[u8],
    ) -> Result<usize, (String, String)> {
        self.block();
        if self.state == CompressingState::Fail {
            self.unblock();
            return Err(("Adding files is blocked.".to_string(), "".to_string()));
        }
        self.state = CompressingState::InProcess;

        if self.zip_writer.is_none() {
            self.unblock();
            Err(("zip_writer is not defined.".to_string(), "".to_string()))
        } else {
            let mut zip_writer = std::mem::take(&mut self.zip_writer).unwrap();
            let mut options = zip::write::SimpleFileOptions::default();

            if !self.password.is_empty() {
                options = if self.type_encryption == TypeEncryption::Aes256 {
                    options.with_aes_encryption(
                        zip::AesMode::Aes256,
                        self.password.as_str(),
                    )
                } else {
                    options.with_deprecated_encryption(self.password.as_bytes())
                };
            }

            let new_archive_filename = format!("{}-{}", ind, file_name.as_str());


            if let Err(err) = zip_writer.start_file(new_archive_filename.as_str(), options) {
                // Failed to add file meta information to archive. The response must be returned.
                self.state = CompressingState::Fail;
                self.unblock();
                return Err((
                    format!(
                        "Failed to add file meta information to archive: {}.",
                        file_name
                    ),
                    format!("{}", err)
                ));
            };
            let res_write = zip_writer.write_all(file_data);
            self.unblock();
            let res = match res_write {
                Ok(_) => {
                    // The file was successfully written.
                    Ok(file_data.len())
                }
                Err(err) => {
                    self.state = CompressingState::Fail;
                    // Error writing file to archive.
                    Err((
                        format!(
                            "Error writing file to archive: {}.",
                            file_name
                        ),
                        format!("{}", err)
                    ))
                }
            };
            self.zip_writer = Some(zip_writer);
            res
        }
    }
    /// Complete archiving
    pub fn finish(
        &mut self,
    ) -> Result<Vec<u8>, (String, String)> {
        if self.zip_writer.is_none() {
            self.state = CompressingState::Fail;
            return Err(("zip_writer is not defined.".to_string(), "".to_string()));
        }
        let zip_writer = std::mem::take(&mut self.zip_writer).unwrap();

        let res = match zip_writer.finish() {
            Ok(res) => {
                self.state = CompressingState::Done;
                Ok(res.get_ref().clone())
            }
            Err(err) => {
                self.state = CompressingState::Fail;
                Err(("Failed to create archive.".to_string(), format!("{}", err)))
            }
        };
        res
    }
}

