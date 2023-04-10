/// The HTML `<iframe>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe)
#[doc(alias = "iframe")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Iframe {
    global_attrs: crate::GlobalAttributes,
    /// Address of the resource
    pub src: std::option::Option<std::borrow::Cow<'static, str>>,
    /// A document to render in the iframe
    pub srcdoc: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Name of content navigable
    pub name: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Security rules for nested content
    pub sandbox: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Permissions policy to be applied to the iframe's contents
    pub allow: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether to allow the iframe's contents to use requestFullscreen()
    pub allowfullscreen: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Horizontal dimension
    pub width: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Vertical dimension
    pub height: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Referrer policy for fetches initiated by the element
    pub referrerpolicy: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Used when determining loading deferral
    pub loading: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for Iframe {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<iframe")?;
        if let Some(field) = self.src.as_ref() {
            write!(writer, r#" src="{field}""#)?;
        }
        if let Some(field) = self.srcdoc.as_ref() {
            write!(writer, r#" srcdoc="{field}""#)?;
        }
        if let Some(field) = self.name.as_ref() {
            write!(writer, r#" name="{field}""#)?;
        }
        if let Some(field) = self.sandbox.as_ref() {
            write!(writer, r#" sandbox="{field}""#)?;
        }
        if let Some(field) = self.allow.as_ref() {
            write!(writer, r#" allow="{field}""#)?;
        }
        if let Some(field) = self.allowfullscreen.as_ref() {
            write!(writer, r#" allowfullscreen="{field}""#)?;
        }
        if let Some(field) = self.width.as_ref() {
            write!(writer, r#" width="{field}""#)?;
        }
        if let Some(field) = self.height.as_ref() {
            write!(writer, r#" height="{field}""#)?;
        }
        if let Some(field) = self.referrerpolicy.as_ref() {
            write!(writer, r#" referrerpolicy="{field}""#)?;
        }
        if let Some(field) = self.loading.as_ref() {
            write!(writer, r#" loading="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</iframe>")?;
        Ok(())
    }
}
impl std::fmt::Display for Iframe {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Iframe {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Iframe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
