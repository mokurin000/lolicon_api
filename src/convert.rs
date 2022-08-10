use crate::datatype::DateAfter;
use crate::datatype::DateBefore;
use crate::datatype::Keyword;
use crate::datatype::Proxy;
use crate::datatype::Size;
use crate::datatype::Class;

pub trait Argument {
    fn argument(&self, url: &mut String);
}

impl Argument for Option<Class> {
    fn argument(&self, url: &mut String) {
        if let Some(r) = self {
            let argument = match r {
                Class::NonR18 => return, // default behavior
                Class::R18 => "&r18=1",
                Class::Mixin => "&r18=2",
            };
            url.push_str(argument);
        }
    }
}

impl Argument for Option<u8> {
    fn argument(&self, url: &mut String) {
        if let Some(num) = self {
            let argument = format!("&num={}", num);
            url.push_str(&argument);
        }
    }
}

impl Argument for Option<Vec<u32>> {
    fn argument(&self, url: &mut String) {
        if let Some(uid_list) = self {
            for uid in uid_list {
                let argument = format!("&uid={}", uid);
                url.push_str(&argument);
            }
        }
    }
}

impl Argument for Keyword {
    fn argument(&self, url: &mut String) {
        if let Keyword(Some(key)) = self {
            let argument = format!("&keyword={}", key);
            url.push_str(&argument);
        }
    }
}

impl Argument for Option<Vec<String>> {
    fn argument(&self, url: &mut String) {
        if let Some(ref tag_list) = self {
            for tag in tag_list {
                let argument = format!("&tag={}", tag);
                url.push_str(&argument);
            }
        }
    }
}

impl Argument for Size {
    fn argument(&self, url: &mut String) {
        if let Size(Some(ref size_list)) = self {
            for size in size_list {
                let argument = format!("&size={}", size);
                url.push_str(&argument);
            }
        }
    }
}

impl Argument for Proxy {
    fn argument(&self, url: &mut String) {
        if let Proxy(Some(ref proxy)) = self {
            let argument = format!("&proxy={}", proxy);
            url.push_str(&argument);
        }
    }
}

impl Argument for DateAfter {
    fn argument(&self, url: &mut String) {
        if let DateAfter(Some(date)) = self {
            let argument = format!("&dataAfter={}", date);
            url.push_str(&argument);
        }
    }
}

impl Argument for DateBefore {
    fn argument(&self, url: &mut String) {
        if let DateBefore(Some(date)) = self {
            let argument = format!("&dataBefore={}", date);
            url.push_str(&argument);
        }
    }
}

impl Argument for Option<bool> {
    fn argument(&self, url: &mut String) {
        if let Some(dsc) = self {
            let argument = format!("&dsc={}", dsc);
            url.push_str(&argument);
        }
    }
}
