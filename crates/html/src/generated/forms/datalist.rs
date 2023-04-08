/// The HTML `<datalist>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/datalist)
#[doc(alias = "datalist")]
#[non_exhaustive]
pub struct DataList {
    _sys: html_sys::forms::DataList,
}
impl crate::categories::FlowContent for DataList {}
impl crate::categories::PhrasingContent for DataList {}
