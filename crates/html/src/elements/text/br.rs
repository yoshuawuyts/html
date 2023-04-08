/// The HTML `<br>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/br)
#[doc(alias = "br")]
#[non_exhaustive]
pub struct LineBreak {
    sys: html_sys::text::LineBreak,
}
