/// The HTML `<track>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/track)
#[doc(alias = "track")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Default)]
pub struct TextTrack {
    global_attributes: crate::GlobalAttributes,
    /// The type of text track
    pub kind: std::option::Option<String>,
    /// Address of the resource
    pub src: std::option::Option<String>,
    /// Language of the text track
    pub srclang: std::option::Option<String>,
    /// User-visible label
    pub label: std::option::Option<String>,
    /// Enable the track if no other text track is more suitable
    pub default: std::option::Option<String>,
}
impl crate::RenderElement for TextTrack {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<track")?;
        if let Some(field) = self.kind.as_ref() {
            write!(writer, r#""kind="{field}""#)?;
        }
        if let Some(field) = self.src.as_ref() {
            write!(writer, r#""src="{field}""#)?;
        }
        if let Some(field) = self.srclang.as_ref() {
            write!(writer, r#""srclang="{field}""#)?;
        }
        if let Some(field) = self.label.as_ref() {
            write!(writer, r#""label="{field}""#)?;
        }
        if let Some(field) = self.default.as_ref() {
            write!(writer, r#""default="{field}""#)?;
        }
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        Ok(())
    }
}
impl std::ops::Deref for TextTrack {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}
impl std::ops::DerefMut for TextTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
