/// The HTML `<template>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/template)
#[doc(alias = "template")]
#[non_exhaustive]
pub struct Template {
    sys: html_sys::scripting::Template,
    _children: Vec<()>,
}
impl crate::categories::MetadataContent for Template {}
impl crate::categories::FlowContent for Template {}
impl crate::categories::PhrasingContent for Template {}
impl std::convert::Into<html_sys::scripting::Template> for Template {
    fn into(self) -> html_sys::scripting::Template {
        self.sys
    }
}
impl From<html_sys::scripting::Template> for Template {
    fn from(sys: html_sys::scripting::Template) -> Self {
        Self { sys, _children: vec![] }
    }
}
