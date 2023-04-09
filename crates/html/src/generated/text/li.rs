/// The HTML `<li>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/li)
#[doc(alias = "li")]
#[non_exhaustive]
pub struct ListItem {
    sys: html_sys::text::ListItem,
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
