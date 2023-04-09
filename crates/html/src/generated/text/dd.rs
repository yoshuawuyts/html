/// The HTML `<dd>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dd)
#[doc(alias = "dd")]
#[non_exhaustive]
pub struct DescriptionDetails {
    sys: html_sys::text::DescriptionDetails,
    _children: Vec<T>,
}
impl std::convert::Into<html_sys::text::DescriptionDetails> for DescriptionDetails {
    fn into(self) -> html_sys::text::DescriptionDetails {
        self.sys
    }
}
impl From<html_sys::text::DescriptionDetails> for DescriptionDetails {
    fn from(sys: html_sys::text::DescriptionDetails) -> Self {
        Self { sys, _children: vec![] }
    }
}
