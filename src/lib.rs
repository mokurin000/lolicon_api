use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    /// If any error occurs, this will be the error.
    error: String,
    /// This is the array of pictrues returned.
    data: Vec<Setu>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Setu {
    /// pixiv id of this artwork.
    pid: u32,
    /// page id of this picture.
    p: i32,
    /// author's pixiv uid.
    uid: u32,
    /// title of this artwork.
    title: String,
    /// author's name.
    author: String,
    /// whether if this is r18.
    r18: bool,
    /// width of this picture, in `px`.
    width: u32,
    /// height of this picture, in `px`.
    height: u32,
    /// tags of this pictrue, with Chinese translation when possible.
    tags: String,
    /// extend name of this picture.
    ext: String,
    /// time then it has been uploaded, in millisecond.
    uploadDate: u64,
    /// URLs including all sizes.
    object: Object,
}

#[derive(Debug, Serialize, Deserialize)]
struct Object {
    original: Option<String>,
    regular: Option<String>,
    small: Option<String>,
    thumb: Option<String>,
    mini: Option<String>,
}