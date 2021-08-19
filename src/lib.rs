pub use thiserror::Error;

use convert::Argument;

mod convert;

#[derive(Debug, Error, Copy, Clone)]
pub enum LoliError {
    IllegalNum,
    IllegalSize,
    IllegalUidLen,
    IllegalTags,
}

impl std::fmt::Display for LoliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

struct Tag(Option<Vec<String>>);
struct Size(Option<Vec<String>>);
struct DataAfter(u64);
struct DataBefore(u64);

#[derive(Clone, Debug)]
pub struct Request {
    /// Non-R18 by default.
    r18: Option<R18>,
    /// amount of result's artworks. 1-100 is legal.
    num: Option<u8>,
    /// specified authors. at most 20s.
    uid: Option<Vec<u32>>,
    /// Not very convenient. you should consider use tags instead.
    keyword: Option<String>,
    /// at most 20s
    tag: Tag,
    /// available values were defined in its setter.
    size: Size,
    /// proxy for `pixiv.net`, `i.pixiv.cat`, e.g. See [Lolicon](https://api.lolicon.app/#/setu?id=proxy) for detail.
    proxy: Option<String>,
    /// Only show artworks after this UNIX time in millisecond.
    date_after: DataAfter,
    /// Only show artworks before this UNIX time in millisecond.
    date_before: DataBefore,
    /// If this is `true`, some automatic convert between keywords and tags will be disabled.
    dsc: Option<bool>,
}

#[derive(Copy, Clone, Debug)]
pub enum R18 {
    NonR18,
    R18,
    Mixin,
}

impl Request {
    pub fn r18(mut self, r: R18) -> Self {
        self.r18 = Some(r);
        self
    }

    pub fn num(mut self, amount: u8) -> Result<Self, LoliError> {
        match amount {
            1..=100 => {
                self.num = Some(amount);
                Ok(self)
            }
            _ => Err(LoliError::IllegalNum),
        }
    }

    pub fn uid(mut self, authors: Vec<u32>) -> Result<Self, LoliError> {
        match authors.len() {
            1..=20 => {
                self.uid = Some(authors);
                Ok(self)
            }
            _ => Err(LoliError::IllegalUidLen),
        }
    }

    pub fn keyword(mut self, keyword: String) -> Self {
        self.keyword = Some(keyword);
        self
    }

    pub fn tag(mut self, tag: Vec<String>) -> Result<Self, LoliError> {
        match tag.len() {
            1..=20 => {
                self.tag.0 = Some(tag);
                Ok(self)
            },
            _ => Err(LoliError::IllegalTags),
        }
    }

    pub fn size(mut self, size_list: Vec<String>) -> Result<Self, LoliError> {
        let sizes = ["original", "regular", "small", "thumb", "mini"];
        match size_list.len() {
            1..=5 => {
                for size in size_list {
                    if !sizes.contains(&size.as_str()) {
                        return Err(LoliError::IllegalSize);
                    }
                }
                Ok(self)
            }
            _ => return Err(LoliError::IllegalSize),
        }
    }
}

impl Into<String> for Request {
    fn into(self) -> String {
        let mut url: String = "https://api.lolicon.app/setu/v2?".into();
        self.r18.argument(&mut url);
        self.num.argument(&mut url);
        self.uid.argument(&mut url);
        self.keyword.argument(&mut url);
        self.tag.argument(&mut url);
        self.size.argument(&mut url);
        url
    }
}
