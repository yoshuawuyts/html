/// The HTML `<script>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script)
#[doc(alias = "script")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Script {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Address of the resource
    pub src: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Type of script
    pub type_: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Prevents execution in user agents that support module scripts
    pub nomodule: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Execute script when available, without blocking while fetching
    pub async_: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defer script execution
    pub defer: std::option::Option<std::borrow::Cow<'static, str>>,
    /// How the element handles crossorigin requests
    pub crossorigin: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Integrity metadata used in Subresource Integrity checks [SRI]
    pub integrity: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Referrer policy for fetches initiated by the element
    pub referrerpolicy: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether the element is potentially render-blocking
    pub blocking: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Sets the priority for fetches initiated by the element
    pub fetchpriority: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for Script {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<script")?;
        if let Some(field) = self.src.as_ref() {
            write!(writer, r#" src="{field}""#)?;
        }
        if let Some(field) = self.type_.as_ref() {
            write!(writer, r#" type="{field}""#)?;
        }
        if let Some(field) = self.nomodule.as_ref() {
            write!(writer, r#" nomodule="{field}""#)?;
        }
        if let Some(field) = self.async_.as_ref() {
            write!(writer, r#" async="{field}""#)?;
        }
        if let Some(field) = self.defer.as_ref() {
            write!(writer, r#" defer="{field}""#)?;
        }
        if let Some(field) = self.crossorigin.as_ref() {
            write!(writer, r#" crossorigin="{field}""#)?;
        }
        if let Some(field) = self.integrity.as_ref() {
            write!(writer, r#" integrity="{field}""#)?;
        }
        if let Some(field) = self.referrerpolicy.as_ref() {
            write!(writer, r#" referrerpolicy="{field}""#)?;
        }
        if let Some(field) = self.blocking.as_ref() {
            write!(writer, r#" blocking="{field}""#)?;
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
        write!(writer, "</script>")?;
        Ok(())
    }
}
impl std::fmt::Display for Script {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Script {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Script {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
