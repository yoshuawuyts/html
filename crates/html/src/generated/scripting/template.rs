/// The HTML `<template>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/template)
#[doc(alias = "template")]
#[non_exhaustive]
pub struct Template {
    _sys: html_sys::scripting::Template,
}
impl crate::categories::MetadataContent for Template {}
impl crate::categories::FlowContent for Template {}
impl crate::categories::PhrasingContent for Template {}
