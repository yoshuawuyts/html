/// An HTML Element
pub trait HtmlElement: ::std::fmt::Display {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDirectoryElement<T> {
    deref_target: HTMLElement,
    compact: bool,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLDirectoryElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLDirectoryElement<T> {
    pub fn compact(&self) -> bool {
        self.compact.clone()
    }

    pub fn set_compact(&mut self, value: bool) {
        self.compact = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLDirectoryElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLDirectoryElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<directory>")?;
        write!(f, "</directory>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMapElement<T> {
    deref_target: HTMLElement,
    name: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLMapElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLMapElement<T> {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLMapElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLMapElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<map>")?;
        write!(f, "</map>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct SVGGraphicsElement {
    deref_target: SVGElement,
}

impl ::std::ops::Deref for SVGGraphicsElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl SVGGraphicsElement {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLSpanElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLSpanElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLSpanElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLSpanElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLSpanElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<span>")?;
        write!(f, "</span>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLLegendElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLLegendElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLLegendElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLLegendElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLLegendElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<legend>")?;
        write!(f, "</legend>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLAnchorElement<T> {
    deref_target: HTMLElement,
    target: String,
    download: String,
    rel: String,
    hreflang: String,
    ty: String,
    text: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLAnchorElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLAnchorElement<T> {
    pub fn target(&self) -> String {
        self.target.clone()
    }

    pub fn set_target(&mut self, value: String) {
        self.target = value;
    }
    pub fn download(&self) -> String {
        self.download.clone()
    }

    pub fn set_download(&mut self, value: String) {
        self.download = value;
    }
    pub fn rel(&self) -> String {
        self.rel.clone()
    }

    pub fn set_rel(&mut self, value: String) {
        self.rel = value;
    }
    pub fn hreflang(&self) -> String {
        self.hreflang.clone()
    }

    pub fn set_hreflang(&mut self, value: String) {
        self.hreflang = value;
    }
    pub fn ty(&self) -> String {
        self.ty.clone()
    }

    pub fn set_ty(&mut self, value: String) {
        self.ty = value;
    }
    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, value: String) {
        self.text = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLAnchorElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLAnchorElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<anchor>")?;
        write!(f, "</anchor>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTitleElement<T> {
    deref_target: HTMLElement,
    text: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLTitleElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLTitleElement<T> {
    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, value: String) {
        self.text = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLTitleElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLTitleElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<title>")?;
        write!(f, "</title>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Element {
    deref_target: Node,
    id: String,
    class_name: String,
    slot: String,
}

impl ::std::ops::Deref for Element {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl Element {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn set_id(&mut self, value: String) {
        self.id = value;
    }
    pub fn class_name(&self) -> String {
        self.class_name.clone()
    }

    pub fn set_class_name(&mut self, value: String) {
        self.class_name = value;
    }
    pub fn slot(&self) -> String {
        self.slot.clone()
    }

    pub fn set_slot(&mut self, value: String) {
        self.slot = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTextAreaElement<T> {
    deref_target: HTMLElement,
    autocomplete: String,
    autofocus: bool,
    cols: u64,
    dir_name: String,
    disabled: bool,
    input_mode: String,
    max_length: u64,
    min_length: u64,
    name: String,
    placeholder: String,
    read_only: bool,
    required: bool,
    rows: u64,
    wrap: String,
    default_value: String,
    value: String,
    selection_start: u64,
    selection_end: u64,
    selection_direction: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLTextAreaElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLTextAreaElement<T> {
    pub fn autocomplete(&self) -> String {
        self.autocomplete.clone()
    }

    pub fn set_autocomplete(&mut self, value: String) {
        self.autocomplete = value;
    }
    pub fn autofocus(&self) -> bool {
        self.autofocus.clone()
    }

    pub fn set_autofocus(&mut self, value: bool) {
        self.autofocus = value;
    }
    pub fn cols(&self) -> u64 {
        self.cols.clone()
    }

    pub fn set_cols(&mut self, value: u64) {
        self.cols = value;
    }
    pub fn dir_name(&self) -> String {
        self.dir_name.clone()
    }

    pub fn set_dir_name(&mut self, value: String) {
        self.dir_name = value;
    }
    pub fn disabled(&self) -> bool {
        self.disabled.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.disabled = value;
    }
    pub fn input_mode(&self) -> String {
        self.input_mode.clone()
    }

    pub fn set_input_mode(&mut self, value: String) {
        self.input_mode = value;
    }
    pub fn max_length(&self) -> u64 {
        self.max_length.clone()
    }

    pub fn set_max_length(&mut self, value: u64) {
        self.max_length = value;
    }
    pub fn min_length(&self) -> u64 {
        self.min_length.clone()
    }

    pub fn set_min_length(&mut self, value: u64) {
        self.min_length = value;
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn placeholder(&self) -> String {
        self.placeholder.clone()
    }

    pub fn set_placeholder(&mut self, value: String) {
        self.placeholder = value;
    }
    pub fn read_only(&self) -> bool {
        self.read_only.clone()
    }

    pub fn set_read_only(&mut self, value: bool) {
        self.read_only = value;
    }
    pub fn required(&self) -> bool {
        self.required.clone()
    }

    pub fn set_required(&mut self, value: bool) {
        self.required = value;
    }
    pub fn rows(&self) -> u64 {
        self.rows.clone()
    }

    pub fn set_rows(&mut self, value: u64) {
        self.rows = value;
    }
    pub fn wrap(&self) -> String {
        self.wrap.clone()
    }

    pub fn set_wrap(&mut self, value: String) {
        self.wrap = value;
    }
    pub fn default_value(&self) -> String {
        self.default_value.clone()
    }

    pub fn set_default_value(&mut self, value: String) {
        self.default_value = value;
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
    pub fn selection_start(&self) -> u64 {
        self.selection_start.clone()
    }

    pub fn set_selection_start(&mut self, value: u64) {
        self.selection_start = value;
    }
    pub fn selection_end(&self) -> u64 {
        self.selection_end.clone()
    }

    pub fn set_selection_end(&mut self, value: u64) {
        self.selection_end = value;
    }
    pub fn selection_direction(&self) -> String {
        self.selection_direction.clone()
    }

    pub fn set_selection_direction(&mut self, value: String) {
        self.selection_direction = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLTextAreaElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLTextAreaElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<textarea>")?;
        write!(f, "</textarea>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLEmbedElement<T> {
    deref_target: HTMLElement,
    src: String,
    ty: String,
    width: String,
    height: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLEmbedElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLEmbedElement<T> {
    pub fn src(&self) -> String {
        self.src.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.src = value;
    }
    pub fn ty(&self) -> String {
        self.ty.clone()
    }

    pub fn set_ty(&mut self, value: String) {
        self.ty = value;
    }
    pub fn width(&self) -> String {
        self.width.clone()
    }

    pub fn set_width(&mut self, value: String) {
        self.width = value;
    }
    pub fn height(&self) -> String {
        self.height.clone()
    }

    pub fn set_height(&mut self, value: String) {
        self.height = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLEmbedElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLEmbedElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<embed>")?;
        write!(f, "</embed>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDataListElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLDataListElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLDataListElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLDataListElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLDataListElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<datalist>")?;
        write!(f, "</datalist>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLOutputElement<T> {
    deref_target: HTMLElement,
    name: String,
    default_value: String,
    value: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLOutputElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLOutputElement<T> {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn default_value(&self) -> String {
        self.default_value.clone()
    }

    pub fn set_default_value(&mut self, value: String) {
        self.default_value = value;
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLOutputElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLOutputElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<output>")?;
        write!(f, "</output>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLParagraphElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLParagraphElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLParagraphElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLParagraphElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLParagraphElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<paragraph>")?;
        write!(f, "</paragraph>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLHtmlElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLHtmlElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLHtmlElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLHtmlElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLHtmlElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<html>")?;
        write!(f, "</html>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLElement {
    deref_target: Element,
    title: String,
    lang: String,
    translate: bool,
    dir: String,
    hidden: bool,
    access_key: String,
    draggable: bool,
}

impl ::std::ops::Deref for HTMLElement {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLElement {
    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, value: String) {
        self.title = value;
    }
    pub fn lang(&self) -> String {
        self.lang.clone()
    }

    pub fn set_lang(&mut self, value: String) {
        self.lang = value;
    }
    pub fn translate(&self) -> bool {
        self.translate.clone()
    }

    pub fn set_translate(&mut self, value: bool) {
        self.translate = value;
    }
    pub fn dir(&self) -> String {
        self.dir.clone()
    }

    pub fn set_dir(&mut self, value: String) {
        self.dir = value;
    }
    pub fn hidden(&self) -> bool {
        self.hidden.clone()
    }

    pub fn set_hidden(&mut self, value: bool) {
        self.hidden = value;
    }
    pub fn access_key(&self) -> String {
        self.access_key.clone()
    }

    pub fn set_access_key(&mut self, value: String) {
        self.access_key = value;
    }
    pub fn draggable(&self) -> bool {
        self.draggable.clone()
    }

    pub fn set_draggable(&mut self, value: bool) {
        self.draggable = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLAreaElement<T> {
    deref_target: HTMLElement,
    alt: String,
    coords: String,
    shape: String,
    target: String,
    rel: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLAreaElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLAreaElement<T> {
    pub fn alt(&self) -> String {
        self.alt.clone()
    }

    pub fn set_alt(&mut self, value: String) {
        self.alt = value;
    }
    pub fn coords(&self) -> String {
        self.coords.clone()
    }

    pub fn set_coords(&mut self, value: String) {
        self.coords = value;
    }
    pub fn shape(&self) -> String {
        self.shape.clone()
    }

    pub fn set_shape(&mut self, value: String) {
        self.shape = value;
    }
    pub fn target(&self) -> String {
        self.target.clone()
    }

    pub fn set_target(&mut self, value: String) {
        self.target = value;
    }
    pub fn rel(&self) -> String {
        self.rel.clone()
    }

    pub fn set_rel(&mut self, value: String) {
        self.rel = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLAreaElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLAreaElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<area>")?;
        write!(f, "</area>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLBaseElement<T> {
    deref_target: HTMLElement,
    href: String,
    target: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLBaseElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLBaseElement<T> {
    pub fn href(&self) -> String {
        self.href.clone()
    }

    pub fn set_href(&mut self, value: String) {
        self.href = value;
    }
    pub fn target(&self) -> String {
        self.target.clone()
    }

    pub fn set_target(&mut self, value: String) {
        self.target = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLBaseElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLBaseElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<base>")?;
        write!(f, "</base>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Text {
    deref_target: CharacterData,
}

impl ::std::ops::Deref for Text {
    type Target = CharacterData;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl Text {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLOListElement<T> {
    deref_target: HTMLElement,
    reversed: bool,
    start: u64,
    ty: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLOListElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLOListElement<T> {
    pub fn reversed(&self) -> bool {
        self.reversed.clone()
    }

    pub fn set_reversed(&mut self, value: bool) {
        self.reversed = value;
    }
    pub fn start(&self) -> u64 {
        self.start.clone()
    }

    pub fn set_start(&mut self, value: u64) {
        self.start = value;
    }
    pub fn ty(&self) -> String {
        self.ty.clone()
    }

    pub fn set_ty(&mut self, value: String) {
        self.ty = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLOListElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLOListElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<olist>")?;
        write!(f, "</olist>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLFontElement<T> {
    deref_target: HTMLElement,
    color: String,
    face: String,
    size: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLFontElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLFontElement<T> {
    pub fn color(&self) -> String {
        self.color.clone()
    }

    pub fn set_color(&mut self, value: String) {
        self.color = value;
    }
    pub fn face(&self) -> String {
        self.face.clone()
    }

    pub fn set_face(&mut self, value: String) {
        self.face = value;
    }
    pub fn size(&self) -> String {
        self.size.clone()
    }

    pub fn set_size(&mut self, value: String) {
        self.size = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLFontElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLFontElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<font>")?;
        write!(f, "</font>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct DocumentFragment {
    deref_target: Node,
}

impl ::std::ops::Deref for DocumentFragment {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl DocumentFragment {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLUListElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLUListElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLUListElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLUListElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLUListElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<ulist>")?;
        write!(f, "</ulist>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLSlotElement<T> {
    deref_target: HTMLElement,
    name: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLSlotElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLSlotElement<T> {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLSlotElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLSlotElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<slot>")?;
        write!(f, "</slot>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct DOMImplementation {}

impl DOMImplementation {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLImageElement<T> {
    deref_target: HTMLElement,
    alt: String,
    src: String,
    srcset: String,
    sizes: String,
    cross_origin: String,
    use_map: String,
    is_map: bool,
    width: u64,
    height: u64,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLImageElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLImageElement<T> {
    pub fn alt(&self) -> String {
        self.alt.clone()
    }

    pub fn set_alt(&mut self, value: String) {
        self.alt = value;
    }
    pub fn src(&self) -> String {
        self.src.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.src = value;
    }
    pub fn srcset(&self) -> String {
        self.srcset.clone()
    }

    pub fn set_srcset(&mut self, value: String) {
        self.srcset = value;
    }
    pub fn sizes(&self) -> String {
        self.sizes.clone()
    }

    pub fn set_sizes(&mut self, value: String) {
        self.sizes = value;
    }
    pub fn cross_origin(&self) -> String {
        self.cross_origin.clone()
    }

    pub fn set_cross_origin(&mut self, value: String) {
        self.cross_origin = value;
    }
    pub fn use_map(&self) -> String {
        self.use_map.clone()
    }

    pub fn set_use_map(&mut self, value: String) {
        self.use_map = value;
    }
    pub fn is_map(&self) -> bool {
        self.is_map.clone()
    }

    pub fn set_is_map(&mut self, value: bool) {
        self.is_map = value;
    }
    pub fn width(&self) -> u64 {
        self.width.clone()
    }

    pub fn set_width(&mut self, value: u64) {
        self.width = value;
    }
    pub fn height(&self) -> u64 {
        self.height.clone()
    }

    pub fn set_height(&mut self, value: u64) {
        self.height = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLImageElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLImageElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<image>")?;
        write!(f, "</image>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLInputElement<T> {
    deref_target: HTMLElement,
    accept: String,
    alt: String,
    autocomplete: String,
    autofocus: bool,
    default_checked: bool,
    checked: bool,
    dir_name: String,
    disabled: bool,
    files: FileList,
    form_no_validate: bool,
    form_target: String,
    indeterminate: bool,
    input_mode: String,
    max: String,
    max_length: u64,
    min: String,
    min_length: u64,
    multiple: bool,
    name: String,
    pattern: String,
    placeholder: String,
    read_only: bool,
    required: bool,
    size: u64,
    src: String,
    step: String,
    ty: String,
    default_value: String,
    value: String,
    value_as_number: f64,
    selection_start: u64,
    selection_end: u64,
    selection_direction: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLInputElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLInputElement<T> {
    pub fn accept(&self) -> String {
        self.accept.clone()
    }

    pub fn set_accept(&mut self, value: String) {
        self.accept = value;
    }
    pub fn alt(&self) -> String {
        self.alt.clone()
    }

    pub fn set_alt(&mut self, value: String) {
        self.alt = value;
    }
    pub fn autocomplete(&self) -> String {
        self.autocomplete.clone()
    }

    pub fn set_autocomplete(&mut self, value: String) {
        self.autocomplete = value;
    }
    pub fn autofocus(&self) -> bool {
        self.autofocus.clone()
    }

    pub fn set_autofocus(&mut self, value: bool) {
        self.autofocus = value;
    }
    pub fn default_checked(&self) -> bool {
        self.default_checked.clone()
    }

    pub fn set_default_checked(&mut self, value: bool) {
        self.default_checked = value;
    }
    pub fn checked(&self) -> bool {
        self.checked.clone()
    }

    pub fn set_checked(&mut self, value: bool) {
        self.checked = value;
    }
    pub fn dir_name(&self) -> String {
        self.dir_name.clone()
    }

    pub fn set_dir_name(&mut self, value: String) {
        self.dir_name = value;
    }
    pub fn disabled(&self) -> bool {
        self.disabled.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.disabled = value;
    }
    pub fn files(&self) -> FileList {
        self.files.clone()
    }

    pub fn set_files(&mut self, value: FileList) {
        self.files = value;
    }
    pub fn form_no_validate(&self) -> bool {
        self.form_no_validate.clone()
    }

    pub fn set_form_no_validate(&mut self, value: bool) {
        self.form_no_validate = value;
    }
    pub fn form_target(&self) -> String {
        self.form_target.clone()
    }

    pub fn set_form_target(&mut self, value: String) {
        self.form_target = value;
    }
    pub fn indeterminate(&self) -> bool {
        self.indeterminate.clone()
    }

    pub fn set_indeterminate(&mut self, value: bool) {
        self.indeterminate = value;
    }
    pub fn input_mode(&self) -> String {
        self.input_mode.clone()
    }

    pub fn set_input_mode(&mut self, value: String) {
        self.input_mode = value;
    }
    pub fn max(&self) -> String {
        self.max.clone()
    }

    pub fn set_max(&mut self, value: String) {
        self.max = value;
    }
    pub fn max_length(&self) -> u64 {
        self.max_length.clone()
    }

    pub fn set_max_length(&mut self, value: u64) {
        self.max_length = value;
    }
    pub fn min(&self) -> String {
        self.min.clone()
    }

    pub fn set_min(&mut self, value: String) {
        self.min = value;
    }
    pub fn min_length(&self) -> u64 {
        self.min_length.clone()
    }

    pub fn set_min_length(&mut self, value: u64) {
        self.min_length = value;
    }
    pub fn multiple(&self) -> bool {
        self.multiple.clone()
    }

    pub fn set_multiple(&mut self, value: bool) {
        self.multiple = value;
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn pattern(&self) -> String {
        self.pattern.clone()
    }

    pub fn set_pattern(&mut self, value: String) {
        self.pattern = value;
    }
    pub fn placeholder(&self) -> String {
        self.placeholder.clone()
    }

    pub fn set_placeholder(&mut self, value: String) {
        self.placeholder = value;
    }
    pub fn read_only(&self) -> bool {
        self.read_only.clone()
    }

    pub fn set_read_only(&mut self, value: bool) {
        self.read_only = value;
    }
    pub fn required(&self) -> bool {
        self.required.clone()
    }

    pub fn set_required(&mut self, value: bool) {
        self.required = value;
    }
    pub fn size(&self) -> u64 {
        self.size.clone()
    }

    pub fn set_size(&mut self, value: u64) {
        self.size = value;
    }
    pub fn src(&self) -> String {
        self.src.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.src = value;
    }
    pub fn step(&self) -> String {
        self.step.clone()
    }

    pub fn set_step(&mut self, value: String) {
        self.step = value;
    }
    pub fn ty(&self) -> String {
        self.ty.clone()
    }

    pub fn set_ty(&mut self, value: String) {
        self.ty = value;
    }
    pub fn default_value(&self) -> String {
        self.default_value.clone()
    }

    pub fn set_default_value(&mut self, value: String) {
        self.default_value = value;
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
    pub fn value_as_number(&self) -> f64 {
        self.value_as_number.clone()
    }

    pub fn set_value_as_number(&mut self, value: f64) {
        self.value_as_number = value;
    }
    pub fn selection_start(&self) -> u64 {
        self.selection_start.clone()
    }

    pub fn set_selection_start(&mut self, value: u64) {
        self.selection_start = value;
    }
    pub fn selection_end(&self) -> u64 {
        self.selection_end.clone()
    }

    pub fn set_selection_end(&mut self, value: u64) {
        self.selection_end = value;
    }
    pub fn selection_direction(&self) -> String {
        self.selection_direction.clone()
    }

    pub fn set_selection_direction(&mut self, value: String) {
        self.selection_direction = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLInputElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLInputElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<input>")?;
        write!(f, "</input>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLBRElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLBRElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLBRElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLBRElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLBRElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<br>")?;
        write!(f, "</br>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLButtonElement<T> {
    deref_target: HTMLElement,
    autofocus: bool,
    disabled: bool,
    form_no_validate: bool,
    form_target: String,
    name: String,
    ty: String,
    value: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLButtonElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLButtonElement<T> {
    pub fn autofocus(&self) -> bool {
        self.autofocus.clone()
    }

    pub fn set_autofocus(&mut self, value: bool) {
        self.autofocus = value;
    }
    pub fn disabled(&self) -> bool {
        self.disabled.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.disabled = value;
    }
    pub fn form_no_validate(&self) -> bool {
        self.form_no_validate.clone()
    }

    pub fn set_form_no_validate(&mut self, value: bool) {
        self.form_no_validate = value;
    }
    pub fn form_target(&self) -> String {
        self.form_target.clone()
    }

    pub fn set_form_target(&mut self, value: String) {
        self.form_target = value;
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn ty(&self) -> String {
        self.ty.clone()
    }

    pub fn set_ty(&mut self, value: String) {
        self.ty = value;
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLButtonElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLButtonElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<button>")?;
        write!(f, "</button>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTemplateElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLTemplateElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLTemplateElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLTemplateElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLTemplateElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<template>")?;
        write!(f, "</template>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLHeadingElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLHeadingElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLHeadingElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLHeadingElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLHeadingElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<heading>")?;
        write!(f, "</heading>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct DOMTokenList {
    value: String,
}

impl DOMTokenList {
    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLFrameSetElement<T> {
    deref_target: HTMLElement,
    cols: String,
    rows: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLFrameSetElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLFrameSetElement<T> {
    pub fn cols(&self) -> String {
        self.cols.clone()
    }

    pub fn set_cols(&mut self, value: String) {
        self.cols = value;
    }
    pub fn rows(&self) -> String {
        self.rows.clone()
    }

    pub fn set_rows(&mut self, value: String) {
        self.rows = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLFrameSetElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLFrameSetElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<frameset>")?;
        write!(f, "</frameset>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Document {
    deref_target: Node,
}

impl ::std::ops::Deref for Document {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl Document {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct DOMStringMap {}

impl DOMStringMap {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableElement<T> {
    deref_target: HTMLElement,
    caption: HTMLTableCaptionElement<T>,
    t_head: HTMLTableSectionElement<T>,
    t_foot: HTMLTableSectionElement<T>,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLTableElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLTableElement<T> {
    pub fn caption(&self) -> HTMLTableCaptionElement<T> {
        self.caption.clone()
    }

    pub fn set_caption(&mut self, value: HTMLTableCaptionElement<T>) {
        self.caption = value;
    }
    pub fn t_head(&self) -> HTMLTableSectionElement<T> {
        self.t_head.clone()
    }

    pub fn set_t_head(&mut self, value: HTMLTableSectionElement<T>) {
        self.t_head = value;
    }
    pub fn t_foot(&self) -> HTMLTableSectionElement<T> {
        self.t_foot.clone()
    }

    pub fn set_t_foot(&mut self, value: HTMLTableSectionElement<T>) {
        self.t_foot = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLTableElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLTableElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<table>")?;
        write!(f, "</table>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLFrameElement<T> {
    deref_target: HTMLElement,
    name: String,
    scrolling: String,
    src: String,
    frame_border: String,
    long_desc: String,
    no_resize: bool,
    margin_height: String,
    margin_width: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLFrameElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLFrameElement<T> {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn scrolling(&self) -> String {
        self.scrolling.clone()
    }

    pub fn set_scrolling(&mut self, value: String) {
        self.scrolling = value;
    }
    pub fn src(&self) -> String {
        self.src.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.src = value;
    }
    pub fn frame_border(&self) -> String {
        self.frame_border.clone()
    }

    pub fn set_frame_border(&mut self, value: String) {
        self.frame_border = value;
    }
    pub fn long_desc(&self) -> String {
        self.long_desc.clone()
    }

    pub fn set_long_desc(&mut self, value: String) {
        self.long_desc = value;
    }
    pub fn no_resize(&self) -> bool {
        self.no_resize.clone()
    }

    pub fn set_no_resize(&mut self, value: bool) {
        self.no_resize = value;
    }
    pub fn margin_height(&self) -> String {
        self.margin_height.clone()
    }

    pub fn set_margin_height(&mut self, value: String) {
        self.margin_height = value;
    }
    pub fn margin_width(&self) -> String {
        self.margin_width.clone()
    }

    pub fn set_margin_width(&mut self, value: String) {
        self.margin_width = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLFrameElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLFrameElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<frame>")?;
        write!(f, "</frame>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct CharacterData {
    deref_target: Node,
    data: String,
}

impl ::std::ops::Deref for CharacterData {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl CharacterData {
    pub fn data(&self) -> String {
        self.data.clone()
    }

    pub fn set_data(&mut self, value: String) {
        self.data = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct SVGSVGElement {
    deref_target: SVGGraphicsElement,
}

impl ::std::ops::Deref for SVGSVGElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl SVGSVGElement {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLObjectElement<T> {
    deref_target: HTMLElement,
    data: String,
    ty: String,
    name: String,
    use_map: String,
    width: String,
    height: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLObjectElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLObjectElement<T> {
    pub fn data(&self) -> String {
        self.data.clone()
    }

    pub fn set_data(&mut self, value: String) {
        self.data = value;
    }
    pub fn ty(&self) -> String {
        self.ty.clone()
    }

    pub fn set_ty(&mut self, value: String) {
        self.ty = value;
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn use_map(&self) -> String {
        self.use_map.clone()
    }

    pub fn set_use_map(&mut self, value: String) {
        self.use_map = value;
    }
    pub fn width(&self) -> String {
        self.width.clone()
    }

    pub fn set_width(&mut self, value: String) {
        self.width = value;
    }
    pub fn height(&self) -> String {
        self.height.clone()
    }

    pub fn set_height(&mut self, value: String) {
        self.height = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLObjectElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLObjectElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<object>")?;
        write!(f, "</object>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLScriptElement<T> {
    deref_target: HTMLElement,
    src: String,
    ty: String,
    defer: bool,
    cross_origin: String,
    text: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLScriptElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLScriptElement<T> {
    pub fn src(&self) -> String {
        self.src.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.src = value;
    }
    pub fn ty(&self) -> String {
        self.ty.clone()
    }

    pub fn set_ty(&mut self, value: String) {
        self.ty = value;
    }
    pub fn defer(&self) -> bool {
        self.defer.clone()
    }

    pub fn set_defer(&mut self, value: bool) {
        self.defer = value;
    }
    pub fn cross_origin(&self) -> String {
        self.cross_origin.clone()
    }

    pub fn set_cross_origin(&mut self, value: String) {
        self.cross_origin = value;
    }
    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, value: String) {
        self.text = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLScriptElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLScriptElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<script>")?;
        write!(f, "</script>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLLIElement<T> {
    deref_target: HTMLElement,
    value: u64,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLLIElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLLIElement<T> {
    pub fn value(&self) -> u64 {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: u64) {
        self.value = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLLIElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLLIElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<li>")?;
        write!(f, "</li>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableSectionElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLTableSectionElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLTableSectionElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLTableSectionElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLTableSectionElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<tbody>")?;
        write!(f, "</tbody>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProcessingInstruction {
    deref_target: CharacterData,
}

impl ::std::ops::Deref for ProcessingInstruction {
    type Target = CharacterData;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ProcessingInstruction {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMarqueeElement<T> {
    deref_target: HTMLElement,
    behavior: String,
    bg_color: String,
    direction: String,
    height: String,
    hspace: u64,
    scroll_amount: u64,
    scroll_delay: u64,
    true_speed: bool,
    vspace: u64,
    width: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLMarqueeElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLMarqueeElement<T> {
    pub fn behavior(&self) -> String {
        self.behavior.clone()
    }

    pub fn set_behavior(&mut self, value: String) {
        self.behavior = value;
    }
    pub fn bg_color(&self) -> String {
        self.bg_color.clone()
    }

    pub fn set_bg_color(&mut self, value: String) {
        self.bg_color = value;
    }
    pub fn direction(&self) -> String {
        self.direction.clone()
    }

    pub fn set_direction(&mut self, value: String) {
        self.direction = value;
    }
    pub fn height(&self) -> String {
        self.height.clone()
    }

    pub fn set_height(&mut self, value: String) {
        self.height = value;
    }
    pub fn hspace(&self) -> u64 {
        self.hspace.clone()
    }

    pub fn set_hspace(&mut self, value: u64) {
        self.hspace = value;
    }
    pub fn scroll_amount(&self) -> u64 {
        self.scroll_amount.clone()
    }

    pub fn set_scroll_amount(&mut self, value: u64) {
        self.scroll_amount = value;
    }
    pub fn scroll_delay(&self) -> u64 {
        self.scroll_delay.clone()
    }

    pub fn set_scroll_delay(&mut self, value: u64) {
        self.scroll_delay = value;
    }
    pub fn true_speed(&self) -> bool {
        self.true_speed.clone()
    }

    pub fn set_true_speed(&mut self, value: bool) {
        self.true_speed = value;
    }
    pub fn vspace(&self) -> u64 {
        self.vspace.clone()
    }

    pub fn set_vspace(&mut self, value: u64) {
        self.vspace = value;
    }
    pub fn width(&self) -> String {
        self.width.clone()
    }

    pub fn set_width(&mut self, value: String) {
        self.width = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLMarqueeElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLMarqueeElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<marquee>")?;
        write!(f, "</marquee>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLCanvasElement<T> {
    deref_target: HTMLElement,
    width: u64,
    height: u64,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLCanvasElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLCanvasElement<T> {
    pub fn width(&self) -> u64 {
        self.width.clone()
    }

    pub fn set_width(&mut self, value: u64) {
        self.width = value;
    }
    pub fn height(&self) -> u64 {
        self.height.clone()
    }

    pub fn set_height(&mut self, value: u64) {
        self.height = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLCanvasElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLCanvasElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<canvas>")?;
        write!(f, "</canvas>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLPreElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLPreElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLPreElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLPreElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLPreElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<pre>")?;
        write!(f, "</pre>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMeterElement<T> {
    deref_target: HTMLElement,
    value: f64,
    min: f64,
    max: f64,
    low: f64,
    high: f64,
    optimum: f64,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLMeterElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLMeterElement<T> {
    pub fn value(&self) -> f64 {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
    pub fn min(&self) -> f64 {
        self.min.clone()
    }

    pub fn set_min(&mut self, value: f64) {
        self.min = value;
    }
    pub fn max(&self) -> f64 {
        self.max.clone()
    }

    pub fn set_max(&mut self, value: f64) {
        self.max = value;
    }
    pub fn low(&self) -> f64 {
        self.low.clone()
    }

    pub fn set_low(&mut self, value: f64) {
        self.low = value;
    }
    pub fn high(&self) -> f64 {
        self.high.clone()
    }

    pub fn set_high(&mut self, value: f64) {
        self.high = value;
    }
    pub fn optimum(&self) -> f64 {
        self.optimum.clone()
    }

    pub fn set_optimum(&mut self, value: f64) {
        self.optimum = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLMeterElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLMeterElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<meter>")?;
        write!(f, "</meter>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLLabelElement<T> {
    deref_target: HTMLElement,
    html_for: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLLabelElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLLabelElement<T> {
    pub fn html_for(&self) -> String {
        self.html_for.clone()
    }

    pub fn set_html_for(&mut self, value: String) {
        self.html_for = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLLabelElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLLabelElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<label>")?;
        write!(f, "</label>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct SVGTitleElement {
    deref_target: SVGElement,
}

impl ::std::ops::Deref for SVGTitleElement {
    type Target = SVGElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl SVGTitleElement {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDialogElement<T> {
    deref_target: HTMLElement,
    open: bool,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLDialogElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLDialogElement<T> {
    pub fn open(&self) -> bool {
        self.open.clone()
    }

    pub fn set_open(&mut self, value: bool) {
        self.open = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLDialogElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLDialogElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<dialog>")?;
        write!(f, "</dialog>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTrackElement<T> {
    deref_target: HTMLElement,
    kind: String,
    src: String,
    srclang: String,
    label: String,
    default: bool,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLTrackElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLTrackElement<T> {
    pub fn kind(&self) -> String {
        self.kind.clone()
    }

    pub fn set_kind(&mut self, value: String) {
        self.kind = value;
    }
    pub fn src(&self) -> String {
        self.src.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.src = value;
    }
    pub fn srclang(&self) -> String {
        self.srclang.clone()
    }

    pub fn set_srclang(&mut self, value: String) {
        self.srclang = value;
    }
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn set_label(&mut self, value: String) {
        self.label = value;
    }
    pub fn default(&self) -> bool {
        self.default.clone()
    }

    pub fn set_default(&mut self, value: bool) {
        self.default = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLTrackElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLTrackElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<track>")?;
        write!(f, "</track>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLIFrameElement<T> {
    deref_target: HTMLElement,
    src: String,
    srcdoc: String,
    name: String,
    allow_fullscreen: bool,
    width: String,
    height: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLIFrameElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLIFrameElement<T> {
    pub fn src(&self) -> String {
        self.src.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.src = value;
    }
    pub fn srcdoc(&self) -> String {
        self.srcdoc.clone()
    }

    pub fn set_srcdoc(&mut self, value: String) {
        self.srcdoc = value;
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn allow_fullscreen(&self) -> bool {
        self.allow_fullscreen.clone()
    }

    pub fn set_allow_fullscreen(&mut self, value: bool) {
        self.allow_fullscreen = value;
    }
    pub fn width(&self) -> String {
        self.width.clone()
    }

    pub fn set_width(&mut self, value: String) {
        self.width = value;
    }
    pub fn height(&self) -> String {
        self.height.clone()
    }

    pub fn set_height(&mut self, value: String) {
        self.height = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLIFrameElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLIFrameElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<iframe>")?;
        write!(f, "</iframe>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableRowElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLTableRowElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLTableRowElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLTableRowElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLTableRowElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<tr>")?;
        write!(f, "</tr>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLOptionsCollection {
    deref_target: HTMLCollection,
    length: u64,
    selected_index: u64,
}

impl ::std::ops::Deref for HTMLOptionsCollection {
    type Target = HTMLCollection;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLOptionsCollection {
    pub fn length(&self) -> u64 {
        self.length.clone()
    }

    pub fn set_length(&mut self, value: u64) {
        self.length = value;
    }
    pub fn selected_index(&self) -> u64 {
        self.selected_index.clone()
    }

    pub fn set_selected_index(&mut self, value: u64) {
        self.selected_index = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTimeElement<T> {
    deref_target: HTMLElement,
    date_time: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLTimeElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLTimeElement<T> {
    pub fn date_time(&self) -> String {
        self.date_time.clone()
    }

    pub fn set_date_time(&mut self, value: String) {
        self.date_time = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLTimeElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLTimeElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<time>")?;
        write!(f, "</time>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct DocumentType {
    deref_target: Node,
}

impl ::std::ops::Deref for DocumentType {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl DocumentType {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Comment {
    deref_target: CharacterData,
}

impl ::std::ops::Deref for Comment {
    type Target = CharacterData;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl Comment {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMetaElement<T> {
    deref_target: HTMLElement,
    name: String,
    http_equiv: String,
    content: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLMetaElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLMetaElement<T> {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn http_equiv(&self) -> String {
        self.http_equiv.clone()
    }

    pub fn set_http_equiv(&mut self, value: String) {
        self.http_equiv = value;
    }
    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn set_content(&mut self, value: String) {
        self.content = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLMetaElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLMetaElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<meta>")?;
        write!(f, "</meta>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ShadowRoot {
    deref_target: DocumentFragment,
    inner_html: String,
}

impl ::std::ops::Deref for ShadowRoot {
    type Target = DocumentFragment;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ShadowRoot {
    pub fn inner_html(&self) -> String {
        self.inner_html.clone()
    }

    pub fn set_inner_html(&mut self, value: String) {
        self.inner_html = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLProgressElement<T> {
    deref_target: HTMLElement,
    value: f64,
    max: f64,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLProgressElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLProgressElement<T> {
    pub fn value(&self) -> f64 {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
    pub fn max(&self) -> f64 {
        self.max.clone()
    }

    pub fn set_max(&mut self, value: f64) {
        self.max = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLProgressElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLProgressElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<progress>")?;
        write!(f, "</progress>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct SVGElement {
    deref_target: Element,
}

impl ::std::ops::Deref for SVGElement {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl SVGElement {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDListElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLDListElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLDListElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLDListElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLDListElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<dlist>")?;
        write!(f, "</dlist>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLCollection {}

impl HTMLCollection {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLModElement<T> {
    deref_target: HTMLElement,
    cite: String,
    date_time: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLModElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLModElement<T> {
    pub fn cite(&self) -> String {
        self.cite.clone()
    }

    pub fn set_cite(&mut self, value: String) {
        self.cite = value;
    }
    pub fn date_time(&self) -> String {
        self.date_time.clone()
    }

    pub fn set_date_time(&mut self, value: String) {
        self.date_time = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLModElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLModElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<mod>")?;
        write!(f, "</mod>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLVideoElement<T> {
    deref_target: HTMLMediaElement<T>,
    width: u64,
    height: u64,
    poster: String,
    plays_inline: bool,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLVideoElement<T> {
    type Target = HTMLMediaElement<T>;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLVideoElement<T> {
    pub fn width(&self) -> u64 {
        self.width.clone()
    }

    pub fn set_width(&mut self, value: u64) {
        self.width = value;
    }
    pub fn height(&self) -> u64 {
        self.height.clone()
    }

    pub fn set_height(&mut self, value: u64) {
        self.height = value;
    }
    pub fn poster(&self) -> String {
        self.poster.clone()
    }

    pub fn set_poster(&mut self, value: String) {
        self.poster = value;
    }
    pub fn plays_inline(&self) -> bool {
        self.plays_inline.clone()
    }

    pub fn set_plays_inline(&mut self, value: bool) {
        self.plays_inline = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLVideoElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLVideoElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<video>")?;
        write!(f, "</video>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLFieldSetElement<T> {
    deref_target: HTMLElement,
    disabled: bool,
    name: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLFieldSetElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLFieldSetElement<T> {
    pub fn disabled(&self) -> bool {
        self.disabled.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.disabled = value;
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLFieldSetElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLFieldSetElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<fieldset>")?;
        write!(f, "</fieldset>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLUnknownElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLUnknownElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLUnknownElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLUnknownElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLUnknownElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<unknown>")?;
        write!(f, "</unknown>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableCaptionElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLTableCaptionElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLTableCaptionElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLTableCaptionElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLTableCaptionElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<caption>")?;
        write!(f, "</caption>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct CDATASection {
    deref_target: Text,
}

impl ::std::ops::Deref for CDATASection {
    type Target = Text;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl CDATASection {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLOptGroupElement<T> {
    deref_target: HTMLElement,
    disabled: bool,
    label: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLOptGroupElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLOptGroupElement<T> {
    pub fn disabled(&self) -> bool {
        self.disabled.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.disabled = value;
    }
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn set_label(&mut self, value: String) {
        self.label = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLOptGroupElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLOptGroupElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<optgroup>")?;
        write!(f, "</optgroup>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLSourceElement<T> {
    deref_target: HTMLElement,
    src: String,
    ty: String,
    srcset: String,
    sizes: String,
    media: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLSourceElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLSourceElement<T> {
    pub fn src(&self) -> String {
        self.src.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.src = value;
    }
    pub fn ty(&self) -> String {
        self.ty.clone()
    }

    pub fn set_ty(&mut self, value: String) {
        self.ty = value;
    }
    pub fn srcset(&self) -> String {
        self.srcset.clone()
    }

    pub fn set_srcset(&mut self, value: String) {
        self.srcset = value;
    }
    pub fn sizes(&self) -> String {
        self.sizes.clone()
    }

    pub fn set_sizes(&mut self, value: String) {
        self.sizes = value;
    }
    pub fn media(&self) -> String {
        self.media.clone()
    }

    pub fn set_media(&mut self, value: String) {
        self.media = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLSourceElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLSourceElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<source>")?;
        write!(f, "</source>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMenuElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLMenuElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLMenuElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLMenuElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLMenuElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<menu>")?;
        write!(f, "</menu>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLLinkElement<T> {
    deref_target: HTMLElement,
    href: String,
    cross_origin: String,
    rel: String,
    media: String,
    hreflang: String,
    ty: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLLinkElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLLinkElement<T> {
    pub fn href(&self) -> String {
        self.href.clone()
    }

    pub fn set_href(&mut self, value: String) {
        self.href = value;
    }
    pub fn cross_origin(&self) -> String {
        self.cross_origin.clone()
    }

    pub fn set_cross_origin(&mut self, value: String) {
        self.cross_origin = value;
    }
    pub fn rel(&self) -> String {
        self.rel.clone()
    }

    pub fn set_rel(&mut self, value: String) {
        self.rel = value;
    }
    pub fn media(&self) -> String {
        self.media.clone()
    }

    pub fn set_media(&mut self, value: String) {
        self.media = value;
    }
    pub fn hreflang(&self) -> String {
        self.hreflang.clone()
    }

    pub fn set_hreflang(&mut self, value: String) {
        self.hreflang = value;
    }
    pub fn ty(&self) -> String {
        self.ty.clone()
    }

    pub fn set_ty(&mut self, value: String) {
        self.ty = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLLinkElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLLinkElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<link>")?;
        write!(f, "</link>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDivElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLDivElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLDivElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLDivElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLDivElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<div>")?;
        write!(f, "</div>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDetailsElement<T> {
    deref_target: HTMLElement,
    open: bool,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLDetailsElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLDetailsElement<T> {
    pub fn open(&self) -> bool {
        self.open.clone()
    }

    pub fn set_open(&mut self, value: bool) {
        self.open = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLDetailsElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLDetailsElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<details>")?;
        write!(f, "</details>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLHeadElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLHeadElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLHeadElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLHeadElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLHeadElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<head>")?;
        write!(f, "</head>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLFormElement<T> {
    deref_target: HTMLElement,
    accept_charset: String,
    action: String,
    enctype: String,
    method: String,
    name: String,
    no_validate: bool,
    target: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLFormElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLFormElement<T> {
    pub fn accept_charset(&self) -> String {
        self.accept_charset.clone()
    }

    pub fn set_accept_charset(&mut self, value: String) {
        self.accept_charset = value;
    }
    pub fn action(&self) -> String {
        self.action.clone()
    }

    pub fn set_action(&mut self, value: String) {
        self.action = value;
    }
    pub fn enctype(&self) -> String {
        self.enctype.clone()
    }

    pub fn set_enctype(&mut self, value: String) {
        self.enctype = value;
    }
    pub fn method(&self) -> String {
        self.method.clone()
    }

    pub fn set_method(&mut self, value: String) {
        self.method = value;
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn no_validate(&self) -> bool {
        self.no_validate.clone()
    }

    pub fn set_no_validate(&mut self, value: bool) {
        self.no_validate = value;
    }
    pub fn target(&self) -> String {
        self.target.clone()
    }

    pub fn set_target(&mut self, value: String) {
        self.target = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLFormElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLFormElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<form>")?;
        write!(f, "</form>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLSelectElement<T> {
    deref_target: HTMLElement,
    autofocus: bool,
    disabled: bool,
    multiple: bool,
    name: String,
    required: bool,
    size: u64,
    length: u64,
    selected_index: u64,
    value: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLSelectElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLSelectElement<T> {
    pub fn autofocus(&self) -> bool {
        self.autofocus.clone()
    }

    pub fn set_autofocus(&mut self, value: bool) {
        self.autofocus = value;
    }
    pub fn disabled(&self) -> bool {
        self.disabled.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.disabled = value;
    }
    pub fn multiple(&self) -> bool {
        self.multiple.clone()
    }

    pub fn set_multiple(&mut self, value: bool) {
        self.multiple = value;
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn required(&self) -> bool {
        self.required.clone()
    }

    pub fn set_required(&mut self, value: bool) {
        self.required = value;
    }
    pub fn size(&self) -> u64 {
        self.size.clone()
    }

    pub fn set_size(&mut self, value: u64) {
        self.size = value;
    }
    pub fn length(&self) -> u64 {
        self.length.clone()
    }

    pub fn set_length(&mut self, value: u64) {
        self.length = value;
    }
    pub fn selected_index(&self) -> u64 {
        self.selected_index.clone()
    }

    pub fn set_selected_index(&mut self, value: u64) {
        self.selected_index = value;
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLSelectElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLSelectElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<select>")?;
        write!(f, "</select>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct XMLDocument {
    deref_target: Document,
}

impl ::std::ops::Deref for XMLDocument {
    type Target = Document;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl XMLDocument {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLAudioElement<T> {
    deref_target: HTMLMediaElement<T>,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLAudioElement<T> {
    type Target = HTMLMediaElement<T>;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLAudioElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLAudioElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLAudioElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<audio>")?;
        write!(f, "</audio>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLPictureElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLPictureElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLPictureElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLPictureElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLPictureElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<picture>")?;
        write!(f, "</picture>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLOptionElement<T> {
    deref_target: HTMLElement,
    disabled: bool,
    label: String,
    default_selected: bool,
    selected: bool,
    value: String,
    text: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLOptionElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLOptionElement<T> {
    pub fn disabled(&self) -> bool {
        self.disabled.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.disabled = value;
    }
    pub fn label(&self) -> String {
        self.label.clone()
    }

    pub fn set_label(&mut self, value: String) {
        self.label = value;
    }
    pub fn default_selected(&self) -> bool {
        self.default_selected.clone()
    }

    pub fn set_default_selected(&mut self, value: bool) {
        self.default_selected = value;
    }
    pub fn selected(&self) -> bool {
        self.selected.clone()
    }

    pub fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, value: String) {
        self.text = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLOptionElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLOptionElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<option>")?;
        write!(f, "</option>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLQuoteElement<T> {
    deref_target: HTMLElement,
    cite: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLQuoteElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLQuoteElement<T> {
    pub fn cite(&self) -> String {
        self.cite.clone()
    }

    pub fn set_cite(&mut self, value: String) {
        self.cite = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLQuoteElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLQuoteElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<quote>")?;
        write!(f, "</quote>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Node {
    deref_target: EventTarget,
    node_value: String,
    text_content: String,
}

impl ::std::ops::Deref for Node {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl Node {
    pub fn node_value(&self) -> String {
        self.node_value.clone()
    }

    pub fn set_node_value(&mut self, value: String) {
        self.node_value = value;
    }
    pub fn text_content(&self) -> String {
        self.text_content.clone()
    }

    pub fn set_text_content(&mut self, value: String) {
        self.text_content = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMediaElement<T> {
    deref_target: HTMLElement,
    src: String,
    cross_origin: String,
    preload: String,
    current_time: f64,
    default_playback_rate: f64,
    playback_rate: f64,
    autoplay: bool,
    loop_: bool,
    controls: bool,
    volume: f64,
    muted: bool,
    default_muted: bool,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLMediaElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLMediaElement<T> {
    pub fn src(&self) -> String {
        self.src.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.src = value;
    }
    pub fn cross_origin(&self) -> String {
        self.cross_origin.clone()
    }

    pub fn set_cross_origin(&mut self, value: String) {
        self.cross_origin = value;
    }
    pub fn preload(&self) -> String {
        self.preload.clone()
    }

    pub fn set_preload(&mut self, value: String) {
        self.preload = value;
    }
    pub fn current_time(&self) -> f64 {
        self.current_time.clone()
    }

    pub fn set_current_time(&mut self, value: f64) {
        self.current_time = value;
    }
    pub fn default_playback_rate(&self) -> f64 {
        self.default_playback_rate.clone()
    }

    pub fn set_default_playback_rate(&mut self, value: f64) {
        self.default_playback_rate = value;
    }
    pub fn playback_rate(&self) -> f64 {
        self.playback_rate.clone()
    }

    pub fn set_playback_rate(&mut self, value: f64) {
        self.playback_rate = value;
    }
    pub fn autoplay(&self) -> bool {
        self.autoplay.clone()
    }

    pub fn set_autoplay(&mut self, value: bool) {
        self.autoplay = value;
    }
    pub fn loop_(&self) -> bool {
        self.loop_.clone()
    }

    pub fn set_loop_(&mut self, value: bool) {
        self.loop_ = value;
    }
    pub fn controls(&self) -> bool {
        self.controls.clone()
    }

    pub fn set_controls(&mut self, value: bool) {
        self.controls = value;
    }
    pub fn volume(&self) -> f64 {
        self.volume.clone()
    }

    pub fn set_volume(&mut self, value: f64) {
        self.volume = value;
    }
    pub fn muted(&self) -> bool {
        self.muted.clone()
    }

    pub fn set_muted(&mut self, value: bool) {
        self.muted = value;
    }
    pub fn default_muted(&self) -> bool {
        self.default_muted.clone()
    }

    pub fn set_default_muted(&mut self, value: bool) {
        self.default_muted = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLMediaElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLMediaElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<media>")?;
        write!(f, "</media>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableCellElement<T> {
    deref_target: HTMLElement,
    col_span: u64,
    row_span: u64,
    headers: String,
    scope: String,
    abbr: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLTableCellElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLTableCellElement<T> {
    pub fn col_span(&self) -> u64 {
        self.col_span.clone()
    }

    pub fn set_col_span(&mut self, value: u64) {
        self.col_span = value;
    }
    pub fn row_span(&self) -> u64 {
        self.row_span.clone()
    }

    pub fn set_row_span(&mut self, value: u64) {
        self.row_span = value;
    }
    pub fn headers(&self) -> String {
        self.headers.clone()
    }

    pub fn set_headers(&mut self, value: String) {
        self.headers = value;
    }
    pub fn scope(&self) -> String {
        self.scope.clone()
    }

    pub fn set_scope(&mut self, value: String) {
        self.scope = value;
    }
    pub fn abbr(&self) -> String {
        self.abbr.clone()
    }

    pub fn set_abbr(&mut self, value: String) {
        self.abbr = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLTableCellElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLTableCellElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<tablecell>")?;
        write!(f, "</tablecell>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableColElement<T> {
    deref_target: HTMLElement,
    span: u64,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLTableColElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLTableColElement<T> {
    pub fn span(&self) -> u64 {
        self.span.clone()
    }

    pub fn set_span(&mut self, value: u64) {
        self.span = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLTableColElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLTableColElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<col>")?;
        write!(f, "</col>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLBodyElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLBodyElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLBodyElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLBodyElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLBodyElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<body>")?;
        write!(f, "</body>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct NodeList {}

impl NodeList {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLParamElement<T> {
    deref_target: HTMLElement,
    name: String,
    value: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLParamElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLParamElement<T> {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLParamElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLParamElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<param>")?;
        write!(f, "</param>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLStyleElement<T> {
    deref_target: HTMLElement,
    media: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLStyleElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLStyleElement<T> {
    pub fn media(&self) -> String {
        self.media.clone()
    }

    pub fn set_media(&mut self, value: String) {
        self.media = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLStyleElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLStyleElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<style>")?;
        write!(f, "</style>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDataElement<T> {
    deref_target: HTMLElement,
    value: String,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLDataElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLDataElement<T> {
    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

impl<T: HtmlElement> HtmlElement for HTMLDataElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLDataElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<data>")?;
        write!(f, "</data>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLHRElement<T> {
    deref_target: HTMLElement,
}

impl<T: HtmlElement> ::std::ops::Deref for HTMLHRElement<T> {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl<T: HtmlElement> HTMLHRElement<T> {}

impl<T: HtmlElement> HtmlElement for HTMLHRElement<T> {}

impl<T: HtmlElement> ::std::fmt::Display for HTMLHRElement<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<hr>")?;
        write!(f, "</hr>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct FileList {}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct EventTarget {}
