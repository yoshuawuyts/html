/// The HTML `<meter>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter)
#[doc(alias = "meter")]
#[non_exhaustive]
pub struct Meter {
    sys: html_sys::forms::Meter,
}
impl Meter {
    /// Get the value of the `value` attribute
    pub fn value(&self) -> std::option::Option<&str> {
        self.sys.value.as_deref()
    }
    /// Set the value of the `value` attribute
    pub fn set_value(&mut self, value: std::option::Option<String>) {
        self.sys.value = value;
    }
    /// Get the value of the `min` attribute
    pub fn min(&self) -> std::option::Option<&str> {
        self.sys.min.as_deref()
    }
    /// Set the value of the `min` attribute
    pub fn set_min(&mut self, value: std::option::Option<String>) {
        self.sys.min = value;
    }
    /// Get the value of the `max` attribute
    pub fn max(&self) -> std::option::Option<&str> {
        self.sys.max.as_deref()
    }
    /// Set the value of the `max` attribute
    pub fn set_max(&mut self, value: std::option::Option<String>) {
        self.sys.max = value;
    }
    /// Get the value of the `low` attribute
    pub fn low(&self) -> std::option::Option<&str> {
        self.sys.low.as_deref()
    }
    /// Set the value of the `low` attribute
    pub fn set_low(&mut self, value: std::option::Option<String>) {
        self.sys.low = value;
    }
    /// Get the value of the `high` attribute
    pub fn high(&self) -> std::option::Option<&str> {
        self.sys.high.as_deref()
    }
    /// Set the value of the `high` attribute
    pub fn set_high(&mut self, value: std::option::Option<String>) {
        self.sys.high = value;
    }
    /// Get the value of the `optimum` attribute
    pub fn optimum(&self) -> std::option::Option<&str> {
        self.sys.optimum.as_deref()
    }
    /// Set the value of the `optimum` attribute
    pub fn set_optimum(&mut self, value: std::option::Option<String>) {
        self.sys.optimum = value;
    }
}
impl crate::categories::FlowContent for Meter {}
impl crate::categories::PhrasingContent for Meter {}
impl crate::categories::PalpableContent for Meter {}
impl std::convert::Into<html_sys::forms::Meter> for Meter {
    fn into(self) -> html_sys::forms::Meter {
        self.sys
    }
}
impl From<html_sys::forms::Meter> for Meter {
    fn from(sys: html_sys::forms::Meter) -> Self {
        Self { sys }
    }
}
