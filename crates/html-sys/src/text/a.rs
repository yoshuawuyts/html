/// The HTML `<a>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a)
#[doc(alias = "a")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Anchor {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
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
    /// Language of the linked resource
    pub hreflang: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Hint for the type of the referenced resource
    pub type_: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Referrer policy for fetches initiated by the element
    pub referrerpolicy: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for Anchor {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<a")?;
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
        if let Some(field) = self.hreflang.as_ref() {
            write!(writer, r#" hreflang="{field}""#)?;
        }
        if let Some(field) = self.type_.as_ref() {
            write!(writer, r#" type="{field}""#)?;
        }
        if let Some(field) = self.referrerpolicy.as_ref() {
            write!(writer, r#" referrerpolicy="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</a>")?;
        Ok(())
    }
}
impl std::fmt::Display for Anchor {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Anchor {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Anchor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
