use crate::datatype::Class;
use crate::datatype::DateAfter;
use crate::datatype::DateBefore;
use crate::datatype::Keyword;
use crate::datatype::Proxy;
use crate::datatype::Size;
use crate::ImageSize;

pub trait Argument {
    fn argument(&self, url: &mut String);
}

impl Argument for Class {
    fn argument(&self, url: &mut String) {
        let argument = match self {
            Class::NonR18 => return,
            Class::R18 => "&r18=1",
            Class::Mixin => "&r18=2",
        };
        url.push_str(argument);
    }
}

impl Argument for u8 {
    fn argument(&self, url: &mut String) {
        if self != &1 {
            url.push_str(&format!("&num={self}"));
        }
    }
}

impl Argument for Vec<u32> {
    fn argument(&self, url: &mut String) {
        if !self.is_empty() {
            for uid in self {
                let argument = format!("&uid={}", uid);
                url.push_str(&argument);
            }
        }
    }
}

impl Argument for Option<Keyword> {
    fn argument(&self, url: &mut String) {
        if let Some(Keyword(key)) = self {
            url.push_str(&format!("&keyword={key}"));
        }
    }
}

impl Argument for Vec<String> {
    fn argument(&self, url: &mut String) {
        if !self.is_empty() {
            for tag in self {
                let argument = format!("&tag={tag}");
                url.push_str(&argument);
            }
        }
    }
}

impl Argument for Size {
    fn argument(&self, url: &mut String) {
        let Size(ref size_list) = self;
        if size_list != &[ImageSize::Original] {
            for size in size_list {
                url.push_str(&format!("&size={size}"));
            }
        }
    }
}

impl Argument for Proxy {
    fn argument(&self, url: &mut String) {
        let Proxy(ref proxy) = self;
        if proxy != "i.pixiv.cat" {
            url.push_str(&format!("&proxy={proxy}"));
        }
    }
}

impl Argument for Option<DateAfter> {
    fn argument(&self, url: &mut String) {
        if let Some(DateAfter(date)) = self {
            url.push_str(&format!("&dataAfter={date}"));
        }
    }
}

impl Argument for Option<DateBefore> {
    fn argument(&self, url: &mut String) {
        if let Some(DateBefore(date)) = self {
            url.push_str(&format!("&dataBefore={date}"));
        }
    }
}

impl Argument for bool {
    fn argument(&self, url: &mut String) {
        if *self {
            url.push_str(&format!("&dsc=true"))
        }
    }
}
