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

pub struct Wpl {
    pub name: String,
    pub items: Vec<WplItem>
}

pub enum WplItemType { Track, Stream}
impl WplItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            WplItemType::Track => "Track",
            WplItemType::Stream => "Stream"
        }
    }
}
pub struct WplItem {
    pub path: String,
    pub item_type: WplItemType,
}

pub fn scan_wpl(path: String) -> Result<Wpl> {
    let file_path = std::path::Path::new(&path);
    let base_path = file_path.parent().unwrap();//todo: unwrap!!
    let text = fs::read_to_string(&path).map_err( |err| {
            return WplError::FileError(err.to_string());
        })?;
    let mut scanner = TextScanner::new(&text);

    let title = file_path.file_prefix()
        .map(|f| f.to_str().unwrap_or(""))
        .unwrap_or("");
    scanner.find("<title>");
    let title = scanner.get_until("</title>")
        .map(|t| scanner[&t].to_string())
        .unwrap_or(title.to_string());

    let mut tracks: Vec<WplItem> = vec![];
    loop {
        if !scanner.find("<media src=\"") {
            break;
        }
        let Ok(first_line) = scanner.get_until("\"") else {
            return Err(WplError::ScanError("Could not find end of string.".to_string())) //todo get line and pos: todo: add line and pos to scanner? Yes, but calculate it on demand!
        };

        let path_str = normalize_path(base_path, &scanner[&first_line]);
        tracks.push(path_str);
    }

    let wpl = Wpl {
      name: title.to_string(),
      items: tracks
    };

    Ok(wpl)
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
fn normalize_path(base_path: &Path, rel_track_path_str: &str) -> WplItem {
    if rel_track_path_str.starts_with("http") {
        return WplItem {
            path: rel_track_path_str.to_string(),
            item_type: WplItemType::Stream
        }
    }
    let full_track_path = base_path
        .join(replace_special_xml_chars(&rel_track_path_str));
    let str_path = full_track_path.to_str().unwrap().to_string();
    let str_path = str_path.replace("\\", "/");
    let full_track_path = PathBuf::from(str_path);
    let canonicalized_path = full_track_path.canonicalize();
    match canonicalized_path {
        Ok(path) => WplItem {
            path: path.to_str().unwrap().to_string(), //todo: unwrap!!!
            item_type: WplItemType::Track
        },
        Err(_) => WplItem {
            path: rel_track_path_str.to_string(),
            item_type: WplItemType::Track //assuming it's a track, but really, it's unknown.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let res = scan_wpl(r#"Z:\Music\My Playlists\WebTVs.wpl"#.to_string()).unwrap();

        println!("name: {}", res.name);
        for line in res.items {
            println!("{}::{}", line.item_type.as_str(), line.path);
        }
    }
}