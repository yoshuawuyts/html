/// The HTML `<ol>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ol)
#[doc(alias = "ol")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct OrderedList {
    global_attributes: crate::GlobalAttributes,
    /// Number the list backwards
    pub reversed: std::option::Option<String>,
    /// Starting value of the list
    pub start: std::option::Option<String>,
    /// Kind of list marker
    pub type_: std::option::Option<String>,
}
impl crate::RenderElement for OrderedList {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<ol")?;
        if let Some(field) = self.reversed.as_ref() {
            write!(writer, r#""reversed="{field}""#)?;
        }
        if let Some(field) = self.start.as_ref() {
            write!(writer, r#""start="{field}""#)?;
        }
        if let Some(field) = self.type_.as_ref() {
            write!(writer, r#""type="{field}""#)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</ol>")?;
        Ok(())
    }
}
impl std::ops::Deref for OrderedList {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for OrderedList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
