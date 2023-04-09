/// The HTML `<menu>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/menu)
#[doc(alias = "menu")]
#[non_exhaustive]
pub struct Menu {
    sys: html_sys::text::Menu,
    _children: Vec<()>,
}
impl crate::HtmlElement for Menu {}
impl crate::categories::FlowContent for Menu {}
impl std::convert::Into<html_sys::text::Menu> for Menu {
    fn into(self) -> html_sys::text::Menu {
        self.sys
    }
}
impl From<html_sys::text::Menu> for Menu {
    fn from(sys: html_sys::text::Menu) -> Self {
        Self { sys, _children: vec![] }
    }
}
