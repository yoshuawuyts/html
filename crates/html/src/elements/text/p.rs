/// The HTML `<p>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/p)
#[doc(alias = "p")]
#[non_exhaustive]
pub struct Paragraph {
    sys: html_sys::text::Paragraph,
}
