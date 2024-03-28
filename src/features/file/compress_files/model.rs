use std::io::Write;
use zip::unstable::write::FileOptionsExt;


/// Состояние процесса компрессии.
#[derive(PartialEq)]
pub(crate) enum CompressingState {
    /// Ожидает начала операции компрессии.
    WaitStart,
    /// В процессе компрессии.
    InProcess,
    /// Процесс компрессии завершен.
    Done,
    /// Процесс компрессии завершился неуспешно.
    Fail,
}

pub(crate) struct CompressionFiles {
    password: String,
    zip_writer: zip::ZipWriter<std::io::Cursor<Vec<u8>>>,
    pub state: CompressingState,
    pub need_to_wait: bool,
}


impl CompressionFiles {
    pub fn new(password: String) -> Self {
        Self {
            password,
            zip_writer: zip::ZipWriter::new(std::io::Cursor::new(vec![])),
            state: CompressingState::WaitStart,
            need_to_wait: false,
        }
    }

    pub fn change_state_on_in_process(&mut self) {
        self.state = CompressingState::InProcess
    }

    fn block(&mut self) {
        self.need_to_wait = true;
    }
    fn unblock(&mut self) {
        self.need_to_wait = false;
    }

    /// Добавляем файл к нашему zip_writer.
    pub fn add_file_in_zip(
        &mut self,
        file_name: &yew::AttrValue,
        file_data: &[u8],
    ) -> Result<usize, (String, String)> {
        self.block();
        if self.state == CompressingState::Fail {
            self.unblock();
            return Err(("Добавление файлов заблокировано.".to_string(), "".to_string()));
        }
        self.state = CompressingState::InProcess;
        let mut options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::DEFLATE);

        if !self.password.is_empty() {
            options = options.with_deprecated_encryption(self.password.as_bytes());
        }

        if let Err(err) = self.zip_writer.start_file(format!("{}", file_name), options) {
            // Не удалось добавить метаинформацию о файле в архив. Необходимо вернуть ответ.
            self.state = CompressingState::Fail;
            self.unblock();
            return Err((
                format!(
                    "Для файла {} не удалось добавить метаинформацию о файле в архив.",
                    file_name
                ),
                format!("{}", err)
            ));
        };
        let res_write = self.zip_writer.write_all(file_data);
        self.unblock();
        match res_write {
            Ok(_) => {
                // Файл записан успешно.
                Ok(file_data.len())
            }
            Err(err) => {
                self.state = CompressingState::Fail;
                // Ошибка записи файла в архив.
                Err((
                    format!(
                        "Ошибка записи файла {} в архив.",
                        file_name
                    ),
                    format!("{}", err)
                ))
            }
        }
    }
    /// Завершить запись в архив
    pub fn finish(
        &mut self,
    ) -> Result<Vec<u8>, (String, String)> {
        match self.zip_writer.finish() {
            Ok(res) => {
                self.state = CompressingState::Done;
                Ok(res.get_ref().clone())
            }
            Err(err) => {
                self.state = CompressingState::Fail;
                Err(("Не удалось создать архив.".to_string(), format!("{}", err)))
            }
        }
    }
}

