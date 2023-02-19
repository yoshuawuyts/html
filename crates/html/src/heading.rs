use super::HtmlElement;

/// An HTML heading
#[derive(Debug)]
pub struct Heading<T: HtmlElement> {
    level: HeadingLevel,
    inner: T,
}

impl<T: HtmlElement> Heading<T> {
    /// Create a new instance of `Heading`.
    pub fn new(level: HeadingLevel, inner: T) -> Self {
        Self { level, inner }
    }

    /// Create a new `h1` instance of `Heading`.
    pub fn h1(inner: T) -> Self {
        Self::new(HeadingLevel::H1, inner)
    }

    /// Create a new `h2` instance of `Heading`.
    pub fn h2(inner: T) -> Self {
        Self::new(HeadingLevel::H2, inner)
    }

    /// Create a new `h3` instance of `Heading`.
    pub fn h3(inner: T) -> Self {
        Self::new(HeadingLevel::H3, inner)
    }

    /// Create a new `h4` instance of `Heading`.
    pub fn h4(inner: T) -> Self {
        Self::new(HeadingLevel::H4, inner)
    }

    /// Create a new `h5` instance of `Heading`.
    pub fn h5(inner: T) -> Self {
        Self::new(HeadingLevel::H5, inner)
    }

    /// Create a new `h6` instance of `Heading`.
    pub fn h6(inner: T) -> Self {
        Self::new(HeadingLevel::H6, inner)
    }
}

impl<T: HtmlElement> HtmlElement for Heading<T> {}

impl<T: HtmlElement> std::fmt::Display for Heading<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.level {
            HeadingLevel::H1 => write!(f, "<h1>{}</h1>", self.inner),
            HeadingLevel::H2 => write!(f, "<h1>{}</h1>", self.inner),
            HeadingLevel::H3 => write!(f, "<h3>{}</h3>", self.inner),
            HeadingLevel::H4 => write!(f, "<h4>{}</h4>", self.inner),
            HeadingLevel::H5 => write!(f, "<h5>{}</h5>", self.inner),
            HeadingLevel::H6 => write!(f, "<h6>{}</h6>", self.inner),
        }
    }
}

/// An HTML heading level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeadingLevel {
    /// H1 heading
    H1,
    /// H2 heading
    H2,
    /// H3 heading
    H3,
    /// H4 heading
    H4,
    /// H5 heading
    H5,
    /// H6 heading
    H6,
}
