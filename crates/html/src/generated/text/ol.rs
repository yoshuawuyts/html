/// The HTML `<ol>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ol)
#[doc(alias = "ol")]
#[non_exhaustive]
pub struct OrderedList {
    sys: html_sys::text::OrderedList,
    _children: Vec<()>,
}
impl OrderedList {
    /// Get the value of the `reversed` attribute
    pub fn reversed(&self) -> std::option::Option<&str> {
        self.sys.reversed.as_deref()
    }
    /// Set the value of the `reversed` attribute
    pub fn set_reversed(&mut self, value: std::option::Option<String>) {
        self.sys.reversed = value;
    }
    /// Get the value of the `start` attribute
    pub fn start(&self) -> std::option::Option<&str> {
        self.sys.start.as_deref()
    }
    /// Set the value of the `start` attribute
    pub fn set_start(&mut self, value: std::option::Option<String>) {
        self.sys.start = value;
    }
    /// Get the value of the `type` attribute
    pub fn type_(&self) -> std::option::Option<&str> {
        self.sys.type_.as_deref()
    }
    /// Set the value of the `type` attribute
    pub fn set_type_(&mut self, value: std::option::Option<String>) {
        self.sys.type_ = value;
    }
}
impl crate::HtmlElement for OrderedList {}
impl crate::categories::FlowContent for OrderedList {}
impl std::convert::Into<html_sys::text::OrderedList> for OrderedList {
    fn into(self) -> html_sys::text::OrderedList {
        self.sys
    }
}
impl From<html_sys::text::OrderedList> for OrderedList {
    fn from(sys: html_sys::text::OrderedList) -> Self {
        Self { sys, _children: vec![] }
    }
}
