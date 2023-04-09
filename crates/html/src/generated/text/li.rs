/// The HTML `<li>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/li)
#[doc(alias = "li")]
#[non_exhaustive]
pub struct ListItem<T: crate::categories::FlowContent> {
    sys: html_sys::text::ListItem,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> std::convert::Into<html_sys::text::ListItem>
for ListItem<T> {
    fn into(self) -> html_sys::text::ListItem {
        self.sys
    }
}
impl<T: crate::categories::FlowContent> From<html_sys::text::ListItem> for ListItem<T> {
    fn from(sys: html_sys::text::ListItem) -> Self {
        Self { sys, _children: vec![] }
    }
}
