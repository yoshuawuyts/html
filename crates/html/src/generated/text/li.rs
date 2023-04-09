pub mod element {
    /// The HTML `<li>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/li)
    #[doc(alias = "li")]
    #[non_exhaustive]
    #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
    pub struct ListItem {
        sys: html_sys::text::ListItem,
    }
    impl std::fmt::Display for ListItem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            html_sys::RenderElement::write_opening_tag(&self.sys, f)?;
            html_sys::RenderElement::write_closing_tag(&self.sys, f)?;
            Ok(())
        }
    }
    impl crate::HtmlElement for ListItem {}
    impl std::convert::Into<html_sys::text::ListItem> for ListItem {
        fn into(self) -> html_sys::text::ListItem {
            self.sys
        }
    }
    impl From<html_sys::text::ListItem> for ListItem {
        fn from(sys: html_sys::text::ListItem) -> Self {
            Self { sys }
        }
    }
}
pub mod child {}
