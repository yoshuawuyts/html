/// The HTML `<iframe>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/iframe)
#[doc(alias = "iframe")]
pub struct Iframe {
    src: std::option::Option<String>,
srcdoc: std::option::Option<String>,
name: std::option::Option<String>,
sandbox: std::option::Option<String>,
allow: std::option::Option<String>,
allowfullscreen: std::option::Option<String>,
width: std::option::Option<String>,
height: std::option::Option<String>,
referrerpolicy: std::option::Option<String>,
loading: std::option::Option<String>,
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

impl Iframe {
    /// Get the value of the `src` attribute.
pub fn src(&self) -> std::option::Option<&str> {
    self.src.as_deref()
}

/// Set the value of the `src` attribute.
pub fn set_src(&mut self, value: std::option::Option<String>) {
    self.src = value;
}

/// Get the value of the `srcdoc` attribute.
pub fn srcdoc(&self) -> std::option::Option<&str> {
    self.srcdoc.as_deref()
}

/// Set the value of the `srcdoc` attribute.
pub fn set_srcdoc(&mut self, value: std::option::Option<String>) {
    self.srcdoc = value;
}

/// Get the value of the `name` attribute.
pub fn name(&self) -> std::option::Option<&str> {
    self.name.as_deref()
}

/// Set the value of the `name` attribute.
pub fn set_name(&mut self, value: std::option::Option<String>) {
    self.name = value;
}

/// Get the value of the `sandbox` attribute.
pub fn sandbox(&self) -> std::option::Option<&str> {
    self.sandbox.as_deref()
}

/// Set the value of the `sandbox` attribute.
pub fn set_sandbox(&mut self, value: std::option::Option<String>) {
    self.sandbox = value;
}

/// Get the value of the `allow` attribute.
pub fn allow(&self) -> std::option::Option<&str> {
    self.allow.as_deref()
}

/// Set the value of the `allow` attribute.
pub fn set_allow(&mut self, value: std::option::Option<String>) {
    self.allow = value;
}

/// Get the value of the `allowfullscreen` attribute.
pub fn allowfullscreen(&self) -> std::option::Option<&str> {
    self.allowfullscreen.as_deref()
}

/// Set the value of the `allowfullscreen` attribute.
pub fn set_allowfullscreen(&mut self, value: std::option::Option<String>) {
    self.allowfullscreen = value;
}

/// Get the value of the `width` attribute.
pub fn width(&self) -> std::option::Option<&str> {
    self.width.as_deref()
}

/// Set the value of the `width` attribute.
pub fn set_width(&mut self, value: std::option::Option<String>) {
    self.width = value;
}

/// Get the value of the `height` attribute.
pub fn height(&self) -> std::option::Option<&str> {
    self.height.as_deref()
}

/// Set the value of the `height` attribute.
pub fn set_height(&mut self, value: std::option::Option<String>) {
    self.height = value;
}

/// Get the value of the `referrerpolicy` attribute.
pub fn referrerpolicy(&self) -> std::option::Option<&str> {
    self.referrerpolicy.as_deref()
}

/// Set the value of the `referrerpolicy` attribute.
pub fn set_referrerpolicy(&mut self, value: std::option::Option<String>) {
    self.referrerpolicy = value;
}

/// Get the value of the `loading` attribute.
pub fn loading(&self) -> std::option::Option<&str> {
    self.loading.as_deref()
}

/// Set the value of the `loading` attribute.
pub fn set_loading(&mut self, value: std::option::Option<String>) {
    self.loading = value;
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
