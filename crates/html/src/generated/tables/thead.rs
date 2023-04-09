/// The HTML `<thead>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/thead)
#[doc(alias = "thead")]
#[non_exhaustive]
pub struct TableHead {
    sys: html_sys::tables::TableHead,
}
impl std::convert::Into<html_sys::tables::TableHead> for TableHead {
    fn into(self) -> html_sys::tables::TableHead {
        self.sys
    }
}
impl From<html_sys::tables::TableHead> for TableHead {
    fn from(sys: html_sys::tables::TableHead) -> Self {
        Self { sys }
    }
}
