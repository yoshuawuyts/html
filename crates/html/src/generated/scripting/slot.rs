/// The HTML `<slot>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/slot)
#[doc(alias = "slot")]
#[non_exhaustive]
pub struct Slot {
    sys: html_sys::scripting::Slot,
}
impl Slot {
    /// Get the value of the `name` attribute
    pub fn name(&self) -> std::option::Option<&str> {
        self.sys.name.as_deref()
    }
    /// Set the value of the `name` attribute
    pub fn set_name(&mut self, value: std::option::Option<String>) {
        self.sys.name = value;
    }
}
impl crate::categories::FlowContent for Slot {}
impl crate::categories::PhrasingContent for Slot {}
impl std::convert::Into<html_sys::scripting::Slot> for Slot {
    fn into(self) -> html_sys::scripting::Slot {
        self.sys
    }
}
impl From<html_sys::scripting::Slot> for Slot {
    fn from(sys: html_sys::scripting::Slot) -> Self {
        Self { sys }
    }
}
