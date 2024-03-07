use std::fs::{self, DirEntry};

enum FlatFileType {
    Toml,
    Rust,
    File,
    Dir,
    Symlink,
}

fn match_icon_color_tuple(file_type: &FlatFileType) -> (&str, u8) {
    match file_type {
        FlatFileType::Toml => ("\u{e6b2}", 33),
        FlatFileType::Rust => ("\u{e7a8}", 32),
        FlatFileType::Dir => ("\u{f413}", 34),
        FlatFileType::Symlink => ("\u{f481}", 31),
        FlatFileType::File => ("\u{ea7b}", 0),
    }
}

fn colorize_text(color: u8, text: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", color, text)
}

fn get_file_suffix(file_name: &str) -> &str {
    let parts: Vec<&str> = file_name.split('.').collect();
    if parts.len() > 1 {
        return parts[parts.len() - 1];
    }
    ""
}

fn get_flat_file_type(file_name: &str, dir_entry: &DirEntry) -> FlatFileType {
    if let Ok(file_type) = dir_entry.file_type() {
        if file_type.is_dir() {
            return FlatFileType::Dir;
        }
        if file_type.is_symlink() {
            return FlatFileType::Symlink;
        }
    }
    match get_file_suffix(file_name) {
        "toml" => FlatFileType::Toml,
        "rs" => FlatFileType::Rust,
        _ => FlatFileType::File,
    }
}

fn print_line(file_type: FlatFileType, text: &str) {
    let (icon, color) = match_icon_color_tuple(&file_type);
    println!("{}\t{}", icon, colorize_text(color, text))
}

fn main() {
    let paths = fs::read_dir("./").unwrap();
    for dir_entry in paths.flatten() {
        if let Some(file_name) = dir_entry.file_name().to_str() {
            print_line(get_flat_file_type(file_name, &dir_entry), file_name)
        }
    }
}
