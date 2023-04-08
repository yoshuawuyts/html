/// The HTML `<script>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script)
#[doc(alias = "script")]
#[non_exhaustive]
pub struct Script {
    _sys: html_sys::scripting::Script,
}
impl crate::categories::MetadataContent for Script {}
impl crate::categories::FlowContent for Script {}
impl crate::categories::PhrasingContent for Script {}
