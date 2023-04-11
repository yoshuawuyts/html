/// The HTML `<meter>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meter)
#[doc(alias = "meter")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Meter {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Current value of the element
    pub value: std::option::Option<f64>,
    /// Lower bound of range
    pub min: std::option::Option<f64>,
    /// Upper bound of range
    pub max: std::option::Option<f64>,
    /// High limit of low range
    pub low: std::option::Option<f64>,
    /// Low limit of high range
    pub high: std::option::Option<f64>,
    /// Optimum value in gauge
    pub optimum: std::option::Option<f64>,
}
impl crate::RenderElement for Meter {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<meter")?;
        if let Some(field) = self.value.as_ref() {
            write!(writer, r#" value="{field}""#)?;
        }
        if let Some(field) = self.min.as_ref() {
            write!(writer, r#" min="{field}""#)?;
        }
        if let Some(field) = self.max.as_ref() {
            write!(writer, r#" max="{field}""#)?;
        }
        if let Some(field) = self.low.as_ref() {
            write!(writer, r#" low="{field}""#)?;
        }
        if let Some(field) = self.high.as_ref() {
            write!(writer, r#" high="{field}""#)?;
        }
        if let Some(field) = self.optimum.as_ref() {
            write!(writer, r#" optimum="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</meter>")?;
        Ok(())
    }
}
impl std::fmt::Display for Meter {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Meter {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Meter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
