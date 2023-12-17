/** The HTML `<html>` element

 [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/html)*/
#[doc(alias = "html")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Html {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Describes the role(s) the current element plays in the context of the document.
    pub role: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for Html {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "{}<{}", "<!DOCTYPE html>", "html")?;
        if let Some(field) = self.role.as_ref() {
            write!(writer, r#" {}="{field}""#, "role")?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</{}>", "html")?;
        Ok(())
    }
}
impl std::fmt::Display for Html {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Html {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Html {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
