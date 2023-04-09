pub mod element {
    /// The HTML `<summary>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/summary)
    #[doc(alias = "summary")]
    #[non_exhaustive]
    pub struct Summary {
        sys: html_sys::interactive::Summary,
    }
    impl crate::HtmlElement for Summary {}
    impl std::convert::Into<html_sys::interactive::Summary> for Summary {
        fn into(self) -> html_sys::interactive::Summary {
            self.sys
        }
    }
    impl From<html_sys::interactive::Summary> for Summary {
        fn from(sys: html_sys::interactive::Summary) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
