/// The HTML `<track>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track)
#[doc(alias = "track")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextTrack {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// The type of text track
    pub kind: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Address of the resource
    pub src: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Language of the text track
    pub srclang: std::option::Option<std::borrow::Cow<'static, str>>,
    /// User-visible label
    pub label: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Enable the track if no other text track is more suitable
    pub default: bool,
}
impl crate::RenderElement for TextTrack {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<track")?;
        if let Some(field) = self.kind.as_ref() {
            write!(writer, r#" kind="{field}""#)?;
        }
        if let Some(field) = self.src.as_ref() {
            write!(writer, r#" src="{field}""#)?;
        }
        if let Some(field) = self.srclang.as_ref() {
            write!(writer, r#" srclang="{field}""#)?;
        }
        if let Some(field) = self.label.as_ref() {
            write!(writer, r#" label="{field}""#)?;
        }
        if self.default {
            write!(writer, r#" default"#)?;
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
impl std::fmt::Display for TextTrack {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for TextTrack {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for TextTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
