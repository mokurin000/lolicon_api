use crate::DataAfter;
use crate::DataBefore;
use crate::Size;
use crate::Tag;
use crate::R18;

pub trait Argument {
    fn argument(&self, url: &mut String);
}

impl Argument for Option<R18> {
    fn argument(&self, url: &mut String) {
        let argument = if let Some(r) = self {
            match r {
                R18::NonR18 => "&r18=0",
                R18::R18 => "&r18=1",
                R18::Mixin => "&r18=2",
            }
        } else {
            "&r18=0"
        };

        url.push_str(&argument);
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

impl Argument for Option<String> {
    fn argument(&self, url: &mut String) {
        if let Some(key) = self {
            let argument = format!("&keyword={}", key);
            url.push_str(&argument);
        }
    }
}

impl Argument for Tag {
    fn argument(&self, url: &mut String) {
        if let Some(ref tag_list) = self.0 {
            for tag in tag_list {
                let argument = format!("&tag={}", tag);
                url.push_str(&argument);
            }
        }
    }
}

impl Argument for Size {
    fn argument(&self, url: &mut String) {
        if let Some(ref size_list) = self.0 {
            for size in size_list {
                let argument = format!("&size={}", size);
                url.push_str(&argument);
            }
        }
    }
}
