/// The HTML `<time>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time)
#[doc(alias = "time")]
#[non_exhaustive]
pub struct Time {
    sys: html_sys::text::Time,
    _children: Vec<()>,
}
impl Time {
    /// Get the value of the `datetime` attribute
    pub fn date_time(&self) -> std::option::Option<&str> {
        self.sys.date_time.as_deref()
    }
    /// Set the value of the `datetime` attribute
    pub fn set_date_time(&mut self, value: std::option::Option<String>) {
        self.sys.date_time = value;
    }
}
impl crate::categories::FlowContent for Time {}
impl crate::categories::PhrasingContent for Time {}
impl crate::categories::PalpableContent for Time {}
impl std::convert::Into<html_sys::text::Time> for Time {
    fn into(self) -> html_sys::text::Time {
        self.sys
    }
}
impl From<html_sys::text::Time> for Time {
    fn from(sys: html_sys::text::Time) -> Self {
        Self { sys, _children: vec![] }
    }
}
