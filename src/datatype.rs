use crate::convert::Argument;
use std::{
    fmt::{Display, Formatter},
    ops::RangeInclusive,
};
use thiserror::Error;

#[derive(Copy, Clone, Debug)]
/// Non-R18 by default.
pub enum Class {
    NonR18,
    R18,
    Mixin,
}

#[derive(Debug, Clone)]
/// Not very convenient. you should consider use tags instead.
pub(crate) struct Keyword(pub(crate) String);

#[derive(Debug, Clone)]
/// available values were defined in its setter.
pub(crate) struct Size(pub(crate) Vec<ImageSize>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageSize {
    Original,
    Regular,
    Small,
    Thumb,
    Mini,
}

#[derive(Debug, Clone)]
/// proxy for `pixiv.net`, `i.pixiv.cat`, e.g. See [Lolicon](https://api.lolicon.app/#/setu?id=proxy) for detail.
pub(crate) struct Proxy(pub(crate) String);

#[derive(Debug, Clone)]
/// Only show artworks after this UNIX time in millisecond.
pub(crate) struct DateAfter(pub(crate) u64);

#[derive(Debug, Clone)]
/// Only show artworks before this UNIX time in millisecond.
pub(crate) struct DateBefore(pub(crate) u64);

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

/// the only possible error is invalid fields passed to `Request`'s setters
/// e.g. `num` cannot greater than 100
#[derive(Debug, Error, Clone, PartialEq, Eq)]

pub enum Error {
    #[error("excepted {range:?}, found {actual} {filed}")]
    OutOfRange {
        range: RangeInclusive<usize>,
        actual: usize,
        filed: &'static str,
    },
}

#[must_use]
#[derive(Clone, Debug)]
pub struct Request {
    /// Non-R18 by default.
    class: Class,
    /// amount of result's artworks. 1-100 is allowed.
    num: u8,
    /// specified authors. at most 20s, at least one.
    uid: Vec<u32>,
    /// Not very convenient. you should consider use `tag` instead.
    keyword: Option<Keyword>,
    /// at most 20s, at least one.
    tag: Vec<String>,
    /// size of images.
    size: Size,
    /// proxy for `pixiv.net`, `i.pixiv.cat`, e.g. See [Lolicon](https://api.lolicon.app/#/setu?id=proxy) for detail.
    proxy: Proxy,
    /// Only show artworks after this UNIX time in millisecond.
    date_after: Option<DateAfter>,
    /// Only show artworks before this UNIX time in millisecond.
    date_before: Option<DateBefore>,
    /// If this is `true`, some automatic convert between keywords and tags will be disabled.
    dsc: bool,
}

impl std::default::Default for Request {
    fn default() -> Self {
        Request {
            class: Class::NonR18,
            num: 1,
            uid: vec![],
            keyword: None,
            tag: vec![],
            size: Size(vec![ImageSize::Original]),
            proxy: Proxy("i.pixiv.cat".into()),
            date_after: None,
            date_before: None,
            dsc: false,
        }
    }
}

impl Request {
    /// set whether the result includes R18 artworks.
    pub fn class(self, class: Class) -> Self {
        Self { class, ..self }
    }

    /// set amount of result's artworks. 1-100 is allowed.
    pub fn num(self, amount: u8) -> Result<Self, Error> {
        if (0..=100).contains(&amount) {
            Ok(Self {
                num: amount,
                ..self
            })
        } else {
            Err(Error::OutOfRange {
                range: 0..=100,
                actual: amount as usize,
                filed: "",
            })
        }
    }

    /// set artworks' authors.
    /// if authors.len() == 0, we do not limit author anyway
    pub fn uid(self, authors: &[u32]) -> Result<Self, Error> {
        if (0..=20).contains(&authors.len()) {
            Ok(Self {
                uid: authors.into(),
                ..self
            })
        } else {
            Err(Error::OutOfRange {
                range: 0..=20,
                actual: authors.len(),
                filed: "uid",
            })
        }
    }

    /// set keyword.
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keyword = Some(Keyword(keyword.into()));
        self
    }

    /// set tags.
    pub fn tag(self, tag: &[String]) -> Result<Self, Error> {
        if (1..=20).contains(&tag.len()) {
            Ok(Self {
                tag: tag.into(),
                ..self
            })
        } else {
            Err(Error::OutOfRange {
                range: 1..=20,
                actual: tag.len(),
                filed: "tag",
            })
        }
    }

    /// set sizes.
    /// if you passed an empty list, you will not get `url`'s, but the information about the picture
    pub fn size(self, size_list: &[ImageSize]) -> Result<Self, Error> {
        match size_list.len() {
            0..=5 => Ok(Self {
                size: Size(size_list.into()),
                ..self
            }),
            _ => Err(Error::OutOfRange {
                range: 0..=5,
                actual: size_list.len(),
                filed: "size",
            }),
        }
    }

    /// proxy for `pixiv.net`, `i.pixiv.cat`, e.g. See [Lolicon](https://api.lolicon.app/#/setu?id=proxy) for detail.
    pub fn proxy(self, proxy: impl Into<String>) -> Self {
        Self {
            proxy: Proxy(proxy.into()),
            ..self
        }
    }

    /// Only show artworks after this UNIX time in millisecond.
    pub fn date_after(mut self, date_after: u64) -> Self {
        self.date_after = Some(DateAfter(date_after));
        self
    }

    /// Only show artworks before this UNIX time in millisecond.
    pub fn date_before(mut self, date_before: u64) -> Self {
        self.date_before = Some(DateBefore(date_before));
        self
    }

    /// If this is `true`, some automatic convert between keywords and tags will be disabled.
    pub fn dsc(self, dsc: bool) -> Self {
        Self { dsc, ..self }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut url: String = "https://api.lolicon.app/setu/v2?".into();

        url.add_argument(&self.class);
        url.add_argument(&self.date_after);
        url.add_argument(&self.date_before);
        url.add_argument(&self.dsc);
        url.add_argument(&self.keyword);
        url.add_argument(&self.num);
        url.add_argument(&self.proxy);
        url.add_argument(&self.size);
        url.add_argument(&self.tag);
        url.add_argument(&self.uid);

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
