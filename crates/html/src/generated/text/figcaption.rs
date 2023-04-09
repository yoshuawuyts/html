/// The HTML `<figcaption>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figcaption)
#[doc(alias = "figcaption")]
#[non_exhaustive]
pub struct FigureCaption {
    sys: html_sys::text::FigureCaption,
    _children: Vec<T>,
}
impl std::convert::Into<html_sys::text::FigureCaption> for FigureCaption {
    fn into(self) -> html_sys::text::FigureCaption {
        self.sys
    }
}
impl From<html_sys::text::FigureCaption> for FigureCaption {
    fn from(sys: html_sys::text::FigureCaption) -> Self {
        Self { sys, _children: vec![] }
    }
}
