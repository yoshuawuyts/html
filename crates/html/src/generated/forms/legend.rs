pub mod element {
    /// The HTML `<legend>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/legend)
    #[doc(alias = "legend")]
    #[non_exhaustive]
    pub struct Legend {
        sys: html_sys::forms::Legend,
    }
    impl crate::HtmlElement for Legend {}
    impl std::convert::Into<html_sys::forms::Legend> for Legend {
        fn into(self) -> html_sys::forms::Legend {
            self.sys
        }
    }
    impl From<html_sys::forms::Legend> for Legend {
        fn from(sys: html_sys::forms::Legend) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
