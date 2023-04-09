/// The HTML `<script>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script)
#[doc(alias = "script")]
#[non_exhaustive]
pub struct Script {
    sys: html_sys::scripting::Script,
    _children: Vec<()>,
}
impl Script {
    /// Get the value of the `src` attribute
    pub fn src(&self) -> std::option::Option<&str> {
        self.sys.src.as_deref()
    }
    /// Set the value of the `src` attribute
    pub fn set_src(&mut self, value: std::option::Option<String>) {
        self.sys.src = value;
    }
    /// Get the value of the `type` attribute
    pub fn type_(&self) -> std::option::Option<&str> {
        self.sys.type_.as_deref()
    }
    /// Set the value of the `type` attribute
    pub fn set_type_(&mut self, value: std::option::Option<String>) {
        self.sys.type_ = value;
    }
    /// Get the value of the `nomodule` attribute
    pub fn nomodule(&self) -> std::option::Option<&str> {
        self.sys.nomodule.as_deref()
    }
    /// Set the value of the `nomodule` attribute
    pub fn set_nomodule(&mut self, value: std::option::Option<String>) {
        self.sys.nomodule = value;
    }
    /// Get the value of the `async` attribute
    pub fn async_(&self) -> std::option::Option<&str> {
        self.sys.async_.as_deref()
    }
    /// Set the value of the `async` attribute
    pub fn set_async_(&mut self, value: std::option::Option<String>) {
        self.sys.async_ = value;
    }
    /// Get the value of the `defer` attribute
    pub fn defer(&self) -> std::option::Option<&str> {
        self.sys.defer.as_deref()
    }
    /// Set the value of the `defer` attribute
    pub fn set_defer(&mut self, value: std::option::Option<String>) {
        self.sys.defer = value;
    }
    /// Get the value of the `crossorigin` attribute
    pub fn crossorigin(&self) -> std::option::Option<&str> {
        self.sys.crossorigin.as_deref()
    }
    /// Set the value of the `crossorigin` attribute
    pub fn set_crossorigin(&mut self, value: std::option::Option<String>) {
        self.sys.crossorigin = value;
    }
    /// Get the value of the `integrity` attribute
    pub fn integrity(&self) -> std::option::Option<&str> {
        self.sys.integrity.as_deref()
    }
    /// Set the value of the `integrity` attribute
    pub fn set_integrity(&mut self, value: std::option::Option<String>) {
        self.sys.integrity = value;
    }
    /// Get the value of the `referrerpolicy` attribute
    pub fn referrerpolicy(&self) -> std::option::Option<&str> {
        self.sys.referrerpolicy.as_deref()
    }
    /// Set the value of the `referrerpolicy` attribute
    pub fn set_referrerpolicy(&mut self, value: std::option::Option<String>) {
        self.sys.referrerpolicy = value;
    }
    /// Get the value of the `blocking` attribute
    pub fn blocking(&self) -> std::option::Option<&str> {
        self.sys.blocking.as_deref()
    }
    /// Set the value of the `blocking` attribute
    pub fn set_blocking(&mut self, value: std::option::Option<String>) {
        self.sys.blocking = value;
    }
    /// Get the value of the `fetchpriority` attribute
    pub fn fetchpriority(&self) -> std::option::Option<&str> {
        self.sys.fetchpriority.as_deref()
    }
    /// Set the value of the `fetchpriority` attribute
    pub fn set_fetchpriority(&mut self, value: std::option::Option<String>) {
        self.sys.fetchpriority = value;
    }
}
impl crate::categories::MetadataContent for Script {}
impl crate::categories::FlowContent for Script {}
impl crate::categories::PhrasingContent for Script {}
impl std::convert::Into<html_sys::scripting::Script> for Script {
    fn into(self) -> html_sys::scripting::Script {
        self.sys
    }
}
impl From<html_sys::scripting::Script> for Script {
    fn from(sys: html_sys::scripting::Script) -> Self {
        Self { sys, _children: vec![] }
    }
}
