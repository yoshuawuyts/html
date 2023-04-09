/// The HTML `<body>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/body)
#[doc(alias = "body")]
#[non_exhaustive]
pub struct Body<T: crate::categories::FlowContent> {
    _sys: html_sys::sections::Body,
    _children: Vec<T>,
}
