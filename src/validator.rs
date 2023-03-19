use std::path::Path;

pub fn valid(file: &str) -> bool {
    let ext = get_ext(file);
    let valid_exts = vec!["png", "jpg", "jpeg", "gif", "bmp"];

    valid_exts.contains(&ext)
}

fn get_ext(file: &str) -> &str {
    Path::new(file)
        .extension()
        .map_or("", |ext| ext.to_str().unwrap_or(""))
}
