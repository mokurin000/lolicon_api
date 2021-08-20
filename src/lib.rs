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

#[derive(Debug, Clone)]
struct Tag(Option<Vec<String>>);

#[derive(Debug, Clone)]
struct Size(Option<Vec<String>>);

#[derive(Debug, Clone)]
struct DataAfter(Option<u64>);

#[derive(Debug, Clone)]
struct DataBefore(Option<u64>);

#[derive(Copy, Clone, Debug)]
pub enum R18 {
    NonR18,
    R18,
    Mixin,
}

impl std::default::Default for Request {
    fn default() -> Self {
        Request {
            r18: None,
            num: None,
            uid: None,
            keyword: None,
            tag: Tag(None),
            size: Size(None),
            proxy: None,
            date_after: DataAfter(None),
            date_before: DataBefore(None),
            dsc: None,
        }
    }
}

impl Request {
    /// set whether the result includes R18 artworks.
    pub fn r18(mut self, r: R18) -> Self {
        self.r18 = Some(r);
        self
    }

    /// set amount of result's artworks.
    pub fn num(mut self, amount: u8) -> Result<Self, LoliError> {
        match amount {
            1..=100 => {
                self.num = Some(amount);
                Ok(self)
            }
            _ => Err(LoliError::IllegalNum),
        }
    }

    /// set artworks' authors.
    pub fn uid(mut self, authors: Vec<u32>) -> Result<Self, LoliError> {
        match authors.len() {
            1..=20 => {
                self.uid = Some(authors);
                Ok(self)
            }
            _ => Err(LoliError::IllegalUidLen),
        }
    }

    /// set keyword.
    pub fn keyword(mut self, keyword: String) -> Self {
        self.keyword = Some(keyword);
        self
    }

    /// set tags.
    pub fn tag(mut self, tag: Vec<String>) -> Result<Self, LoliError> {
        match tag.len() {
            1..=20 => {
                self.tag.0 = Some(tag);
                Ok(self)
            }
            _ => Err(LoliError::IllegalTags),
        }
    }

    /// set sizes. `original`, `regular`, `small`, `thumb`, `mini` are available.
    pub fn size(mut self, size_list: Vec<String>) -> Result<Self, LoliError> {
        let sizes = ["original", "regular", "small", "thumb", "mini"];
        match size_list.len() {
            1..=5 => {
                for size in &size_list {
                    if !sizes.contains(&size.as_str()) {
                        return Err(LoliError::IllegalSize);
                    }
                }
                self.size.0 = Some(size_list);
                Ok(self)
            }
            _ => Err(LoliError::IllegalSize),
        }
    }
}

impl From<Request> for String {
    fn from(req: Request) -> Self {
        let mut url: String = "https://api.lolicon.app/setu/v2?".into();
        req.r18.argument(&mut url);
        req.num.argument(&mut url);
        req.uid.argument(&mut url);
        req.keyword.argument(&mut url);
        req.tag.argument(&mut url);
        req.size.argument(&mut url);
        url
    }
}