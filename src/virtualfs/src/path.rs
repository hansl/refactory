use std::borrow::Borrow;
use std::ops::Deref;

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

/// An unowned, immutable Path (akin to [`Path`]).
pub struct Path {
    inner: str,
}

impl Path {
    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> &Path {
        unsafe { &*(s.as_ref() as *const str as *const Path) }
    }
}

impl ToOwned for Path {
    type Owned = OwnedPath;

    /// Create a [`OwnedPath`] from a [`Path`].
    fn to_owned(&self) -> Self::Owned {
        OwnedPath::from(self.inner.to_owned())
    }
}
