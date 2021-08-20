use crate::DateAfter;
use crate::DateBefore;
use crate::Keyword;
use crate::Proxy;
use crate::Size;
use crate::Tag;
use crate::R18;

pub trait Argument {
    fn argument(&self, url: &mut String);
}

impl Argument for Option<R18> {
    fn argument(&self, url: &mut String) {
        if let Some(r) = self {
            let argument = match r {
                R18::NonR18 => return, // default behavior
                R18::R18 => "&r18=1",
                R18::Mixin => "&r18=2",
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

impl Argument for Tag {
    fn argument(&self, url: &mut String) {
        if let Tag(Some(ref tag_list)) = self {
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
            let argument = format!("&dsc={:?}", dsc);
            url.push_str(&argument);
        }
    }
}
