/// The HTML `<area>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area)
#[doc(alias = "area")]
pub struct ImageMapArea {
    alt: std::option::Option<String>,
coords: std::option::Option<String>,
shape: std::option::Option<String>,
href: std::option::Option<String>,
target: std::option::Option<String>,
download: std::option::Option<String>,
ping: std::option::Option<String>,
rel: std::option::Option<String>,
referrerpolicy: std::option::Option<String>,
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

impl ImageMapArea {
    /// Get the value of the `alt` attribute.
pub fn alt(&self) -> std::option::Option<&str> {
    self.alt.as_deref()
}

/// Set the value of the `alt` attribute.
pub fn set_alt(&mut self, value: std::option::Option<String>) {
    self.alt = value;
}

/// Get the value of the `coords` attribute.
pub fn coords(&self) -> std::option::Option<&str> {
    self.coords.as_deref()
}

/// Set the value of the `coords` attribute.
pub fn set_coords(&mut self, value: std::option::Option<String>) {
    self.coords = value;
}

/// Get the value of the `shape` attribute.
pub fn shape(&self) -> std::option::Option<&str> {
    self.shape.as_deref()
}

/// Set the value of the `shape` attribute.
pub fn set_shape(&mut self, value: std::option::Option<String>) {
    self.shape = value;
}

/// Get the value of the `href` attribute.
pub fn href(&self) -> std::option::Option<&str> {
    self.href.as_deref()
}

/// Set the value of the `href` attribute.
pub fn set_href(&mut self, value: std::option::Option<String>) {
    self.href = value;
}

/// Get the value of the `target` attribute.
pub fn target(&self) -> std::option::Option<&str> {
    self.target.as_deref()
}

/// Set the value of the `target` attribute.
pub fn set_target(&mut self, value: std::option::Option<String>) {
    self.target = value;
}

/// Get the value of the `download` attribute.
pub fn download(&self) -> std::option::Option<&str> {
    self.download.as_deref()
}

/// Set the value of the `download` attribute.
pub fn set_download(&mut self, value: std::option::Option<String>) {
    self.download = value;
}

/// Get the value of the `ping` attribute.
pub fn ping(&self) -> std::option::Option<&str> {
    self.ping.as_deref()
}

/// Set the value of the `ping` attribute.
pub fn set_ping(&mut self, value: std::option::Option<String>) {
    self.ping = value;
}

/// Get the value of the `rel` attribute.
pub fn rel(&self) -> std::option::Option<&str> {
    self.rel.as_deref()
}

/// Set the value of the `rel` attribute.
pub fn set_rel(&mut self, value: std::option::Option<String>) {
    self.rel = value;
}

/// Get the value of the `referrerpolicy` attribute.
pub fn referrerpolicy(&self) -> std::option::Option<&str> {
    self.referrerpolicy.as_deref()
}

/// Set the value of the `referrerpolicy` attribute.
pub fn set_referrerpolicy(&mut self, value: std::option::Option<String>) {
    self.referrerpolicy = value;
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
