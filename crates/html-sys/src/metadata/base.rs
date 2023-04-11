/// The HTML `<base>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base)
#[doc(alias = "base")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Base {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Document base URL
    pub href: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Default navigable for hyperlink navigation and form submission
    pub target: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for Base {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<base")?;
        if let Some(field) = self.href.as_ref() {
            write!(writer, r#" href="{field}""#)?;
        }
        if let Some(field) = self.target.as_ref() {
            write!(writer, r#" target="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        Ok(())
    }
}
impl std::fmt::Display for Base {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Base {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Base {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
