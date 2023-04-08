/// The HTML `<dialog>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog)
#[doc(alias = "dialog")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct Dialog {
    global_attributes: crate::GlobalAttributes,
    /// Whether the dialog box is showing
    pub open: std::option::Option<String>,
}
impl crate::RenderElement for Dialog {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<dialog")?;
        if let Some(field) = self.open.as_ref() {
            write!(writer, r#""open="{field}""#)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</dialog>")?;
        Ok(())
    }
}
impl std::ops::Deref for Dialog {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for Dialog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
