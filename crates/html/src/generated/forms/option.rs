/// The HTML `<option>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/option)
#[doc(alias = "option")]
#[non_exhaustive]
pub struct Option {
    sys: html_sys::forms::Option,
}
impl std::convert::Into<html_sys::forms::Option> for Option {
    fn into(self) -> html_sys::forms::Option {
        self.sys
    }
}
impl From<html_sys::forms::Option> for Option {
    fn from(sys: html_sys::forms::Option) -> Self {
        Self { sys }
    }
}
