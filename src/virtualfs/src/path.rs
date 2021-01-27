use std::borrow::Borrow;
use std::ops::Deref;

pub const SEPARATOR: char = '/';

pub fn is_separator(c: char) -> bool {
    c == SEPARATOR
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, Hash)]
pub enum Component<'a> {
    /// The root directory component. This will be the first component if a path starts with
    /// `/`.
    RootDir,

    CurDir,

    ParentDir,

    Named(&'a str),
}

impl<'a> Component<'a> {
    pub(super) fn from_unchecked_name(name: &'a str) -> Self {
        match name {
            "" => Self::CurDir,
            "." => Self::CurDir,
            ".." => Self::ParentDir,
            x => Self::Named(x),
        }
    }
}

impl<'a> Into<&'a str> for Component<'a> {
    fn into(self) -> &'a str {
        match self {
            Self::RootDir => "",
            Self::CurDir => ".",
            Self::ParentDir => "..",
            Self::Named(x) => x,
        }
    }
}

impl<'a> AsRef<str> for Component<'a> {
    fn as_ref(&self) -> &str {
        match self {
            Self::RootDir => "",
            Self::CurDir => ".",
            Self::ParentDir => "..",
            Self::Named(x) => x,
        }
    }
}

impl<'a> AsRef<Path> for Component<'a> {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl<'a> std::fmt::Debug for Component<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(r#"Component("{}")"#, self.as_ref() as &str))
    }
}

impl<'a> std::fmt::Display for Component<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl<'a, A: AsRef<str>> std::cmp::PartialEq<A> for Component<'a> {
    fn eq(&self, other: &A) -> bool {
        (self.as_ref() as &str) == other.as_ref()
    }
}

/// An owned, mutable Path (akin to [`PathBuf`]).
///
/// This path can be converted for free into [`PathBuf`] but can only contain
/// UTF-8 valid characters. This is a design decision to keep [`Path`]s inside
/// the `virtualfs` as platform independent as possible. Because of this,
/// there is not direct conversion from [`PathBuf`] to this type; you will
/// have to handle potential errors.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct OwnedPath {
    inner: String,
}

impl OwnedPath {
    pub fn root() -> Self {
        Self {
            inner: "/".to_owned(),
        }
    }

    pub fn as_path(&self) -> &Path {
        self
    }

    pub fn join<P: AsRef<Path>>(&self, other: P) -> Self {
        let other = other.as_ref();
        if other.is_relative() {
            Self {
                inner: self.inner.clone() + &SEPARATOR.to_string() + &other.inner,
            }
        } else {
            Self {
                inner: self.inner.clone() + &other.inner,
            }
        }
    }
}

impl std::fmt::Debug for OwnedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OwnedPath")
            .field("inner", &self.inner)
            .finish()
    }
}

impl std::fmt::Display for OwnedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.iter() {
            f.write_str(c.as_ref())?;
            f.write_str(&SEPARATOR.to_string())?;
        }
        Ok(())
    }
}

impl From<String> for OwnedPath {
    /// Create a path from a String. This is a zero cost operation.
    fn from(inner: String) -> Self {
        Self { inner }
    }
}

impl Deref for OwnedPath {
    type Target = Path;

    fn deref(&self) -> &Path {
        Path::new(&self.inner)
    }
}

impl Borrow<Path> for OwnedPath {
    fn borrow(&self) -> &Path {
        self.deref()
    }
}

impl AsRef<Path> for OwnedPath {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

/// An unowned, immutable Path (akin to [`Path`]).
#[derive(Debug)]
pub struct Path {
    inner: str,
}

impl Path {
    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> &Path {
        unsafe { &*(s.as_ref() as *const str as *const Path) }
    }

    #[inline]
    pub fn is_absolute(&self) -> bool {
        self.inner.starts_with(is_separator)
    }

    #[inline]
    pub fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    pub fn is_entry(&self) -> bool {
        !self.inner.ends_with(is_separator) && !self.is_empty()
    }
    pub fn is_root(&self) -> bool {
        &self.inner == "/"
    }

    pub fn basename(&self) -> Option<Component> {
        if !self.is_entry() {
            None
        } else {
            self.iter().last()
        }
    }

    pub fn dirname(&self) -> Option<Component> {
        if self.is_empty() || self.is_root() {
            return None;
        }
        let last_slash = if self.is_entry() {
            self.inner[..self.inner.len() - 1].rfind(is_separator)
        } else {
            self.inner.rfind(is_separator)
        };

        if last_slash.is_none() {
            return None;
        }
        let previous_slash = self.inner[0..last_slash.unwrap()].rfind(is_separator);
        if previous_slash.is_none() {
            None
        } else {
            Some(Component::from_unchecked_name(
                &self.inner[previous_slash.unwrap()..last_slash.unwrap()],
            ))
        }
    }

    pub fn parent(&self) -> Option<&Path> {
        if self.is_empty() || self.is_root() {
            None
        } else {
            let last_slash = if self.is_entry() {
                self.inner[..self.inner.len() - 1].rfind(is_separator)
            } else {
                self.inner.rfind(is_separator)
            };

            Some(Path::new(&self.inner[..last_slash.unwrap_or(0) + 1]))
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Component<'_>> {
        let abs = self.is_absolute();
        let head = (if abs { Some(Component::RootDir) } else { None }).into_iter();

        let tail = self.inner.split(is_separator);

        let len = self.inner.split(is_separator).count() - if self.is_entry() { 0 } else { 1 };
        let skip = if abs { 1 } else { 0 };

        head.chain(
            tail.map(Component::from_unchecked_name)
                .skip(skip)
                .take(len - skip),
        )
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl ToOwned for Path {
    type Owned = OwnedPath;

    /// Create a [`OwnedPath`] from a [`Path`].
    fn to_owned(&self) -> Self::Owned {
        OwnedPath::from(self.inner.to_owned())
    }
}

impl AsRef<Path> for str {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl AsRef<str> for Path {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl<A: AsRef<str>> std::cmp::PartialEq<A> for Path {
    fn eq(&self, other: &A) -> bool {
        (self.as_ref() as &str) == other.as_ref()
    }
}

impl std::cmp::PartialEq<str> for &Path {
    fn eq(&self, other: &str) -> bool {
        (self.as_ref() as &str) == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn components() {
        assert_eq!(
            Path::new("/").iter().collect::<Vec<Component>>(),
            [Component::RootDir]
        );
        assert_eq!(
            Path::new("/a").iter().collect::<Vec<Component>>(),
            [Component::RootDir, Component::Named("a")]
        );
        assert_eq!(
            Path::new("/ab/cd/e").iter().collect::<Vec<Component>>(),
            ["", "ab", "cd", "e"]
        );
        assert_eq!(
            Path::new("//a///e").iter().collect::<Vec<Component>>(),
            ["", ".", "a", ".", ".", "e"]
        );
        assert_eq!(
            Path::new("/a/b/cd/e/").iter().collect::<Vec<Component>>(),
            ["", "a", "b", "cd", "e"]
        );
        assert_eq!(
            Path::new("a/b/cd/e").iter().collect::<Vec<Component>>(),
            ["a", "b", "cd", "e"]
        );
    }

    #[test]
    fn to_str() {
        assert_eq!(&Path::new("/hello/world"), "/hello/world");
        assert_eq!(&Path::new("/hello/world/"), "/hello/world/");
    }

    #[test]
    fn basename() {
        assert_eq!(Path::new("").basename(), None);
        assert_eq!(Path::new("/").basename(), None);
        assert_eq!(Path::new("/a").basename().unwrap(), "a");
        assert_eq!(Path::new("/a/").basename(), None);
        assert_eq!(Path::new("/a/b").basename().unwrap(), "b");
    }

    #[test]
    fn dirname() {
        assert!(Path::new("").parent().is_none());
        assert!(Path::new("/").parent().is_none());
        assert_eq!(&Path::new("/a").parent().unwrap(), "/");
        assert_eq!(&Path::new("/a/").parent().unwrap(), "/a/");
        assert_eq!(&Path::new("/a/b").parent().unwrap(), "/a/");
    }
}
