use std::fmt::Write;

use crate::datatype::AspectRatio;
use crate::datatype::Category;
use crate::datatype::DateAfter;
use crate::datatype::DateBefore;
use crate::datatype::Dsc;
use crate::datatype::ExcludeAI;
use crate::datatype::Keyword;
use crate::datatype::Num;
use crate::datatype::Proxy;
use crate::datatype::Size;
use crate::datatype::Tag;
use crate::datatype::Uid;
use crate::ImageSize;

pub trait Parameterize {
    fn param(&self, url: &mut String);
}

impl Parameterize for Category {
    fn param(&self, url: &mut String) {
        let argument = match self {
            Category::NonR18 => "&r18=0",
            Category::R18 => "&r18=1",
            Category::Mixin => "&r18=2",
        };
        url.push_str(argument);
    }
}

impl Parameterize for Uid {
    fn param(&self, url: &mut String) {
        for uid in &self.0 {
            let _ = url.write_fmt(format_args!("&uid={uid}"));
        }
    }
}

impl Parameterize for Option<Keyword> {
    fn param(&self, url: &mut String) {
        if let Some(Keyword(key)) = self {
            let _ = url.write_fmt(format_args!("&keyword={key}"));
        }
    }
}

impl Parameterize for Tag {
    fn param(&self, url: &mut String) {
        for tag in &self.0 {
            let _ = url.write_fmt(format_args!("&tag={tag}"));
        }
    }
}

impl Parameterize for Size {
    fn param(&self, url: &mut String) {
        let Size(ref size_list) = self;
        if size_list != &[ImageSize::Original] {
            for size in size_list {
                let _ = url.write_fmt(format_args!("&size={size}"));
            }
        }
    }
}

impl Parameterize for Proxy {
    fn param(&self, url: &mut String) {
        let Proxy(ref proxy) = self;
        let _ = url.write_fmt(format_args!("&proxy={proxy}"));
    }
}

impl Parameterize for Option<DateAfter> {
    fn param(&self, url: &mut String) {
        if let Some(DateAfter(date)) = self {
            let _ = url.write_fmt(format_args!("&dateAfter={date}"));
        }
    }
}

impl Parameterize for Option<DateBefore> {
    fn param(&self, url: &mut String) {
        if let Some(DateBefore(date)) = self {
            let _ = url.write_fmt(format_args!("&dateBefore={date}"));
        }
    }
}

impl Parameterize for Dsc {
    fn param(&self, url: &mut String) {
        let _ = url.write_fmt(format_args!("&dsc=true"));
    }
}

impl Parameterize for Num {
    fn param(&self, url: &mut String) {
        let _ = url.write_fmt(format_args!("&num={}", self.0));
    }
}

impl Parameterize for ExcludeAI {
    fn param(&self, url: &mut String) {
        let _ = url.write_fmt(format_args!("&excludeAI={}", self.0));
    }
}

impl Parameterize for AspectRatio {
    fn param(&self, url: &mut String) {
        if let Some(s) = &self.0 {
            let _ = url.write_fmt(format_args!("&aspectRatio={s}"));
        }
    }
}
