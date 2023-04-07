/// The HTML `<link>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link)
#[doc(alias = "link")]
pub struct Link {
    href: std::option::Option<String>,
crossorigin: std::option::Option<String>,
rel: std::option::Option<String>,
media: std::option::Option<String>,
integrity: std::option::Option<String>,
hreflang: std::option::Option<String>,
type_: std::option::Option<String>,
referrerpolicy: std::option::Option<String>,
sizes: std::option::Option<String>,
imagesrcset: std::option::Option<String>,
imagesizes: std::option::Option<String>,
as_: std::option::Option<String>,
blocking: std::option::Option<String>,
color: std::option::Option<String>,
disabled: std::option::Option<String>,
fetchpriority: std::option::Option<String>,
access_key: std::option::Option<String>,
auto_capitalize: std::option::Option<String>,
autofocus: std::option::Option<String>,
content_editable: std::option::Option<String>,
direction: std::option::Option<String>,
draggable: std::option::Option<String>,
enter_key_hint: std::option::Option<String>,
hidden: std::option::Option<String>,
inert: std::option::Option<String>,
input_mode: std::option::Option<String>,
is_: std::option::Option<String>,
item_id: std::option::Option<String>,
item_prop: std::option::Option<String>,
item_ref: std::option::Option<String>,
item_scope: std::option::Option<String>,
item_type: std::option::Option<String>,
lang: std::option::Option<String>,
nonce: std::option::Option<String>,
popover: std::option::Option<String>,
spellcheck: std::option::Option<String>,
style: std::option::Option<String>,
tab_index: std::option::Option<String>,
title: std::option::Option<String>,
translate: std::option::Option<String>,

}

impl Link {
    /// Get the value of the `href` attribute.
pub fn href(&self) -> std::option::Option<&str> {
    self.href.as_deref()
}

/// Set the value of the `href` attribute.
pub fn set_href(&mut self, value: std::option::Option<String>) {
    self.href = value;
}

/// Get the value of the `crossorigin` attribute.
pub fn crossorigin(&self) -> std::option::Option<&str> {
    self.crossorigin.as_deref()
}

/// Set the value of the `crossorigin` attribute.
pub fn set_crossorigin(&mut self, value: std::option::Option<String>) {
    self.crossorigin = value;
}

/// Get the value of the `rel` attribute.
pub fn rel(&self) -> std::option::Option<&str> {
    self.rel.as_deref()
}

/// Set the value of the `rel` attribute.
pub fn set_rel(&mut self, value: std::option::Option<String>) {
    self.rel = value;
}

/// Get the value of the `media` attribute.
pub fn media(&self) -> std::option::Option<&str> {
    self.media.as_deref()
}

/// Set the value of the `media` attribute.
pub fn set_media(&mut self, value: std::option::Option<String>) {
    self.media = value;
}

/// Get the value of the `integrity` attribute.
pub fn integrity(&self) -> std::option::Option<&str> {
    self.integrity.as_deref()
}

/// Set the value of the `integrity` attribute.
pub fn set_integrity(&mut self, value: std::option::Option<String>) {
    self.integrity = value;
}

/// Get the value of the `hreflang` attribute.
pub fn hreflang(&self) -> std::option::Option<&str> {
    self.hreflang.as_deref()
}

/// Set the value of the `hreflang` attribute.
pub fn set_hreflang(&mut self, value: std::option::Option<String>) {
    self.hreflang = value;
}

/// Get the value of the `type` attribute.
pub fn type_(&self) -> std::option::Option<&str> {
    self.type_.as_deref()
}

/// Set the value of the `type` attribute.
pub fn set_type_(&mut self, value: std::option::Option<String>) {
    self.type_ = value;
}

/// Get the value of the `referrerpolicy` attribute.
pub fn referrerpolicy(&self) -> std::option::Option<&str> {
    self.referrerpolicy.as_deref()
}

/// Set the value of the `referrerpolicy` attribute.
pub fn set_referrerpolicy(&mut self, value: std::option::Option<String>) {
    self.referrerpolicy = value;
}

/// Get the value of the `sizes` attribute.
pub fn sizes(&self) -> std::option::Option<&str> {
    self.sizes.as_deref()
}

/// Set the value of the `sizes` attribute.
pub fn set_sizes(&mut self, value: std::option::Option<String>) {
    self.sizes = value;
}

/// Get the value of the `imagesrcset` attribute.
pub fn imagesrcset(&self) -> std::option::Option<&str> {
    self.imagesrcset.as_deref()
}

/// Set the value of the `imagesrcset` attribute.
pub fn set_imagesrcset(&mut self, value: std::option::Option<String>) {
    self.imagesrcset = value;
}

/// Get the value of the `imagesizes` attribute.
pub fn imagesizes(&self) -> std::option::Option<&str> {
    self.imagesizes.as_deref()
}

/// Set the value of the `imagesizes` attribute.
pub fn set_imagesizes(&mut self, value: std::option::Option<String>) {
    self.imagesizes = value;
}

/// Get the value of the `as` attribute.
pub fn as_(&self) -> std::option::Option<&str> {
    self.as_.as_deref()
}

/// Set the value of the `as` attribute.
pub fn set_as_(&mut self, value: std::option::Option<String>) {
    self.as_ = value;
}

/// Get the value of the `blocking` attribute.
pub fn blocking(&self) -> std::option::Option<&str> {
    self.blocking.as_deref()
}

/// Set the value of the `blocking` attribute.
pub fn set_blocking(&mut self, value: std::option::Option<String>) {
    self.blocking = value;
}

/// Get the value of the `color` attribute.
pub fn color(&self) -> std::option::Option<&str> {
    self.color.as_deref()
}

/// Set the value of the `color` attribute.
pub fn set_color(&mut self, value: std::option::Option<String>) {
    self.color = value;
}

/// Get the value of the `disabled` attribute.
pub fn disabled(&self) -> std::option::Option<&str> {
    self.disabled.as_deref()
}

/// Set the value of the `disabled` attribute.
pub fn set_disabled(&mut self, value: std::option::Option<String>) {
    self.disabled = value;
}

/// Get the value of the `fetchpriority` attribute.
pub fn fetchpriority(&self) -> std::option::Option<&str> {
    self.fetchpriority.as_deref()
}

/// Set the value of the `fetchpriority` attribute.
pub fn set_fetchpriority(&mut self, value: std::option::Option<String>) {
    self.fetchpriority = value;
}

/// Get the value of the `accesskey` attribute.
pub fn access_key(&self) -> std::option::Option<&str> {
    self.access_key.as_deref()
}

/// Set the value of the `accesskey` attribute.
pub fn set_access_key(&mut self, value: std::option::Option<String>) {
    self.access_key = value;
}

/// Get the value of the `autocapitalize` attribute.
pub fn auto_capitalize(&self) -> std::option::Option<&str> {
    self.auto_capitalize.as_deref()
}

/// Set the value of the `autocapitalize` attribute.
pub fn set_auto_capitalize(&mut self, value: std::option::Option<String>) {
    self.auto_capitalize = value;
}

/// Get the value of the `autofocus` attribute.
pub fn autofocus(&self) -> std::option::Option<&str> {
    self.autofocus.as_deref()
}

/// Set the value of the `autofocus` attribute.
pub fn set_autofocus(&mut self, value: std::option::Option<String>) {
    self.autofocus = value;
}

/// Get the value of the `contenteditable` attribute.
pub fn content_editable(&self) -> std::option::Option<&str> {
    self.content_editable.as_deref()
}

/// Set the value of the `contenteditable` attribute.
pub fn set_content_editable(&mut self, value: std::option::Option<String>) {
    self.content_editable = value;
}

/// Get the value of the `dir` attribute.
pub fn direction(&self) -> std::option::Option<&str> {
    self.direction.as_deref()
}

/// Set the value of the `dir` attribute.
pub fn set_direction(&mut self, value: std::option::Option<String>) {
    self.direction = value;
}

/// Get the value of the `draggable` attribute.
pub fn draggable(&self) -> std::option::Option<&str> {
    self.draggable.as_deref()
}

/// Set the value of the `draggable` attribute.
pub fn set_draggable(&mut self, value: std::option::Option<String>) {
    self.draggable = value;
}

/// Get the value of the `enterkeyhint` attribute.
pub fn enter_key_hint(&self) -> std::option::Option<&str> {
    self.enter_key_hint.as_deref()
}

/// Set the value of the `enterkeyhint` attribute.
pub fn set_enter_key_hint(&mut self, value: std::option::Option<String>) {
    self.enter_key_hint = value;
}

/// Get the value of the `hidden` attribute.
pub fn hidden(&self) -> std::option::Option<&str> {
    self.hidden.as_deref()
}

/// Set the value of the `hidden` attribute.
pub fn set_hidden(&mut self, value: std::option::Option<String>) {
    self.hidden = value;
}

/// Get the value of the `inert` attribute.
pub fn inert(&self) -> std::option::Option<&str> {
    self.inert.as_deref()
}

/// Set the value of the `inert` attribute.
pub fn set_inert(&mut self, value: std::option::Option<String>) {
    self.inert = value;
}

/// Get the value of the `inputmode` attribute.
pub fn input_mode(&self) -> std::option::Option<&str> {
    self.input_mode.as_deref()
}

/// Set the value of the `inputmode` attribute.
pub fn set_input_mode(&mut self, value: std::option::Option<String>) {
    self.input_mode = value;
}

/// Get the value of the `is` attribute.
pub fn is_(&self) -> std::option::Option<&str> {
    self.is_.as_deref()
}

/// Set the value of the `is` attribute.
pub fn set_is_(&mut self, value: std::option::Option<String>) {
    self.is_ = value;
}

/// Get the value of the `itemid` attribute.
pub fn item_id(&self) -> std::option::Option<&str> {
    self.item_id.as_deref()
}

/// Set the value of the `itemid` attribute.
pub fn set_item_id(&mut self, value: std::option::Option<String>) {
    self.item_id = value;
}

/// Get the value of the `itemprop` attribute.
pub fn item_prop(&self) -> std::option::Option<&str> {
    self.item_prop.as_deref()
}

/// Set the value of the `itemprop` attribute.
pub fn set_item_prop(&mut self, value: std::option::Option<String>) {
    self.item_prop = value;
}

/// Get the value of the `itemref` attribute.
pub fn item_ref(&self) -> std::option::Option<&str> {
    self.item_ref.as_deref()
}

/// Set the value of the `itemref` attribute.
pub fn set_item_ref(&mut self, value: std::option::Option<String>) {
    self.item_ref = value;
}

/// Get the value of the `itemscope` attribute.
pub fn item_scope(&self) -> std::option::Option<&str> {
    self.item_scope.as_deref()
}

/// Set the value of the `itemscope` attribute.
pub fn set_item_scope(&mut self, value: std::option::Option<String>) {
    self.item_scope = value;
}

/// Get the value of the `itemtype` attribute.
pub fn item_type(&self) -> std::option::Option<&str> {
    self.item_type.as_deref()
}

/// Set the value of the `itemtype` attribute.
pub fn set_item_type(&mut self, value: std::option::Option<String>) {
    self.item_type = value;
}

/// Get the value of the `lang` attribute.
pub fn lang(&self) -> std::option::Option<&str> {
    self.lang.as_deref()
}

/// Set the value of the `lang` attribute.
pub fn set_lang(&mut self, value: std::option::Option<String>) {
    self.lang = value;
}

/// Get the value of the `nonce` attribute.
pub fn nonce(&self) -> std::option::Option<&str> {
    self.nonce.as_deref()
}

/// Set the value of the `nonce` attribute.
pub fn set_nonce(&mut self, value: std::option::Option<String>) {
    self.nonce = value;
}

/// Get the value of the `popover` attribute.
pub fn popover(&self) -> std::option::Option<&str> {
    self.popover.as_deref()
}

/// Set the value of the `popover` attribute.
pub fn set_popover(&mut self, value: std::option::Option<String>) {
    self.popover = value;
}

/// Get the value of the `spellcheck` attribute.
pub fn spellcheck(&self) -> std::option::Option<&str> {
    self.spellcheck.as_deref()
}

/// Set the value of the `spellcheck` attribute.
pub fn set_spellcheck(&mut self, value: std::option::Option<String>) {
    self.spellcheck = value;
}

/// Get the value of the `style` attribute.
pub fn style(&self) -> std::option::Option<&str> {
    self.style.as_deref()
}

/// Set the value of the `style` attribute.
pub fn set_style(&mut self, value: std::option::Option<String>) {
    self.style = value;
}

/// Get the value of the `tabindex` attribute.
pub fn tab_index(&self) -> std::option::Option<&str> {
    self.tab_index.as_deref()
}

/// Set the value of the `tabindex` attribute.
pub fn set_tab_index(&mut self, value: std::option::Option<String>) {
    self.tab_index = value;
}

/// Get the value of the `title` attribute.
pub fn title(&self) -> std::option::Option<&str> {
    self.title.as_deref()
}

/// Set the value of the `title` attribute.
pub fn set_title(&mut self, value: std::option::Option<String>) {
    self.title = value;
}

/// Get the value of the `translate` attribute.
pub fn translate(&self) -> std::option::Option<&str> {
    self.translate.as_deref()
}

/// Set the value of the `translate` attribute.
pub fn set_translate(&mut self, value: std::option::Option<String>) {
    self.translate = value;
}


}
