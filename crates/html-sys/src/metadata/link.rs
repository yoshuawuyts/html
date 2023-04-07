/// The HTML `<link>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link)
#[doc(alias = "link")]
#[non_exhaustive]
pub struct Link {
    /// Address of the hyperlink
pub href: std::option::Option<String>,
/// How the element handles crossorigin requests
pub crossorigin: std::option::Option<String>,
/// Relationship between the document containing the hyperlink and the destination resource
pub rel: std::option::Option<String>,
/// Applicable media
pub media: std::option::Option<String>,
/// Integrity metadata used in Subresource Integrity checks [SRI]
pub integrity: std::option::Option<String>,
/// Language of the linked resource
pub hreflang: std::option::Option<String>,
/// Hint for the type of the referenced resource
pub type_: std::option::Option<String>,
/// Referrer policy for fetches initiated by the element
pub referrerpolicy: std::option::Option<String>,
/// Sizes of the icons (for rel="icon")
pub sizes: std::option::Option<String>,
/// Images to use in different situations, e.g., high-resolution displays, small monitors, etc. (for rel="preload")
pub imagesrcset: std::option::Option<String>,
/// Image sizes for different page layouts (for rel="preload")
pub imagesizes: std::option::Option<String>,
/// Potential destination for a preload request (for rel="preload" and rel="modulepreload")
pub as_: std::option::Option<String>,
/// Whether the element is potentially render-blocking
pub blocking: std::option::Option<String>,
/// Color to use when customizing a site's icon (for rel="mask-icon")
pub color: std::option::Option<String>,
/// Whether the link is disabled
pub disabled: std::option::Option<String>,
/// Sets the priority for fetches initiated by the element
pub fetchpriority: std::option::Option<String>,
/// 
pub access_key: std::option::Option<String>,
/// 
pub auto_capitalize: std::option::Option<String>,
/// 
pub autofocus: std::option::Option<String>,
/// 
pub class_: std::option::Option<String>,
/// 
pub content_editable: std::option::Option<String>,
/// 
pub direction: std::option::Option<String>,
/// 
pub draggable: std::option::Option<String>,
/// 
pub enter_key_hint: std::option::Option<String>,
/// 
pub hidden: std::option::Option<String>,
/// 
pub id: std::option::Option<String>,
/// 
pub inert: std::option::Option<String>,
/// 
pub input_mode: std::option::Option<String>,
/// 
pub is_: std::option::Option<String>,
/// 
pub item_id: std::option::Option<String>,
/// 
pub item_prop: std::option::Option<String>,
/// 
pub item_ref: std::option::Option<String>,
/// 
pub item_scope: std::option::Option<String>,
/// 
pub item_type: std::option::Option<String>,
/// 
pub lang: std::option::Option<String>,
/// 
pub nonce: std::option::Option<String>,
/// 
pub popover: std::option::Option<String>,
/// 
pub slot: std::option::Option<String>,
/// 
pub spellcheck: std::option::Option<String>,
/// 
pub style: std::option::Option<String>,
/// 
pub tab_index: std::option::Option<String>,
/// 
pub title: std::option::Option<String>,
/// 
pub translate: std::option::Option<String>,

}

impl crate::RenderElement for Link {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<link")?;
if let Some(field) = self.href.as_ref() {
    write!(writer, r#""href="{}""#, field)?;
}
if let Some(field) = self.crossorigin.as_ref() {
    write!(writer, r#""crossorigin="{}""#, field)?;
}
if let Some(field) = self.rel.as_ref() {
    write!(writer, r#""rel="{}""#, field)?;
}
if let Some(field) = self.media.as_ref() {
    write!(writer, r#""media="{}""#, field)?;
}
if let Some(field) = self.integrity.as_ref() {
    write!(writer, r#""integrity="{}""#, field)?;
}
if let Some(field) = self.hreflang.as_ref() {
    write!(writer, r#""hreflang="{}""#, field)?;
}
if let Some(field) = self.type_.as_ref() {
    write!(writer, r#""type="{}""#, field)?;
}
if let Some(field) = self.referrerpolicy.as_ref() {
    write!(writer, r#""referrerpolicy="{}""#, field)?;
}
if let Some(field) = self.sizes.as_ref() {
    write!(writer, r#""sizes="{}""#, field)?;
}
if let Some(field) = self.imagesrcset.as_ref() {
    write!(writer, r#""imagesrcset="{}""#, field)?;
}
if let Some(field) = self.imagesizes.as_ref() {
    write!(writer, r#""imagesizes="{}""#, field)?;
}
if let Some(field) = self.as_.as_ref() {
    write!(writer, r#""as="{}""#, field)?;
}
if let Some(field) = self.blocking.as_ref() {
    write!(writer, r#""blocking="{}""#, field)?;
}
if let Some(field) = self.color.as_ref() {
    write!(writer, r#""color="{}""#, field)?;
}
if let Some(field) = self.disabled.as_ref() {
    write!(writer, r#""disabled="{}""#, field)?;
}
if let Some(field) = self.fetchpriority.as_ref() {
    write!(writer, r#""fetchpriority="{}""#, field)?;
}
if let Some(field) = self.access_key.as_ref() {
    write!(writer, r#""accesskey="{}""#, field)?;
}
if let Some(field) = self.auto_capitalize.as_ref() {
    write!(writer, r#""autocapitalize="{}""#, field)?;
}
if let Some(field) = self.autofocus.as_ref() {
    write!(writer, r#""autofocus="{}""#, field)?;
}
if let Some(field) = self.class_.as_ref() {
    write!(writer, r#""class="{}""#, field)?;
}
if let Some(field) = self.content_editable.as_ref() {
    write!(writer, r#""contenteditable="{}""#, field)?;
}
if let Some(field) = self.direction.as_ref() {
    write!(writer, r#""dir="{}""#, field)?;
}
if let Some(field) = self.draggable.as_ref() {
    write!(writer, r#""draggable="{}""#, field)?;
}
if let Some(field) = self.enter_key_hint.as_ref() {
    write!(writer, r#""enterkeyhint="{}""#, field)?;
}
if let Some(field) = self.hidden.as_ref() {
    write!(writer, r#""hidden="{}""#, field)?;
}
if let Some(field) = self.id.as_ref() {
    write!(writer, r#""id="{}""#, field)?;
}
if let Some(field) = self.inert.as_ref() {
    write!(writer, r#""inert="{}""#, field)?;
}
if let Some(field) = self.input_mode.as_ref() {
    write!(writer, r#""inputmode="{}""#, field)?;
}
if let Some(field) = self.is_.as_ref() {
    write!(writer, r#""is="{}""#, field)?;
}
if let Some(field) = self.item_id.as_ref() {
    write!(writer, r#""itemid="{}""#, field)?;
}
if let Some(field) = self.item_prop.as_ref() {
    write!(writer, r#""itemprop="{}""#, field)?;
}
if let Some(field) = self.item_ref.as_ref() {
    write!(writer, r#""itemref="{}""#, field)?;
}
if let Some(field) = self.item_scope.as_ref() {
    write!(writer, r#""itemscope="{}""#, field)?;
}
if let Some(field) = self.item_type.as_ref() {
    write!(writer, r#""itemtype="{}""#, field)?;
}
if let Some(field) = self.lang.as_ref() {
    write!(writer, r#""lang="{}""#, field)?;
}
if let Some(field) = self.nonce.as_ref() {
    write!(writer, r#""nonce="{}""#, field)?;
}
if let Some(field) = self.popover.as_ref() {
    write!(writer, r#""popover="{}""#, field)?;
}
if let Some(field) = self.slot.as_ref() {
    write!(writer, r#""slot="{}""#, field)?;
}
if let Some(field) = self.spellcheck.as_ref() {
    write!(writer, r#""spellcheck="{}""#, field)?;
}
if let Some(field) = self.style.as_ref() {
    write!(writer, r#""style="{}""#, field)?;
}
if let Some(field) = self.tab_index.as_ref() {
    write!(writer, r#""tabindex="{}""#, field)?;
}
if let Some(field) = self.title.as_ref() {
    write!(writer, r#""title="{}""#, field)?;
}
if let Some(field) = self.translate.as_ref() {
    write!(writer, r#""translate="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        
        Ok(())
    }
}
