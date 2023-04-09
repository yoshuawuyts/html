/// The HTML `<datalist>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/datalist)
#[doc(alias = "datalist")]
#[non_exhaustive]
pub struct DataList {
    sys: html_sys::forms::DataList,
    _children: Vec<()>,
}
impl crate::HtmlElement for DataList {}
impl crate::categories::FlowContent for DataList {}
impl crate::categories::PhrasingContent for DataList {}
impl std::convert::Into<html_sys::forms::DataList> for DataList {
    fn into(self) -> html_sys::forms::DataList {
        self.sys
    }
}
impl From<html_sys::forms::DataList> for DataList {
    fn from(sys: html_sys::forms::DataList) -> Self {
        Self { sys, _children: vec![] }
    }
}
