use std::fmt::{Formatter, Display};
use thiserror::Error as Error;
use crate::convert::Argument;

#[derive(Copy, Clone, Debug)]
/// Non-R18 by default.
pub enum R18 {
    NonR18,
    R18,
    Mixin,
}

#[derive(Debug, Clone)]
/// Not very convenient. you should consider use tags instead.
pub(crate) struct Keyword(pub(crate) Option<String>);

#[derive(Debug, Clone)]
/// at most 20s, at least one.
pub(crate) struct Tag(pub(crate) Option<Vec<String>>);

#[derive(Debug, Clone)]
/// available values were defined in its setter.
pub(crate) struct Size(pub(crate) Option<Vec<ImageSize>>);

#[derive(Debug, Clone)]
pub enum ImageSize {
    Original,
    Regular,
    Small,
    Thumb,
    Mini,
}

#[derive(Debug, Clone)]
/// proxy for `pixiv.net`, `i.pixiv.cat`, e.g. See [Lolicon](https://api.lolicon.app/#/setu?id=proxy) for detail.
pub(crate) struct Proxy(pub(crate) Option<String>);

#[derive(Debug, Clone)]
/// Only show artworks after this UNIX time in millisecond.
pub(crate) struct DateAfter(pub(crate) Option<u64>);

#[derive(Debug, Clone)]
/// Only show artworks before this UNIX time in millisecond.
pub(crate) struct DateBefore(pub(crate) Option<u64>);

impl std::fmt::Display for ImageSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let parameter = match self {
            ImageSize::Original => "original",
            ImageSize::Regular => "regular",
            ImageSize::Small => "small",
            ImageSize::Thumb => "thumb",
            ImageSize::Mini => "mini",
        };
        write!(f, "{}", parameter)
    }
}

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
    /// specified authors. at most 20s, at least one.
    uid: Option<Vec<u32>>,
    /// Not very convenient. you should consider use `tag` instead.
    keyword: Keyword,
    /// at most 20s, at least one.
    tag: Tag,
    /// size of images.
    size: Size,
    /// proxy for `pixiv.net`, `i.pixiv.cat`, e.g. See [Lolicon](https://api.lolicon.app/#/setu?id=proxy) for detail.
    proxy: Proxy,
    /// Only show artworks after this UNIX time in millisecond.
    date_after: DateAfter,
    /// Only show artworks before this UNIX time in millisecond.
    date_before: DateBefore,
    /// If this is `true`, some automatic convert between keywords and tags will be disabled.
    dsc: Option<bool>,
}

impl std::default::Default for Request {
    fn default() -> Self {
        Request {
            r18: None, // Non-R18 by default
            num: None, // 1 by default
            uid: None,
            keyword: Keyword(None),
            tag: Tag(None),
            size: Size(None), // ["original"] by default
            proxy: Proxy(None), // `i.pixiv.cat` by default
            date_after: DateAfter(None),
            date_before: DateBefore(None),
            dsc: None, // `false` by default
        }
    }
}

impl Request {
    /// set whether the result includes R18 artworks.
    pub fn r18(mut self, r: R18) -> Self {
        self.r18 = Some(r);
        self
    }

    /// set amount of result's artworks. 1-100 is legal.
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
    pub fn uid(mut self, authors: &[u32]) -> Result<Self, LoliError> {
        match authors.len() {
            1..=20 => {
                self.uid = Some(authors.into());
                Ok(self)
            }
            _ => Err(LoliError::IllegalUidLen),
        }
    }

    /// set keyword.
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keyword.0 = Some(keyword.into());
        self
    }

    /// set tags.
    pub fn tag(mut self, tag: &[String]) -> Result<Self, LoliError> {
        match tag.len() {
            1..=20 => {
                self.tag.0 = Some(tag.into());
                Ok(self)
            }
            _ => Err(LoliError::IllegalTags),
        }
    }

    /// set sizes.
    pub fn size(mut self, size_list: &[ImageSize]) -> Result<Self, LoliError> {
        match size_list.len() {
            1..=5 => {
                self.size.0 = Some(size_list.into());
                Ok(self)
            }
            _ => Err(LoliError::IllegalSize),
        }
    }

    /// proxy for `pixiv.net`, `i.pixiv.cat`, e.g. See [Lolicon](https://api.lolicon.app/#/setu?id=proxy) for detail.
    pub fn proxy(mut self, proxy: impl Into<String>) -> Self {
        self.proxy.0 = Some(proxy.into());
        self
    }

    /// Only show artworks after this UNIX time in millisecond.
    pub fn date_after(mut self, date_after: u64) -> Self {
        self.date_after.0 = Some(date_after);
        self
    }

    /// Only show artworks before this UNIX time in millisecond.
    pub fn date_before(mut self, date_before: u64) -> Self {
        self.date_before.0 = Some(date_before);
        self
    }

    /// If this is `true`, some automatic convert between keywords and tags will be disabled.
    pub fn dsc(mut self, dsc: bool) -> Self {
        self.dsc = Some(dsc);
        self
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut url: String = "https://api.lolicon.app/setu/v2?".into();

        url.add_argument(&self.r18);
        url.add_argument(&self.num);
        url.add_argument(&self.uid);
        url.add_argument(&self.keyword);
        url.add_argument(&self.tag);
        url.add_argument(&self.size);
        url.add_argument(&self.proxy);
        url.add_argument(&self.date_after);
        url.add_argument(&self.date_before);
        url.add_argument(&self.dsc);

        write!(f, "{}", url)
    }
}

impl From<Request> for String {
    fn from(request: Request) -> Self {
        request.to_string()
    }
}

trait AddArgument {
    /// to convert a argument into url field.
    fn add_argument(&mut self, object: &impl Argument);
}

impl AddArgument for String {
    /// to convert a argument into url field.
    fn add_argument(&mut self, object: &impl Argument) {
        object.argument(self);
    }
}
