/// The HTML `<canvas>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas)
#[doc(alias = "canvas")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct Canvas {
    global_attrs: crate::GlobalAttributes,
    /// Horizontal dimension
    pub width: std::option::Option<String>,
    /// Vertical dimension
    pub height: std::option::Option<String>,
}
impl crate::RenderElement for Canvas {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<canvas")?;
        if let Some(field) = self.width.as_ref() {
            write!(writer, r#""width="{field}""#)?;
        }
        if let Some(field) = self.height.as_ref() {
            write!(writer, r#""height="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</canvas>")?;
        Ok(())
    }
}
impl std::fmt::Display for Canvas {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Canvas {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Canvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
