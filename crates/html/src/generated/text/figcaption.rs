/// The HTML `<figcaption>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/figcaption)
#[doc(alias = "figcaption")]
#[non_exhaustive]
pub struct FigureCaption<T: crate::categories::FlowContent> {
    _sys: html_sys::text::FigureCaption,
    _children: Vec<T>,
}
