/// The HTML `<base>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base)
#[doc(alias = "base")]
#[non_exhaustive]
pub struct Base {
    global_attributes: crate::GlobalAttributes,
    /// Document base URL
    pub href: std::option::Option<String>,
    /// Default navigable for hyperlink navigation and form submission
    pub target: std::option::Option<String>,
}
impl crate::RenderElement for Base {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<base")?;
        if let Some(field) = self.href.as_ref() {
            write!(writer, r#""href="{}""#, field)?;
        }
        if let Some(field) = self.target.as_ref() {
            write!(writer, r#""target="{}""#, field)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        Ok(())
    }
}
impl std::ops::Deref for Base {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for Base {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
