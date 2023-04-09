/// The HTML `<time>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time)
#[doc(alias = "time")]
#[non_exhaustive]
pub struct Time {
    sys: html_sys::text::Time,
}
impl Time {
    /// Get the value of the `datetime` attribute
    pub fn datetime(&self) -> std::option::Option<&str> {
        self.sys.datetime.as_deref()
    }
    /// Set the value of the `datetime` attribute
    pub fn set_datetime(&mut self, value: std::option::Option<String>) {
        self.sys.datetime = value;
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
        Self { sys }
    }
}
