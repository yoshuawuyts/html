/// The HTML `<picture>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/picture)
#[doc(alias = "picture")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Picture {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Indicates whether the element is exposed to an accessibility API. See related aria-disabled.
    pub aria_hidden: bool,
}
impl crate::RenderElement for Picture {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<picture")?;
        if self.aria_hidden {
            write!(writer, r#" aria-hidden"#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</picture>")?;
        Ok(())
    }
}
impl std::fmt::Display for Picture {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Picture {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Picture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
