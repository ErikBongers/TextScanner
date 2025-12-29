use std::{fmt, fs};
use std::path::{Path, PathBuf};
use text_scanner::TextScanner;

pub type Result<T> = std::result::Result<T, WplError>;

#[derive(Debug)]
pub enum WplError {
    FileError(String),
    ScanError(String),
}

impl fmt::Display for WplError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WplError::FileError(msg) => write!(f, "{}", msg),
            WplError::ScanError(msg) => write!(f, "{}", msg),
        }
    }
}

pub fn scan_wpl(path: String) -> Result<Vec<String>> {
    let file_path = std::path::Path::new(&path);
    let base_path = file_path.parent().unwrap();//todo: unwrap!!
    let text = fs::read_to_string(&path).map_err( |err| {
            return WplError::FileError(err.to_string());
        })?;
    let mut scanner = TextScanner::new(&text);
    let mut tracks: Vec<String> = vec![];
    loop {
        if !scanner.find("<media src=\"") {
            break;
        }
        let Ok(first_line) = scanner.get_until("\" ") else {
            return Err(WplError::ScanError("Could not find end of string.".to_string())) //todo get line and pos: todo: add line and pos to scanner? Yes, but calculate it on demand!
        };
        let rel_track_path = scanner[&first_line].to_string();
        let abs_track_path = base_path.join(rel_track_path);

        let path_str = replace_special_xml_chars(&abs_track_path.as_path().to_str().unwrap());
        let path_str = normalize_path(base_path, &path_str);
        tracks.push(path_str);
    }
    Ok(tracks)
}

fn replace_special_xml_chars(xml: &str) -> String {
    xml
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

/// Returns the canonicalized path of the track.
/// In case this fails, the original relative path is returned.
fn normalize_path(base_path: &Path, rel_track_path_str: &str) -> String {
    let full_track_path = base_path.join(rel_track_path_str);
    let str_path = full_track_path.to_str().unwrap().to_string();
    let str_path = str_path.replace("\\", "/");
    let full_track_path = PathBuf::from(str_path);
    let canonicalized_path = full_track_path.canonicalize();
    match canonicalized_path {
        Ok(path) => {
            path.to_str().unwrap().to_string() //todo: unwrap!!!
        }
        Err(_) => {
            rel_track_path_str.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = scan_wpl(r#"Z:\Music\My Playlists\Religioso.wpl"#.to_string());

        for line in res.unwrap() {
            println!("{}", line);
        }
    }
}