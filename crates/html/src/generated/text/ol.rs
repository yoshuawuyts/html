/// The HTML `<ol>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ol)
#[doc(alias = "ol")]
#[non_exhaustive]
pub struct OrderedList {
    sys: html_sys::text::OrderedList,
}
impl crate::categories::FlowContent for OrderedList {}
impl std::convert::Into<html_sys::text::OrderedList> for OrderedList {
    fn into(self) -> html_sys::text::OrderedList {
        self.sys
    }
}
impl From<html_sys::text::OrderedList> for OrderedList {
    fn from(sys: html_sys::text::OrderedList) -> Self {
        Self { sys }
    }
}
