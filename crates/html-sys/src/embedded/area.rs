/// The HTML `<area>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area)
#[doc(alias = "area")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct ImageMapArea {
    global_attrs: crate::GlobalAttributes,
    /// Replacement text for use when images are not available
    pub alt: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Coordinates for the shape to be created in an image map
    pub coords: std::option::Option<std::borrow::Cow<'static, str>>,
    /// The kind of shape to be created in an image map
    pub shape: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Address of the hyperlink
    pub href: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Navigable for hyperlink navigation
    pub target: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether to download the resource instead of navigating to it, and its filename if so
    pub download: std::option::Option<std::borrow::Cow<'static, str>>,
    /// URLs to ping
    pub ping: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Relationship between the location in the document containing the hyperlink and the destination resource
    pub rel: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Referrer policy for fetches initiated by the element
    pub referrerpolicy: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for ImageMapArea {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<area")?;
        if let Some(field) = self.alt.as_ref() {
            write!(writer, r#" alt="{field}""#)?;
        }
        if let Some(field) = self.coords.as_ref() {
            write!(writer, r#" coords="{field}""#)?;
        }
        if let Some(field) = self.shape.as_ref() {
            write!(writer, r#" shape="{field}""#)?;
        }
        if let Some(field) = self.href.as_ref() {
            write!(writer, r#" href="{field}""#)?;
        }
        if let Some(field) = self.target.as_ref() {
            write!(writer, r#" target="{field}""#)?;
        }
        if let Some(field) = self.download.as_ref() {
            write!(writer, r#" download="{field}""#)?;
        }
        if let Some(field) = self.ping.as_ref() {
            write!(writer, r#" ping="{field}""#)?;
        }
        if let Some(field) = self.rel.as_ref() {
            write!(writer, r#" rel="{field}""#)?;
        }
        if let Some(field) = self.referrerpolicy.as_ref() {
            write!(writer, r#" referrerpolicy="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        Ok(())
    }
}
impl std::fmt::Display for ImageMapArea {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for ImageMapArea {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for ImageMapArea {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
