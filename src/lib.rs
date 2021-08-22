//! Lolicon API wrapper.
//!
//! Example usage:
//!
//! ```rust
//! use lolicon_api::Request;
//! use lolicon_api::R18;
//! use lolicon_api::ImageSize;
//!
//! let req = Request::default()
//!     .r18(R18::R18) // R-18
//!     .num(1).unwrap() // 一张
//!     .uid(vec![16731]).unwrap() // 玉之けだま老师
//!     .size(vec![ImageSize::Original]).unwrap(); // 原图（默认行为）
//!
//! let req_url = String::from(req);
//!
//! assert_eq!(&req_url, "https://api.lolicon.app/setu/v2?&r18=1&num=1&uid=16731&size=original");
//! ```
//!
//! **Note:** `req_url`'s fields does not graduated to be the same sort as building.


mod convert;
mod datatype;

pub use datatype::Request;
pub use datatype::{R18, ImageSize, LoliError};
