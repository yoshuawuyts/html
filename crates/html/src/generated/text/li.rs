/// The HTML `<li>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/li)
#[doc(alias = "li")]
#[non_exhaustive]
pub struct ListItem<T: crate::categories::FlowContent> {
    _sys: html_sys::text::ListItem,
    _children: Vec<T>,
}
