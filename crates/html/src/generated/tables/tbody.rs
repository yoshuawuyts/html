pub mod element {
    /// The HTML `<tbody>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tbody)
    #[doc(alias = "tbody")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct TableBody {
        sys: html_sys::tables::TableBody,
    }
    impl std::fmt::Display for TableBody {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for TableBody {}
    impl std::convert::Into<html_sys::tables::TableBody> for TableBody {
        fn into(self) -> html_sys::tables::TableBody {
            self.sys
        }
    }
    impl From<html_sys::tables::TableBody> for TableBody {
        fn from(sys: html_sys::tables::TableBody) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
