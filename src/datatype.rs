use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString};

use crate::convert::Parameterize;

use std::{
    fmt::{Display, Formatter},
    ops::RangeInclusive,
};

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, EnumIs, EnumCount, EnumString)]
/// Non-R18 by default.
pub enum Category {
    #[strum(to_string = "non-R18", serialize = "non-r18")]
    NonR18,
    #[strum(serialize = "r18")]
    R18,
    #[strum(to_string = "R-18 or non-R18", serialize = "mixin")]
    Mixin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Not very convenient. you should consider use tags instead.
pub(crate) struct Keyword(pub(crate) String);

#[derive(Debug, Clone, PartialEq, Eq)]
/// available values were defined in its setter.
pub(crate) struct Size(pub(crate) Vec<ImageSize>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, EnumIs, EnumCount, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum ImageSize {
    Original,
    Regular,
    Small,
    Thumb,
    Mini,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// proxy for `pixiv.net`, `i.pixiv.re` by default. See [Lolicon](https://api.lolicon.app/#/setu?id=proxy) for detail.
pub(crate) struct Proxy(pub(crate) String);

#[derive(Debug, Clone, PartialEq, Eq)]
/// Only show artworks after this UNIX time in millisecond.
pub(crate) struct DateAfter(pub(crate) u64);

#[derive(Debug, Clone, PartialEq, Eq)]
/// Only show artworks before this UNIX time in millisecond.
pub(crate) struct DateBefore(pub(crate) u64);

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
    #[error("each tag condition could only contain at most 20 OR tags!")]
    InvalidTag,
    #[cfg(feature = "aspect_validate")]
    #[error("aspect ratio must match regex")]
    InvalidAspectRatio,
}

#[must_use]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    /// Non-R18 by default.
    category: Category,
    /// amount of result's artworks. 1~20 is allowed.
    num: Num,
    /// specified authors. at least one, at most 20.
    uid: Uid,
    /// Not very convenient. you should consider using `tag` instead.
    keyword: Option<Keyword>,
    /// at least one, at most 20.
    tag: Tag,
    /// size of images.
    size: Size,
    /// proxy for `pixiv.net`. `i.pixiv.cat` by default. See [Lolicon](https://api.lolicon.app/#/setu?id=proxy) for detail.
    proxy: Proxy,
    /// Only show artworks after this UNIX time in millisecond.
    date_after: Option<DateAfter>,
    /// Only show artworks before this UNIX time in millisecond.
    date_before: Option<DateBefore>,
    /// If this is `true`, some automatic convert between keywords and tags will be disabled.
    dsc: Dsc,
    /// exclude AI artworks
    exclude_ai: ExcludeAI,
    /// aspect ratio
    aspect_ratio: AspectRatio,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Uid(pub Vec<u32>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Tag(pub Vec<String>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Num(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Dsc(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExcludeAI(pub bool);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AspectRatio(pub Option<String>);

impl std::default::Default for Request {
    fn default() -> Self {
        Request {
            category: Category::NonR18,
            num: Num(1),
            uid: Uid(vec![]),
            keyword: None,
            tag: Tag(vec![]),
            size: Size(vec![ImageSize::Original]),
            proxy: Proxy("i.pixiv.re".into()),
            date_after: None,
            date_before: None,
            dsc: Dsc(false),
            exclude_ai: ExcludeAI(false),
            aspect_ratio: AspectRatio(None),
        }
    }
}

impl Request {
    /// set whether the result includes R18 artworks.
    pub fn category(self, category: Category) -> Self {
        Self { category, ..self }
    }

    /// set amount of result's artworks. 0-100 is allowed.
    pub fn num(self, amount: u8) -> Result<Self, Error> {
        let valid_range = 1..=20;
        if valid_range.contains(&(amount as usize)) {
            Ok(Self {
                num: Num(amount),
                ..self
            })
        } else {
            Err(Error::OutOfRange {
                range: valid_range,
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
                uid: Uid(authors.into()),
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
    pub fn keyword(self, keyword: impl Into<String>) -> Self {
        Self {
            keyword: Some(Keyword(keyword.into())),
            ..self
        }
    }

    /// set tags.
    ///
    /// You can provide at most 3 AND tag groups.
    ///
    /// each AND tag group contains at most 20 OR tags splitted by `|`
    pub fn tag(self, tag: &[impl AsRef<str>]) -> Result<Self, Error> {
        if (0..=3).contains(&tag.len()) {
            if tag
                .iter()
                .map(AsRef::as_ref)
                .any(|s| s.split("|").count() > 20)
            {
                Err(Error::InvalidTag)?
            }
            Ok(Self {
                tag: Tag(tag.iter().map(AsRef::as_ref).map(String::from).collect()),
                ..self
            })
        } else {
            Err(Error::OutOfRange {
                range: 0..=3,
                actual: tag.len(),
                filed: "AND tag",
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
    pub fn date_after(self, date_after: u64) -> Self {
        Self {
            date_after: Some(DateAfter(date_after)),
            ..self
        }
    }

    /// Only show artworks before this UNIX time in millisecond.
    pub fn date_before(self, date_before: u64) -> Self {
        Self {
            date_before: Some(DateBefore(date_before)),
            ..self
        }
    }

    /// If this is `true`, some automatic convert between keywords and tags will be disabled.
    pub fn dsc(self, dsc: bool) -> Self {
        Self {
            dsc: Dsc(dsc),
            ..self
        }
    }

    /// Exclude AI artworks
    pub fn exclude_ai(self, exclude_ai: bool) -> Self {
        Self {
            exclude_ai: ExcludeAI(exclude_ai),
            ..self
        }
    }

    /// filter aspect ratio
    ///
    /// format: `<gt|gte|lt|lte|eq><decimal>`, once or twice
    ///
    /// example: `gte1.777lte1.778`
    ///
    /// when `aspect_validate` is disabled, this always return `Ok(_)`
    pub fn aspect_ratio(self, aspect_ratio: impl AsRef<str>) -> Result<Self, Error> {
        let aspect_ratio = aspect_ratio.as_ref();

        #[cfg(feature = "aspect_validate")]
        {
            use regex::Regex;
            use std::sync::LazyLock;

            static RE: LazyLock<Regex> =
                LazyLock::new(|| Regex::new(r#"^((gt|gte|lt|lte|eq)[\d.]+){1,2}$"#).unwrap());

            if !RE.is_match(aspect_ratio) {
                Err(Error::InvalidAspectRatio)?
            }
        }

        Ok(Self {
            aspect_ratio: AspectRatio(Some(aspect_ratio.into())),
            ..self
        })
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut url: String = "https://api.lolicon.app/setu/v2?".into();

        url.append(&self.category);
        url.append(&self.date_after);
        url.append(&self.date_before);
        url.append(&self.dsc);
        url.append(&self.keyword);
        url.append(&self.num);
        url.append(&self.proxy);
        url.append(&self.size);
        url.append(&self.tag);
        url.append(&self.uid);
        url.append(&self.exclude_ai);
        url.append(&self.aspect_ratio);

        write!(f, "{}", url)
    }
}

impl From<Request> for String {
    fn from(request: Request) -> Self {
        request.to_string()
    }
}

trait AddArgument {
    /// append a argument into url field.
    fn append(&mut self, option: &impl Parameterize);
}

impl AddArgument for String {
    fn append(&mut self, option: &impl Parameterize) {
        option.param(self);
    }
}
