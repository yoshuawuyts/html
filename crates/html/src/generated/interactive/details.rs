/// The HTML `<details>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/details)
#[doc(alias = "details")]
#[non_exhaustive]
pub struct Details {
    sys: html_sys::interactive::Details,
    _children: Vec<()>,
}
impl Details {
    /// Get the value of the `open` attribute
    pub fn open(&self) -> bool {
        self.sys.open
    }
    /// Set the value of the `open` attribute
    pub fn set_open(&mut self, value: bool) {
        self.sys.open = value;
    }
}
impl crate::categories::FlowContent for Details {}
impl crate::categories::InteractiveContent for Details {}
impl crate::categories::PalpableContent for Details {}
impl std::convert::Into<html_sys::interactive::Details> for Details {
    fn into(self) -> html_sys::interactive::Details {
        self.sys
    }
}
impl From<html_sys::interactive::Details> for Details {
    fn from(sys: html_sys::interactive::Details) -> Self {
        Self { sys, _children: vec![] }
    }
}
