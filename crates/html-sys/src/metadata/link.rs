/// The HTML `<link>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link)
#[doc(alias = "link")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Link {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Address of the hyperlink
    pub href: std::option::Option<std::borrow::Cow<'static, str>>,
    /// How the element handles crossorigin requests
    pub crossorigin: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Relationship between the document containing the hyperlink and the destination resource
    pub rel: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Applicable media
    pub media: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Integrity metadata used in Subresource Integrity checks [SRI]
    pub integrity: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Language of the linked resource
    pub hreflang: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Hint for the type of the referenced resource
    pub type_: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Referrer policy for fetches initiated by the element
    pub referrerpolicy: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Sizes of the icons (for rel="icon")
    pub sizes: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Images to use in different situations, e.g., high-resolution displays, small monitors, etc. (for rel="preload")
    pub imagesrcset: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Image sizes for different page layouts (for rel="preload")
    pub imagesizes: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Potential destination for a preload request (for rel="preload" and rel="modulepreload")
    pub as_: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether the element is potentially render-blocking
    pub blocking: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Color to use when customizing a site's icon (for rel="mask-icon")
    pub color: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether the link is disabled
    pub disabled: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Sets the priority for fetches initiated by the element
    pub fetchpriority: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for Link {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<link")?;
        if let Some(field) = self.href.as_ref() {
            write!(writer, r#" href="{field}""#)?;
        }
        if let Some(field) = self.crossorigin.as_ref() {
            write!(writer, r#" crossorigin="{field}""#)?;
        }
        if let Some(field) = self.rel.as_ref() {
            write!(writer, r#" rel="{field}""#)?;
        }
        if let Some(field) = self.media.as_ref() {
            write!(writer, r#" media="{field}""#)?;
        }
        if let Some(field) = self.integrity.as_ref() {
            write!(writer, r#" integrity="{field}""#)?;
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
        if let Some(field) = self.sizes.as_ref() {
            write!(writer, r#" sizes="{field}""#)?;
        }
        if let Some(field) = self.imagesrcset.as_ref() {
            write!(writer, r#" imagesrcset="{field}""#)?;
        }
        if let Some(field) = self.imagesizes.as_ref() {
            write!(writer, r#" imagesizes="{field}""#)?;
        }
        if let Some(field) = self.as_.as_ref() {
            write!(writer, r#" as="{field}""#)?;
        }
        if let Some(field) = self.blocking.as_ref() {
            write!(writer, r#" blocking="{field}""#)?;
        }
        if let Some(field) = self.color.as_ref() {
            write!(writer, r#" color="{field}""#)?;
        }
        if let Some(field) = self.disabled.as_ref() {
            write!(writer, r#" disabled="{field}""#)?;
        }
        if let Some(field) = self.fetchpriority.as_ref() {
            write!(writer, r#" fetchpriority="{field}""#)?;
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
impl std::fmt::Display for Link {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Link {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Link {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
