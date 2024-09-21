use std::fmt::Write;

use crate::datatype::Category;
use crate::datatype::DateAfter;
use crate::datatype::DateBefore;
use crate::datatype::Keyword;
use crate::datatype::Proxy;
use crate::datatype::Size;
use crate::ImageSize;

pub trait Parameterize {
    fn param(&self, url: &mut String);
}

impl Parameterize for Category {
    fn param(&self, url: &mut String) {
        let argument = match self {
            Category::NonR18 => return,
            Category::R18 => "&r18=1",
            Category::Mixin => "&r18=2",
        };
        url.push_str(argument);
    }
}

impl Parameterize for u8 {
    fn param(&self, url: &mut String) {
        if self != &1 {
            let _ = url.write_fmt(format_args!("&num={self}"));
        }
    }
}

impl Parameterize for Vec<u32> {
    fn param(&self, url: &mut String) {
        if !self.is_empty() {
            for uid in self {
                let _ = url.write_fmt(format_args!("&uid={uid}"));
            }
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

impl Parameterize for Vec<String> {
    fn param(&self, url: &mut String) {
        if !self.is_empty() {
            for tag in self {
                let _ = url.write_fmt(format_args!("&tag={tag}"));
            }
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
        if proxy != "i.pixiv.cat" {
            let _ = url.write_fmt(format_args!("&proxy={proxy}"));
        }
    }
}

impl Parameterize for Option<DateAfter> {
    fn param(&self, url: &mut String) {
        if let Some(DateAfter(date)) = self {
            let _ = url.write_fmt(format_args!("&dataAfter={date}"));
        }
    }
}

impl Parameterize for Option<DateBefore> {
    fn param(&self, url: &mut String) {
        if let Some(DateBefore(date)) = self {
            let _ = url.write_fmt(format_args!("&dataBefore={date}"));
        }
    }
}

impl Parameterize for bool {
    fn param(&self, url: &mut String) {
        if *self {
            let _ = url.write_fmt(format_args!("&dsc=true"));
        }
    }
}
