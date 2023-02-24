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
            url.push_str(&format!("&num={self}"));
        }
    }
}

impl Parameterize for Vec<u32> {
    fn param(&self, url: &mut String) {
        if !self.is_empty() {
            for uid in self {
                let argument = format!("&uid={}", uid);
                url.push_str(&argument);
            }
        }
    }
}

impl Parameterize for Option<Keyword> {
    fn param(&self, url: &mut String) {
        if let Some(Keyword(key)) = self {
            url.push_str(&format!("&keyword={key}"));
        }
    }
}

impl Parameterize for Vec<String> {
    fn param(&self, url: &mut String) {
        if !self.is_empty() {
            for tag in self {
                let argument = format!("&tag={tag}");
                url.push_str(&argument);
            }
        }
    }
}

impl Parameterize for Size {
    fn param(&self, url: &mut String) {
        let Size(ref size_list) = self;
        if size_list != &[ImageSize::Original] {
            for size in size_list {
                url.push_str(&format!("&size={size}"));
            }
        }
    }
}

impl Parameterize for Proxy {
    fn param(&self, url: &mut String) {
        let Proxy(ref proxy) = self;
        if proxy != "i.pixiv.cat" {
            url.push_str(&format!("&proxy={proxy}"));
        }
    }
}

impl Parameterize for Option<DateAfter> {
    fn param(&self, url: &mut String) {
        if let Some(DateAfter(date)) = self {
            url.push_str(&format!("&dataAfter={date}"));
        }
    }
}

impl Parameterize for Option<DateBefore> {
    fn param(&self, url: &mut String) {
        if let Some(DateBefore(date)) = self {
            url.push_str(&format!("&dataBefore={date}"));
        }
    }
}

impl Parameterize for bool {
    fn param(&self, url: &mut String) {
        if *self {
            url.push_str(&format!("&dsc=true"))
        }
    }
}
