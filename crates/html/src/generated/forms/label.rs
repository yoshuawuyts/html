/// The HTML `<label>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/label)
#[doc(alias = "label")]
#[non_exhaustive]
pub struct Label {
    sys: html_sys::forms::Label,
    _children: Vec<()>,
}
impl Label {
    /// Get the value of the `for` attribute
    pub fn for_(&self) -> std::option::Option<&str> {
        self.sys.for_.as_deref()
    }
    /// Set the value of the `for` attribute
    pub fn set_for_(&mut self, value: std::option::Option<String>) {
        self.sys.for_ = value;
    }
}
impl crate::HtmlElement for Label {}
impl crate::categories::FlowContent for Label {}
impl crate::categories::PhrasingContent for Label {}
impl crate::categories::InteractiveContent for Label {}
impl crate::categories::PalpableContent for Label {}
impl std::convert::Into<html_sys::forms::Label> for Label {
    fn into(self) -> html_sys::forms::Label {
        self.sys
    }
}
impl From<html_sys::forms::Label> for Label {
    fn from(sys: html_sys::forms::Label) -> Self {
        Self { sys, _children: vec![] }
    }
}
