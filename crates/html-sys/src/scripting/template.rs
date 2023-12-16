/// The HTML `<template>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/template)
#[doc(alias = "template")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Template {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Enables streaming declarative shadow roots
    pub shadow_root_mode: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Sets delegates focus on a declarative shadow root
    pub shadow_root_delegates_focus: bool,
}
impl crate::RenderElement for Template {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<template")?;
        if let Some(field) = self.shadow_root_mode.as_ref() {
            write!(writer, r#" shadowrootmode="{field}""#)?;
        }
        if self.shadow_root_delegates_focus {
            write!(writer, r#" shadowrootdelegatesfocus"#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</template>")?;
        Ok(())
    }
}
impl std::fmt::Display for Template {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Template {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Template {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
