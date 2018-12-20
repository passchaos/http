#![allow(missing_docs)]

use bytes::Bytes;
use byte_str::ByteStr;

#[derive(Clone, Debug)]
pub struct Fragment {
    pub(super) data: ByteStr,
}

impl Fragment {
    pub(super) fn empty() -> Self {
        Fragment {
            data: ByteStr::new(),
        }
    }
    pub fn from_shared(s: Bytes) -> Self {
        Fragment {
            data: unsafe { ByteStr::from_utf8_unchecked(s) },
        }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.data[..]
    }
}

impl AsRef<str> for Fragment {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq for Fragment {
    fn eq(&self, other: &Fragment) -> bool {
        self.as_str().eq(other.as_str())
    }
}

impl Eq for Fragment {}

impl PartialEq<str> for Fragment {
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq(other)
    }
}

impl PartialEq<Fragment> for str {
    fn eq(&self, other: &Fragment) -> bool {
        self.eq(other.as_str())
    }
}

impl<'a> PartialEq<Fragment> for &'a str {
    fn eq(&self, other: &Fragment) -> bool {
        self.eq(&other.as_str())
    }
}

impl<'a> PartialEq<&'a str> for Fragment {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str().eq(*other)
    }
}

impl PartialEq<String> for Fragment {
    fn eq(&self, other: &String) -> bool {
        self.as_str().eq(other)
    }
}

impl PartialEq<Fragment> for String {
    fn eq(&self, other: &Fragment) -> bool {
        self.eq(other.as_str())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
