use crate::Error;

#[allow(dead_code)]
#[derive(Debug, Error, Copy, Clone)]
enum LoliError {
    IlleagalNum,
    IlleagalSize,
    TooManyUid,
}

impl std::fmt::Display for LoliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Request {
    /// Non-R18 by default.
    r18: Option<R18>,
    /// amount of result's artworks. 1-100 is legal.
    num: Option<u8>,
    /// specified authors. at most 20s.
    uid: Option<Vec<u32>>,
    /// Not very conveninent. you should consider use tags instead.
    keyword: Option<String>,
    /// at most 20s
    tag: Option<Vec<String>>,
    /// avaliable values are `response::Object`'s variants.
    size: Option<Vec<String>>,
    /// proxy for `pixiv.net`, `i.pixiv.cat`, e.g.
    proxy: Option<String>,
    /// Only show artworks after this UNIX time in millisecond.
    date_after: Option<u64>,
    /// Only show artworks before this UNIX time in millisecond.
    date_before: Option<u64>,
    /// If this is `true`, some automatic convert between keywords and tags will be disabled.
    dsc: Option<bool>,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum R18 {
    NonR18,
    R18,
    Mixin,
}

#[allow(dead_code)]
impl Request {

    fn r18(mut self, r: R18) -> Self {
        self.r18 = Some(r);
        self
    }

    fn num(mut self, amount: u8) -> Result<Self, LoliError> {
        match amount {
            1..=100 => {
                self.num = Some(amount);
                Ok(self)
            }
            _ => {
                Err(LoliError::IlleagalNum)
            }
        } 
    }

    fn uid(mut self, authors: Vec<u32>) -> Result<Self, LoliError> {
        if authors.len() <= 20 {
            self.uid = Some(authors);
            Ok(self)
        } else {
            Err(LoliError::TooManyUid)
        }
    }

    fn keyword(mut self, keyword: String) -> Self {
        self.keyword = Some(keyword);
        self
    }

}

impl Into<String> for Request {
    fn into(self) -> String {
        let mut url: String = "https://api.lolicon.app/setu/v2?".into();
        url += &self.r18.into_argument();
        url
    }
}

trait IntoArgument {
    fn into_argument(&self) -> String;
}

impl IntoArgument for Option<R18> {
    fn into_argument(&self) -> String {
        if let Some(r) = self {
            match r {
                R18::NonR18 => {
                    "&r18=0"
                }
                R18::R18 => {
                    "&r18=1"
                }
                R18::Mixin => {
                    "&r18=2"
                }
            }
        } else {
            "&r18=0"
        } .into()
    }
}