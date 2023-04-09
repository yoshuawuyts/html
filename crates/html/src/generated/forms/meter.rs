/// The HTML `<meter>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter)
#[doc(alias = "meter")]
#[non_exhaustive]
pub struct Meter {
    sys: html_sys::forms::Meter,
    _children: Vec<()>,
}
impl Meter {
    /// Get the value of the `value` attribute
    pub fn value(&self) -> std::option::Option<f64> {
        self.sys.value
    }
    /// Set the value of the `value` attribute
    pub fn set_value(&mut self, value: std::option::Option<f64>) {
        self.sys.value = value;
    }
    /// Get the value of the `min` attribute
    pub fn min(&self) -> std::option::Option<f64> {
        self.sys.min
    }
    /// Set the value of the `min` attribute
    pub fn set_min(&mut self, value: std::option::Option<f64>) {
        self.sys.min = value;
    }
    /// Get the value of the `max` attribute
    pub fn max(&self) -> std::option::Option<f64> {
        self.sys.max
    }
    /// Set the value of the `max` attribute
    pub fn set_max(&mut self, value: std::option::Option<f64>) {
        self.sys.max = value;
    }
    /// Get the value of the `low` attribute
    pub fn low(&self) -> std::option::Option<f64> {
        self.sys.low
    }
    /// Set the value of the `low` attribute
    pub fn set_low(&mut self, value: std::option::Option<f64>) {
        self.sys.low = value;
    }
    /// Get the value of the `high` attribute
    pub fn high(&self) -> std::option::Option<f64> {
        self.sys.high
    }
    /// Set the value of the `high` attribute
    pub fn set_high(&mut self, value: std::option::Option<f64>) {
        self.sys.high = value;
    }
    /// Get the value of the `optimum` attribute
    pub fn optimum(&self) -> std::option::Option<f64> {
        self.sys.optimum
    }
    /// Set the value of the `optimum` attribute
    pub fn set_optimum(&mut self, value: std::option::Option<f64>) {
        self.sys.optimum = value;
    }
}
impl crate::HtmlElement for Meter {}
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
        Self { sys, _children: vec![] }
    }
}
