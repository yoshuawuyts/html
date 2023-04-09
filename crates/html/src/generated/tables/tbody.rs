/// The HTML `<tbody>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/tbody)
#[doc(alias = "tbody")]
#[non_exhaustive]
pub struct TableBody {
    sys: html_sys::tables::TableBody,
}
impl std::convert::Into<html_sys::tables::TableBody> for TableBody {
    fn into(self) -> html_sys::tables::TableBody {
        self.sys
    }
}
impl From<html_sys::tables::TableBody> for TableBody {
    fn from(sys: html_sys::tables::TableBody) -> Self {
        Self { sys }
    }
}
