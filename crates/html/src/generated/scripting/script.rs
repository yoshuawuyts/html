/// The HTML `<script>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script)
#[doc(alias = "script")]
#[non_exhaustive]
pub struct Script {
    sys: html_sys::scripting::Script,
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
        Self { sys }
    }
}
