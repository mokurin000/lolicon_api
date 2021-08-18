use crate::Error;

#[derive(Error, Copy, Clone)]
enum LoliError {
    IlleagalNum,
    IlleagalUid,
    IlleagalSize,
}

#[derive(Clone, Debug)]
struct Request {
    /// Non-R18 by default.
    r18: Option<R18>,
    /// amount of result's artworks. 1-100 is legal.
    num: Option<u8>,
    /// specified authors. at most 20s.
    uid: Option<Vec<u32>>,
    /// Not very conveninent. you should consider use tags instead.
    keyword: Option<String>,
    /// at most 20s
    tag: Option<Vec<String>>,
    /// avaliable values are `response::Object`'s variants.
    size: Option<Vec<String>>,
    /// proxy for `pixiv.net`, `i.pixiv.cat`, e.g.
    proxy: Option<String>,
    /// Only show artworks after this UNIX time in millisecond.
    date_after: <u64>,
    /// Only show artworks before this UNIX time in millisecond.
    date_before: <u64>,
    /// If this is `true`, some automatic convert between keywords and tags will be disabled.
    dsc: Option<bool>,
}

#[derive(Copy, Clone)]
pub enum R18 {
    NonR18,
    R18,
    Mixin,
}

