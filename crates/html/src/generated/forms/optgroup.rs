/// The HTML `<optgroup>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup)
#[doc(alias = "optgroup")]
#[non_exhaustive]
pub struct OptionGroup {
    sys: html_sys::forms::OptionGroup,
}
impl std::convert::Into<html_sys::forms::OptionGroup> for OptionGroup {
    fn into(self) -> html_sys::forms::OptionGroup {
        self.sys
    }
}
impl From<html_sys::forms::OptionGroup> for OptionGroup {
    fn from(sys: html_sys::forms::OptionGroup) -> Self {
        Self { sys }
    }
}
