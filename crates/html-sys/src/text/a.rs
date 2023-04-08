/// The HTML `<a>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a)
#[doc(alias = "a")]
#[non_exhaustive]
pub struct Anchor {
    global_attributes: crate::GlobalAttributes,
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
    /// Language of the linked resource
    pub hreflang: std::option::Option<String>,
    /// Hint for the type of the referenced resource
    pub type_: std::option::Option<String>,
    /// Referrer policy for fetches initiated by the element
    pub referrerpolicy: std::option::Option<String>,
}
impl crate::RenderElement for Anchor {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<a")?;
        if let Some(field) = self.href.as_ref() {
            write!(writer, r#""href="{field}""#)?;
        }
        if let Some(field) = self.target.as_ref() {
            write!(writer, r#""target="{field}""#)?;
        }
        if let Some(field) = self.download.as_ref() {
            write!(writer, r#""download="{field}""#)?;
        }
        if let Some(field) = self.ping.as_ref() {
            write!(writer, r#""ping="{field}""#)?;
        }
        if let Some(field) = self.rel.as_ref() {
            write!(writer, r#""rel="{field}""#)?;
        }
        if let Some(field) = self.hreflang.as_ref() {
            write!(writer, r#""hreflang="{field}""#)?;
        }
        if let Some(field) = self.type_.as_ref() {
            write!(writer, r#""type="{field}""#)?;
        }
        if let Some(field) = self.referrerpolicy.as_ref() {
            write!(writer, r#""referrerpolicy="{field}""#)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</a>")?;
        Ok(())
    }
}
impl std::ops::Deref for Anchor {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for Anchor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
