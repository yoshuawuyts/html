/// The HTML `<del>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/del)
#[doc(alias = "del")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct DeletedText {
    global_attrs: crate::GlobalAttributes,
    /// Link to the source of the quotation or more information about the edit
    pub cite: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Date and (optionally) time of the change
    pub date_time: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for DeletedText {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<del")?;
        if let Some(field) = self.cite.as_ref() {
            write!(writer, r#" cite="{field}""#)?;
        }
        if let Some(field) = self.date_time.as_ref() {
            write!(writer, r#" datetime="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</del>")?;
        Ok(())
    }
}
impl std::fmt::Display for DeletedText {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for DeletedText {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for DeletedText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
