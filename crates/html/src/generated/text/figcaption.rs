/// The HTML `<figcaption>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figcaption)
#[doc(alias = "figcaption")]
#[non_exhaustive]
pub struct FigureCaption<T: crate::categories::FlowContent> {
    sys: html_sys::text::FigureCaption,
    _children: Vec<T>,
}
impl<T: crate::categories::FlowContent> std::convert::Into<html_sys::text::FigureCaption>
for FigureCaption<T> {
    fn into(self) -> html_sys::text::FigureCaption {
        self.sys
    }
}
impl<T: crate::categories::FlowContent> From<html_sys::text::FigureCaption>
for FigureCaption<T> {
    fn from(sys: html_sys::text::FigureCaption) -> Self {
        Self { sys, _children: vec![] }
    }
}
