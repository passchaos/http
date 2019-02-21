#![allow(missing_docs)]

use bytes::Bytes;
use byte_str::ByteStr;

#[derive(Clone, Debug)]
pub struct Fragment {
    has_tag: bool,
    data: ByteStr,
}

impl Fragment {
    pub(super) fn none() -> Self {
        Fragment {
            has_tag: false,
            data: ByteStr::new(),
        }
    }
    pub fn from_shared(s: Bytes) -> Self {
        Fragment {
            has_tag: true,
            data: unsafe { ByteStr::from_utf8_unchecked(s) },
        }
    }

    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        if self.has_tag {
            Some(&self.data[..])
        } else {
            None
        }
    }

    pub fn is_none(&self) -> bool {
        !self.has_tag
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// impl AsRef<str> for Fragment {
//     fn as_ref(&self) -> &str {
//         self.as_str()
//     }
// }

impl PartialEq for Fragment {
    fn eq(&self, other: &Fragment) -> bool {
        self.as_str().eq(&other.as_str())
    }
}

impl Eq for Fragment {}

impl PartialEq<str> for Fragment {
    fn eq(&self, other: &str) -> bool {
        match self.as_str() {
            Some(sstr) => sstr.eq(other),
            None => false,
        }
    }
}

impl PartialEq<Fragment> for str {
    fn eq(&self, other: &Fragment) -> bool {
        match other.as_str() {
            Some(ostr) => self.eq(ostr),
            None => false,
        }
    }
}

impl<'a> PartialEq<Fragment> for &'a str {
    fn eq(&self, other: &Fragment) -> bool {
        match other.as_str() {
            Some(ostr) => self.eq(&ostr),
            None => false,
        }
    }
}

impl<'a> PartialEq<&'a str> for Fragment {
    fn eq(&self, other: &&'a str) -> bool {
        match self.as_str() {
            Some(sstr) => sstr.eq(*other),
            None => false,
        }
    }
}

impl PartialEq<String> for Fragment {
    fn eq(&self, other: &String) -> bool {
        match self.as_str() {
            Some(sstr) => sstr.eq(other),
            None => false,
        }
    }
}

impl PartialEq<Fragment> for String {
    fn eq(&self, other: &Fragment) -> bool {
        match other.as_str() {
            Some(ostr) => self.eq(ostr),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
