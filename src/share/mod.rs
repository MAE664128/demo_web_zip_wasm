pub mod fs_tools;


const SUFFIX: [&str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

pub fn size_to_string(size: f64) -> String {
    let size = size as f32;
    if size <= 0.0 {
        "0 B".to_string()
    } else {
        let base = size.log10() / 1024.0_f32.log10();
        let mut buffer = ryu::Buffer::new();
        let result = buffer
            // Source for this hack: https://stackoverflow.com/a/28656825
            .format((1024.0_f32.powf(base - base.floor()) * 10.0).round() / 10.0)
            .trim_end_matches(".0");
        // Add suffix
        [result, SUFFIX[base.floor() as usize]].join(" ")
    }
}