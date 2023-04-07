/// The HTML `<input>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
#[doc(alias = "input")]
pub struct Input {
    accept: std::option::Option<String>,
alt: std::option::Option<String>,
autocomplete: std::option::Option<String>,
checked: std::option::Option<String>,
dirname: std::option::Option<String>,
disabled: std::option::Option<String>,
form: std::option::Option<String>,
formaction: std::option::Option<String>,
formenctype: std::option::Option<String>,
formmethod: std::option::Option<String>,
formnovalidate: std::option::Option<String>,
formtarget: std::option::Option<String>,
height: std::option::Option<String>,
list: std::option::Option<String>,
max: std::option::Option<String>,
maxlength: std::option::Option<String>,
min: std::option::Option<String>,
minlength: std::option::Option<String>,
multiple: std::option::Option<String>,
name: std::option::Option<String>,
pattern: std::option::Option<String>,
placeholder: std::option::Option<String>,
readonly: std::option::Option<String>,
required: std::option::Option<String>,
size: std::option::Option<String>,
src: std::option::Option<String>,
step: std::option::Option<String>,
type_: std::option::Option<String>,
value: std::option::Option<String>,
width: std::option::Option<String>,
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

impl Input {
    /// Get the value of the `accept` attribute.
pub fn accept(&self) -> std::option::Option<&str> {
    self.accept.as_deref()
}

/// Set the value of the `accept` attribute.
pub fn set_accept(&mut self, value: std::option::Option<String>) {
    self.accept = value;
}

/// Get the value of the `alt` attribute.
pub fn alt(&self) -> std::option::Option<&str> {
    self.alt.as_deref()
}

/// Set the value of the `alt` attribute.
pub fn set_alt(&mut self, value: std::option::Option<String>) {
    self.alt = value;
}

/// Get the value of the `autocomplete` attribute.
pub fn autocomplete(&self) -> std::option::Option<&str> {
    self.autocomplete.as_deref()
}

/// Set the value of the `autocomplete` attribute.
pub fn set_autocomplete(&mut self, value: std::option::Option<String>) {
    self.autocomplete = value;
}

/// Get the value of the `checked` attribute.
pub fn checked(&self) -> std::option::Option<&str> {
    self.checked.as_deref()
}

/// Set the value of the `checked` attribute.
pub fn set_checked(&mut self, value: std::option::Option<String>) {
    self.checked = value;
}

/// Get the value of the `dirname` attribute.
pub fn dirname(&self) -> std::option::Option<&str> {
    self.dirname.as_deref()
}

/// Set the value of the `dirname` attribute.
pub fn set_dirname(&mut self, value: std::option::Option<String>) {
    self.dirname = value;
}

/// Get the value of the `disabled` attribute.
pub fn disabled(&self) -> std::option::Option<&str> {
    self.disabled.as_deref()
}

/// Set the value of the `disabled` attribute.
pub fn set_disabled(&mut self, value: std::option::Option<String>) {
    self.disabled = value;
}

/// Get the value of the `form` attribute.
pub fn form(&self) -> std::option::Option<&str> {
    self.form.as_deref()
}

/// Set the value of the `form` attribute.
pub fn set_form(&mut self, value: std::option::Option<String>) {
    self.form = value;
}

/// Get the value of the `formaction` attribute.
pub fn formaction(&self) -> std::option::Option<&str> {
    self.formaction.as_deref()
}

/// Set the value of the `formaction` attribute.
pub fn set_formaction(&mut self, value: std::option::Option<String>) {
    self.formaction = value;
}

/// Get the value of the `formenctype` attribute.
pub fn formenctype(&self) -> std::option::Option<&str> {
    self.formenctype.as_deref()
}

/// Set the value of the `formenctype` attribute.
pub fn set_formenctype(&mut self, value: std::option::Option<String>) {
    self.formenctype = value;
}

/// Get the value of the `formmethod` attribute.
pub fn formmethod(&self) -> std::option::Option<&str> {
    self.formmethod.as_deref()
}

/// Set the value of the `formmethod` attribute.
pub fn set_formmethod(&mut self, value: std::option::Option<String>) {
    self.formmethod = value;
}

/// Get the value of the `formnovalidate` attribute.
pub fn formnovalidate(&self) -> std::option::Option<&str> {
    self.formnovalidate.as_deref()
}

/// Set the value of the `formnovalidate` attribute.
pub fn set_formnovalidate(&mut self, value: std::option::Option<String>) {
    self.formnovalidate = value;
}

/// Get the value of the `formtarget` attribute.
pub fn formtarget(&self) -> std::option::Option<&str> {
    self.formtarget.as_deref()
}

/// Set the value of the `formtarget` attribute.
pub fn set_formtarget(&mut self, value: std::option::Option<String>) {
    self.formtarget = value;
}

/// Get the value of the `height` attribute.
pub fn height(&self) -> std::option::Option<&str> {
    self.height.as_deref()
}

/// Set the value of the `height` attribute.
pub fn set_height(&mut self, value: std::option::Option<String>) {
    self.height = value;
}

/// Get the value of the `list` attribute.
pub fn list(&self) -> std::option::Option<&str> {
    self.list.as_deref()
}

/// Set the value of the `list` attribute.
pub fn set_list(&mut self, value: std::option::Option<String>) {
    self.list = value;
}

/// Get the value of the `max` attribute.
pub fn max(&self) -> std::option::Option<&str> {
    self.max.as_deref()
}

/// Set the value of the `max` attribute.
pub fn set_max(&mut self, value: std::option::Option<String>) {
    self.max = value;
}

/// Get the value of the `maxlength` attribute.
pub fn maxlength(&self) -> std::option::Option<&str> {
    self.maxlength.as_deref()
}

/// Set the value of the `maxlength` attribute.
pub fn set_maxlength(&mut self, value: std::option::Option<String>) {
    self.maxlength = value;
}

/// Get the value of the `min` attribute.
pub fn min(&self) -> std::option::Option<&str> {
    self.min.as_deref()
}

/// Set the value of the `min` attribute.
pub fn set_min(&mut self, value: std::option::Option<String>) {
    self.min = value;
}

/// Get the value of the `minlength` attribute.
pub fn minlength(&self) -> std::option::Option<&str> {
    self.minlength.as_deref()
}

/// Set the value of the `minlength` attribute.
pub fn set_minlength(&mut self, value: std::option::Option<String>) {
    self.minlength = value;
}

/// Get the value of the `multiple` attribute.
pub fn multiple(&self) -> std::option::Option<&str> {
    self.multiple.as_deref()
}

/// Set the value of the `multiple` attribute.
pub fn set_multiple(&mut self, value: std::option::Option<String>) {
    self.multiple = value;
}

/// Get the value of the `name` attribute.
pub fn name(&self) -> std::option::Option<&str> {
    self.name.as_deref()
}

/// Set the value of the `name` attribute.
pub fn set_name(&mut self, value: std::option::Option<String>) {
    self.name = value;
}

/// Get the value of the `pattern` attribute.
pub fn pattern(&self) -> std::option::Option<&str> {
    self.pattern.as_deref()
}

/// Set the value of the `pattern` attribute.
pub fn set_pattern(&mut self, value: std::option::Option<String>) {
    self.pattern = value;
}

/// Get the value of the `placeholder` attribute.
pub fn placeholder(&self) -> std::option::Option<&str> {
    self.placeholder.as_deref()
}

/// Set the value of the `placeholder` attribute.
pub fn set_placeholder(&mut self, value: std::option::Option<String>) {
    self.placeholder = value;
}

/// Get the value of the `readonly` attribute.
pub fn readonly(&self) -> std::option::Option<&str> {
    self.readonly.as_deref()
}

/// Set the value of the `readonly` attribute.
pub fn set_readonly(&mut self, value: std::option::Option<String>) {
    self.readonly = value;
}

/// Get the value of the `required` attribute.
pub fn required(&self) -> std::option::Option<&str> {
    self.required.as_deref()
}

/// Set the value of the `required` attribute.
pub fn set_required(&mut self, value: std::option::Option<String>) {
    self.required = value;
}

/// Get the value of the `size` attribute.
pub fn size(&self) -> std::option::Option<&str> {
    self.size.as_deref()
}

/// Set the value of the `size` attribute.
pub fn set_size(&mut self, value: std::option::Option<String>) {
    self.size = value;
}

/// Get the value of the `src` attribute.
pub fn src(&self) -> std::option::Option<&str> {
    self.src.as_deref()
}

/// Set the value of the `src` attribute.
pub fn set_src(&mut self, value: std::option::Option<String>) {
    self.src = value;
}

/// Get the value of the `step` attribute.
pub fn step(&self) -> std::option::Option<&str> {
    self.step.as_deref()
}

/// Set the value of the `step` attribute.
pub fn set_step(&mut self, value: std::option::Option<String>) {
    self.step = value;
}

/// Get the value of the `type` attribute.
pub fn type_(&self) -> std::option::Option<&str> {
    self.type_.as_deref()
}

/// Set the value of the `type` attribute.
pub fn set_type_(&mut self, value: std::option::Option<String>) {
    self.type_ = value;
}

/// Get the value of the `value` attribute.
pub fn value(&self) -> std::option::Option<&str> {
    self.value.as_deref()
}

/// Set the value of the `value` attribute.
pub fn set_value(&mut self, value: std::option::Option<String>) {
    self.value = value;
}

/// Get the value of the `width` attribute.
pub fn width(&self) -> std::option::Option<&str> {
    self.width.as_deref()
}

/// Set the value of the `width` attribute.
pub fn set_width(&mut self, value: std::option::Option<String>) {
    self.width = value;
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
