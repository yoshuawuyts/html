/// The HTML `<meta>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta)
#[doc(alias = "meta")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Meta {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Metadata name
    pub name: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Pragma directive
    pub http_equiv: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Value of the element
    pub content: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Character encoding declaration
    pub charset: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Applicable media
    pub media: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for Meta {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<meta")?;
        if let Some(field) = self.name.as_ref() {
            write!(writer, r#" name="{field}""#)?;
        }
        if let Some(field) = self.http_equiv.as_ref() {
            write!(writer, r#" http-equiv="{field}""#)?;
        }
        if let Some(field) = self.content.as_ref() {
            write!(writer, r#" content="{field}""#)?;
        }
        if let Some(field) = self.charset.as_ref() {
            write!(writer, r#" charset="{field}""#)?;
        }
        if let Some(field) = self.media.as_ref() {
            write!(writer, r#" media="{field}""#)?;
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
impl std::fmt::Display for Meta {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Meta {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Meta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
