/// The HTML `<figure>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figure)
#[doc(alias = "figure")]
#[non_exhaustive]
pub struct Figure {
    sys: html_sys::text::Figure,
    _children: Vec<()>,
}
impl crate::categories::FlowContent for Figure {}
impl crate::categories::PalpableContent for Figure {}
impl std::convert::Into<html_sys::text::Figure> for Figure {
    fn into(self) -> html_sys::text::Figure {
        self.sys
    }
}
impl From<html_sys::text::Figure> for Figure {
    fn from(sys: html_sys::text::Figure) -> Self {
        Self { sys, _children: vec![] }
    }
}
