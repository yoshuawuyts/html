/// The HTML `<source>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/source)
#[doc(alias = "source")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MediaSource {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Type of embedded resource
    pub type_: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Applicable media
    pub media: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Address of the resource (in audio or video)
    pub src: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Images to use in different situations, e.g., high-resolution displays, small monitors, etc. (in picture)
    pub srcset: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Image sizes for different page layouts (in picture)
    pub sizes: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Horizontal dimension (in picture)
    pub width: std::option::Option<i64>,
    /// Vertical dimension (in picture)
    pub height: std::option::Option<i64>,
}
impl crate::RenderElement for MediaSource {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<source")?;
        if let Some(field) = self.type_.as_ref() {
            write!(writer, r#" type="{field}""#)?;
        }
        if let Some(field) = self.media.as_ref() {
            write!(writer, r#" media="{field}""#)?;
        }
        if let Some(field) = self.src.as_ref() {
            write!(writer, r#" src="{field}""#)?;
        }
        if let Some(field) = self.srcset.as_ref() {
            write!(writer, r#" srcset="{field}""#)?;
        }
        if let Some(field) = self.sizes.as_ref() {
            write!(writer, r#" sizes="{field}""#)?;
        }
        if let Some(field) = self.width.as_ref() {
            write!(writer, r#" width="{field}""#)?;
        }
        if let Some(field) = self.height.as_ref() {
            write!(writer, r#" height="{field}""#)?;
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
impl std::fmt::Display for MediaSource {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for MediaSource {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for MediaSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
