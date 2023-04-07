/// The HTML `<area>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area)
#[doc(alias = "area")]
#[non_exhaustive]
pub struct ImageMapArea {
    global_attributes: crate::GlobalAttributes,
    /// Replacement text for use when images are not available
    pub alt: std::option::Option<String>,
    /// Coordinates for the shape to be created in an image map
    pub coords: std::option::Option<String>,
    /// The kind of shape to be created in an image map
    pub shape: std::option::Option<String>,
    /// Address of the hyperlink
    pub href: std::option::Option<String>,
    /// Navigable for hyperlink navigation
    pub target: std::option::Option<String>,
    /// Whether to download the resource instead of navigating to it, and its filename if so
    pub download: std::option::Option<String>,
    /// URLs to ping
    pub ping: std::option::Option<String>,
    /// Relationship between the location in the document containing the hyperlink and the destination resource
    pub rel: std::option::Option<String>,
    /// Referrer policy for fetches initiated by the element
    pub referrerpolicy: std::option::Option<String>,
}
impl crate::RenderElement for ImageMapArea {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<area")?;
        if let Some(field) = self.alt.as_ref() {
            write!(writer, r#""alt="{}""#, field)?;
        }
        if let Some(field) = self.coords.as_ref() {
            write!(writer, r#""coords="{}""#, field)?;
        }
        if let Some(field) = self.shape.as_ref() {
            write!(writer, r#""shape="{}""#, field)?;
        }
        if let Some(field) = self.href.as_ref() {
            write!(writer, r#""href="{}""#, field)?;
        }
        if let Some(field) = self.target.as_ref() {
            write!(writer, r#""target="{}""#, field)?;
        }
        if let Some(field) = self.download.as_ref() {
            write!(writer, r#""download="{}""#, field)?;
        }
        if let Some(field) = self.ping.as_ref() {
            write!(writer, r#""ping="{}""#, field)?;
        }
        if let Some(field) = self.rel.as_ref() {
            write!(writer, r#""rel="{}""#, field)?;
        }
        if let Some(field) = self.referrerpolicy.as_ref() {
            write!(writer, r#""referrerpolicy="{}""#, field)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        Ok(())
    }
}
impl std::ops::Deref for ImageMapArea {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for ImageMapArea {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
