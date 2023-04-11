/// The HTML `<dialog>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog)
#[doc(alias = "dialog")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Dialog {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Whether the dialog box is showing
    pub open: bool,
}
impl crate::RenderElement for Dialog {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<dialog")?;
        if self.open {
            write!(writer, r#" open"#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</dialog>")?;
        Ok(())
    }
}
impl std::fmt::Display for Dialog {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Dialog {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Dialog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
