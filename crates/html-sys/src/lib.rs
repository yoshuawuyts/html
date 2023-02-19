#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLEmbedElement {
    deref_target: HTMLElement,
    src: String,
    type_: String,
    width: String,
    height: String,
}

impl ::std::ops::Deref for HTMLEmbedElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLEmbedElement {
    pub fn src(&self) -> String {
        self.ty.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.ty = value;
    }

    pub fn type_(&self) -> String {
        self.ty.clone()
    }

    pub fn set_type_(&mut self, value: String) {
        self.ty = value;
    }

    pub fn width(&self) -> String {
        self.ty.clone()
    }

    pub fn set_width(&mut self, value: String) {
        self.ty = value;
    }

    pub fn height(&self) -> String {
        self.ty.clone()
    }

    pub fn set_height(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct Document {
    deref_target: Node,
}

impl ::std::ops::Deref for Document {
    type Target = Node;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl Document {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct DocumentType {
    deref_target: Node,
}

impl ::std::ops::Deref for DocumentType {
    type Target = Node;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl DocumentType {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLMeterElement {
    deref_target: HTMLElement,
    value: f64,
    min: f64,
    max: f64,
    low: f64,
    high: f64,
    optimum: f64,
}

impl ::std::ops::Deref for HTMLMeterElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLMeterElement {
    pub fn value(&self) -> f64 {
        self.ty.clone()
    }

    pub fn set_value(&mut self, value: f64) {
        self.ty = value;
    }

    pub fn min(&self) -> f64 {
        self.ty.clone()
    }

    pub fn set_min(&mut self, value: f64) {
        self.ty = value;
    }

    pub fn max(&self) -> f64 {
        self.ty.clone()
    }

    pub fn set_max(&mut self, value: f64) {
        self.ty = value;
    }

    pub fn low(&self) -> f64 {
        self.ty.clone()
    }

    pub fn set_low(&mut self, value: f64) {
        self.ty = value;
    }

    pub fn high(&self) -> f64 {
        self.ty.clone()
    }

    pub fn set_high(&mut self, value: f64) {
        self.ty = value;
    }

    pub fn optimum(&self) -> f64 {
        self.ty.clone()
    }

    pub fn set_optimum(&mut self, value: f64) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLTableColElement {
    deref_target: HTMLElement,
    span: u64,
}

impl ::std::ops::Deref for HTMLTableColElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLTableColElement {
    pub fn span(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_span(&mut self, value: u64) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct DocumentFragment {
    deref_target: Node,
}

impl ::std::ops::Deref for DocumentFragment {
    type Target = Node;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl DocumentFragment {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLFormElement {
    deref_target: HTMLElement,
    acceptCharset: String,
    action: String,
    enctype: String,
    method: String,
    name: String,
    noValidate: bool,
    target: String,
}

impl ::std::ops::Deref for HTMLFormElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLFormElement {
    pub fn acceptCharset(&self) -> String {
        self.ty.clone()
    }

    pub fn set_acceptCharset(&mut self, value: String) {
        self.ty = value;
    }

    pub fn action(&self) -> String {
        self.ty.clone()
    }

    pub fn set_action(&mut self, value: String) {
        self.ty = value;
    }

    pub fn enctype(&self) -> String {
        self.ty.clone()
    }

    pub fn set_enctype(&mut self, value: String) {
        self.ty = value;
    }

    pub fn method(&self) -> String {
        self.ty.clone()
    }

    pub fn set_method(&mut self, value: String) {
        self.ty = value;
    }

    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }

    pub fn noValidate(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_noValidate(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn target(&self) -> String {
        self.ty.clone()
    }

    pub fn set_target(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct CDATASection {
    deref_target: Text,
}

impl ::std::ops::Deref for CDATASection {
    type Target = Text;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl CDATASection {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLPictureElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLPictureElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLPictureElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLMenuElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLMenuElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLMenuElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLFrameSetElement {
    deref_target: HTMLElement,
    cols: String,
    rows: String,
}

impl ::std::ops::Deref for HTMLFrameSetElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLFrameSetElement {
    pub fn cols(&self) -> String {
        self.ty.clone()
    }

    pub fn set_cols(&mut self, value: String) {
        self.ty = value;
    }

    pub fn rows(&self) -> String {
        self.ty.clone()
    }

    pub fn set_rows(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLParagraphElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLParagraphElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLParagraphElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLLegendElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLLegendElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLLegendElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct NodeList {

}



impl NodeList {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLHeadElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLHeadElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLHeadElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLFrameElement {
    deref_target: HTMLElement,
    name: String,
    scrolling: String,
    src: String,
    frameBorder: String,
    longDesc: String,
    noResize: bool,
    marginHeight: String,
    marginWidth: String,
}

impl ::std::ops::Deref for HTMLFrameElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLFrameElement {
    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }

    pub fn scrolling(&self) -> String {
        self.ty.clone()
    }

    pub fn set_scrolling(&mut self, value: String) {
        self.ty = value;
    }

    pub fn src(&self) -> String {
        self.ty.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.ty = value;
    }

    pub fn frameBorder(&self) -> String {
        self.ty.clone()
    }

    pub fn set_frameBorder(&mut self, value: String) {
        self.ty = value;
    }

    pub fn longDesc(&self) -> String {
        self.ty.clone()
    }

    pub fn set_longDesc(&mut self, value: String) {
        self.ty = value;
    }

    pub fn noResize(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_noResize(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn marginHeight(&self) -> String {
        self.ty.clone()
    }

    pub fn set_marginHeight(&mut self, value: String) {
        self.ty = value;
    }

    pub fn marginWidth(&self) -> String {
        self.ty.clone()
    }

    pub fn set_marginWidth(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLHeadingElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLHeadingElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLHeadingElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLSpanElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLSpanElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLSpanElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLPreElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLPreElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLPreElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLQuoteElement {
    deref_target: HTMLElement,
    cite: String,
}

impl ::std::ops::Deref for HTMLQuoteElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLQuoteElement {
    pub fn cite(&self) -> String {
        self.ty.clone()
    }

    pub fn set_cite(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLHRElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLHRElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLHRElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct SVGTitleElement {
    deref_target: SVGElement,
}

impl ::std::ops::Deref for SVGTitleElement {
    type Target = SVGElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl SVGTitleElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLOListElement {
    deref_target: HTMLElement,
    reversed: bool,
    start: u64,
    type_: String,
}

impl ::std::ops::Deref for HTMLOListElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLOListElement {
    pub fn reversed(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_reversed(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn start(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_start(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn type_(&self) -> String {
        self.ty.clone()
    }

    pub fn set_type_(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct ProcessingInstruction {
    deref_target: CharacterData,
}

impl ::std::ops::Deref for ProcessingInstruction {
    type Target = CharacterData;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl ProcessingInstruction {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLStyleElement {
    deref_target: HTMLElement,
    media: String,
}

impl ::std::ops::Deref for HTMLStyleElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLStyleElement {
    pub fn media(&self) -> String {
        self.ty.clone()
    }

    pub fn set_media(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLCanvasElement {
    deref_target: HTMLElement,
    width: u64,
    height: u64,
}

impl ::std::ops::Deref for HTMLCanvasElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLCanvasElement {
    pub fn width(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_width(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn height(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_height(&mut self, value: u64) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLScriptElement {
    deref_target: HTMLElement,
    src: String,
    type_: String,
    defer: bool,
    crossOrigin: String,
    text: String,
}

impl ::std::ops::Deref for HTMLScriptElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLScriptElement {
    pub fn src(&self) -> String {
        self.ty.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.ty = value;
    }

    pub fn type_(&self) -> String {
        self.ty.clone()
    }

    pub fn set_type_(&mut self, value: String) {
        self.ty = value;
    }

    pub fn defer(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_defer(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn crossOrigin(&self) -> String {
        self.ty.clone()
    }

    pub fn set_crossOrigin(&mut self, value: String) {
        self.ty = value;
    }

    pub fn text(&self) -> String {
        self.ty.clone()
    }

    pub fn set_text(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLDataListElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLDataListElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLDataListElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLBodyElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLBodyElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLBodyElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLSlotElement {
    deref_target: HTMLElement,
    name: String,
}

impl ::std::ops::Deref for HTMLSlotElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLSlotElement {
    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct SVGSVGElement {
    deref_target: SVGGraphicsElement,
}

impl ::std::ops::Deref for SVGSVGElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl SVGSVGElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLProgressElement {
    deref_target: HTMLElement,
    value: f64,
    max: f64,
}

impl ::std::ops::Deref for HTMLProgressElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLProgressElement {
    pub fn value(&self) -> f64 {
        self.ty.clone()
    }

    pub fn set_value(&mut self, value: f64) {
        self.ty = value;
    }

    pub fn max(&self) -> f64 {
        self.ty.clone()
    }

    pub fn set_max(&mut self, value: f64) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLOutputElement {
    deref_target: HTMLElement,
    name: String,
    defaultValue: String,
    value: String,
}

impl ::std::ops::Deref for HTMLOutputElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLOutputElement {
    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }

    pub fn defaultValue(&self) -> String {
        self.ty.clone()
    }

    pub fn set_defaultValue(&mut self, value: String) {
        self.ty = value;
    }

    pub fn value(&self) -> String {
        self.ty.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct DOMTokenList {

    value: String,
}



impl DOMTokenList {
    pub fn value(&self) -> String {
        self.ty.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct SVGElement {
    deref_target: Element,
}

impl ::std::ops::Deref for SVGElement {
    type Target = Element;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl SVGElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLUnknownElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLUnknownElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLUnknownElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLObjectElement {
    deref_target: HTMLElement,
    data: String,
    type_: String,
    name: String,
    useMap: String,
    width: String,
    height: String,
}

impl ::std::ops::Deref for HTMLObjectElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLObjectElement {
    pub fn data(&self) -> String {
        self.ty.clone()
    }

    pub fn set_data(&mut self, value: String) {
        self.ty = value;
    }

    pub fn type_(&self) -> String {
        self.ty.clone()
    }

    pub fn set_type_(&mut self, value: String) {
        self.ty = value;
    }

    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }

    pub fn useMap(&self) -> String {
        self.ty.clone()
    }

    pub fn set_useMap(&mut self, value: String) {
        self.ty = value;
    }

    pub fn width(&self) -> String {
        self.ty.clone()
    }

    pub fn set_width(&mut self, value: String) {
        self.ty = value;
    }

    pub fn height(&self) -> String {
        self.ty.clone()
    }

    pub fn set_height(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLLabelElement {
    deref_target: HTMLElement,
    htmlFor: String,
}

impl ::std::ops::Deref for HTMLLabelElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLLabelElement {
    pub fn htmlFor(&self) -> String {
        self.ty.clone()
    }

    pub fn set_htmlFor(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLElement {
    deref_target: Element,
    title: String,
    lang: String,
    translate: bool,
    dir: String,
    hidden: bool,
    accessKey: String,
    draggable: bool,
}

impl ::std::ops::Deref for HTMLElement {
    type Target = Element;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLElement {
    pub fn title(&self) -> String {
        self.ty.clone()
    }

    pub fn set_title(&mut self, value: String) {
        self.ty = value;
    }

    pub fn lang(&self) -> String {
        self.ty.clone()
    }

    pub fn set_lang(&mut self, value: String) {
        self.ty = value;
    }

    pub fn translate(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_translate(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn dir(&self) -> String {
        self.ty.clone()
    }

    pub fn set_dir(&mut self, value: String) {
        self.ty = value;
    }

    pub fn hidden(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_hidden(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn accessKey(&self) -> String {
        self.ty.clone()
    }

    pub fn set_accessKey(&mut self, value: String) {
        self.ty = value;
    }

    pub fn draggable(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_draggable(&mut self, value: bool) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLTitleElement {
    deref_target: HTMLElement,
    text: String,
}

impl ::std::ops::Deref for HTMLTitleElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLTitleElement {
    pub fn text(&self) -> String {
        self.ty.clone()
    }

    pub fn set_text(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLUListElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLUListElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLUListElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLLIElement {
    deref_target: HTMLElement,
    value: u64,
}

impl ::std::ops::Deref for HTMLLIElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLLIElement {
    pub fn value(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_value(&mut self, value: u64) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLCollection {

}



impl HTMLCollection {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLDivElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLDivElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLDivElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLSelectElement {
    deref_target: HTMLElement,
    autofocus: bool,
    disabled: bool,
    multiple: bool,
    name: String,
    required: bool,
    size: u64,
    length: u64,
    selectedIndex: u64,
    value: String,
}

impl ::std::ops::Deref for HTMLSelectElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLSelectElement {
    pub fn autofocus(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_autofocus(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn disabled(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn multiple(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_multiple(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }

    pub fn required(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_required(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn size(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_size(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn length(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_length(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn selectedIndex(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_selectedIndex(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn value(&self) -> String {
        self.ty.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLLinkElement {
    deref_target: HTMLElement,
    href: String,
    crossOrigin: String,
    rel: String,
    media: String,
    hreflang: String,
    type_: String,
}

impl ::std::ops::Deref for HTMLLinkElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLLinkElement {
    pub fn href(&self) -> String {
        self.ty.clone()
    }

    pub fn set_href(&mut self, value: String) {
        self.ty = value;
    }

    pub fn crossOrigin(&self) -> String {
        self.ty.clone()
    }

    pub fn set_crossOrigin(&mut self, value: String) {
        self.ty = value;
    }

    pub fn rel(&self) -> String {
        self.ty.clone()
    }

    pub fn set_rel(&mut self, value: String) {
        self.ty = value;
    }

    pub fn media(&self) -> String {
        self.ty.clone()
    }

    pub fn set_media(&mut self, value: String) {
        self.ty = value;
    }

    pub fn hreflang(&self) -> String {
        self.ty.clone()
    }

    pub fn set_hreflang(&mut self, value: String) {
        self.ty = value;
    }

    pub fn type_(&self) -> String {
        self.ty.clone()
    }

    pub fn set_type_(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLMediaElement {
    deref_target: HTMLElement,
    src: String,
    crossOrigin: String,
    preload: String,
    currentTime: f64,
    defaultPlaybackRate: f64,
    playbackRate: f64,
    autoplay: bool,
    loop_: bool,
    controls: bool,
    volume: f64,
    muted: bool,
    defaultMuted: bool,
}

impl ::std::ops::Deref for HTMLMediaElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLMediaElement {
    pub fn src(&self) -> String {
        self.ty.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.ty = value;
    }

    pub fn crossOrigin(&self) -> String {
        self.ty.clone()
    }

    pub fn set_crossOrigin(&mut self, value: String) {
        self.ty = value;
    }

    pub fn preload(&self) -> String {
        self.ty.clone()
    }

    pub fn set_preload(&mut self, value: String) {
        self.ty = value;
    }

    pub fn currentTime(&self) -> f64 {
        self.ty.clone()
    }

    pub fn set_currentTime(&mut self, value: f64) {
        self.ty = value;
    }

    pub fn defaultPlaybackRate(&self) -> f64 {
        self.ty.clone()
    }

    pub fn set_defaultPlaybackRate(&mut self, value: f64) {
        self.ty = value;
    }

    pub fn playbackRate(&self) -> f64 {
        self.ty.clone()
    }

    pub fn set_playbackRate(&mut self, value: f64) {
        self.ty = value;
    }

    pub fn autoplay(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_autoplay(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn loop_(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_loop_(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn controls(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_controls(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn volume(&self) -> f64 {
        self.ty.clone()
    }

    pub fn set_volume(&mut self, value: f64) {
        self.ty = value;
    }

    pub fn muted(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_muted(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn defaultMuted(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_defaultMuted(&mut self, value: bool) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct Comment {
    deref_target: CharacterData,
}

impl ::std::ops::Deref for Comment {
    type Target = CharacterData;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl Comment {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLOptionElement {
    deref_target: HTMLElement,
    disabled: bool,
    label: String,
    defaultSelected: bool,
    selected: bool,
    value: String,
    text: String,
}

impl ::std::ops::Deref for HTMLOptionElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLOptionElement {
    pub fn disabled(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn label(&self) -> String {
        self.ty.clone()
    }

    pub fn set_label(&mut self, value: String) {
        self.ty = value;
    }

    pub fn defaultSelected(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_defaultSelected(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn selected(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_selected(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn value(&self) -> String {
        self.ty.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.ty = value;
    }

    pub fn text(&self) -> String {
        self.ty.clone()
    }

    pub fn set_text(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct CharacterData {
    deref_target: Node,
    data: String,
}

impl ::std::ops::Deref for CharacterData {
    type Target = Node;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl CharacterData {
    pub fn data(&self) -> String {
        self.ty.clone()
    }

    pub fn set_data(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLTemplateElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLTemplateElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLTemplateElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLHtmlElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLHtmlElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLHtmlElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLTrackElement {
    deref_target: HTMLElement,
    kind: String,
    src: String,
    srclang: String,
    label: String,
    default: bool,
}

impl ::std::ops::Deref for HTMLTrackElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLTrackElement {
    pub fn kind(&self) -> String {
        self.ty.clone()
    }

    pub fn set_kind(&mut self, value: String) {
        self.ty = value;
    }

    pub fn src(&self) -> String {
        self.ty.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.ty = value;
    }

    pub fn srclang(&self) -> String {
        self.ty.clone()
    }

    pub fn set_srclang(&mut self, value: String) {
        self.ty = value;
    }

    pub fn label(&self) -> String {
        self.ty.clone()
    }

    pub fn set_label(&mut self, value: String) {
        self.ty = value;
    }

    pub fn default(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_default(&mut self, value: bool) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLDirectoryElement {
    deref_target: HTMLElement,
    compact: bool,
}

impl ::std::ops::Deref for HTMLDirectoryElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLDirectoryElement {
    pub fn compact(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_compact(&mut self, value: bool) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct DOMImplementation {

}



impl DOMImplementation {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLTableRowElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLTableRowElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLTableRowElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLTextAreaElement {
    deref_target: HTMLElement,
    autocomplete: String,
    autofocus: bool,
    cols: u64,
    dirName: String,
    disabled: bool,
    inputMode: String,
    maxLength: u64,
    minLength: u64,
    name: String,
    placeholder: String,
    readOnly: bool,
    required: bool,
    rows: u64,
    wrap: String,
    defaultValue: String,
    value: String,
    selectionStart: u64,
    selectionEnd: u64,
    selectionDirection: String,
}

impl ::std::ops::Deref for HTMLTextAreaElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLTextAreaElement {
    pub fn autocomplete(&self) -> String {
        self.ty.clone()
    }

    pub fn set_autocomplete(&mut self, value: String) {
        self.ty = value;
    }

    pub fn autofocus(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_autofocus(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn cols(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_cols(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn dirName(&self) -> String {
        self.ty.clone()
    }

    pub fn set_dirName(&mut self, value: String) {
        self.ty = value;
    }

    pub fn disabled(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn inputMode(&self) -> String {
        self.ty.clone()
    }

    pub fn set_inputMode(&mut self, value: String) {
        self.ty = value;
    }

    pub fn maxLength(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_maxLength(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn minLength(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_minLength(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }

    pub fn placeholder(&self) -> String {
        self.ty.clone()
    }

    pub fn set_placeholder(&mut self, value: String) {
        self.ty = value;
    }

    pub fn readOnly(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_readOnly(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn required(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_required(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn rows(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_rows(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn wrap(&self) -> String {
        self.ty.clone()
    }

    pub fn set_wrap(&mut self, value: String) {
        self.ty = value;
    }

    pub fn defaultValue(&self) -> String {
        self.ty.clone()
    }

    pub fn set_defaultValue(&mut self, value: String) {
        self.ty = value;
    }

    pub fn value(&self) -> String {
        self.ty.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.ty = value;
    }

    pub fn selectionStart(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_selectionStart(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn selectionEnd(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_selectionEnd(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn selectionDirection(&self) -> String {
        self.ty.clone()
    }

    pub fn set_selectionDirection(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLSourceElement {
    deref_target: HTMLElement,
    src: String,
    type_: String,
    srcset: String,
    sizes: String,
    media: String,
}

impl ::std::ops::Deref for HTMLSourceElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLSourceElement {
    pub fn src(&self) -> String {
        self.ty.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.ty = value;
    }

    pub fn type_(&self) -> String {
        self.ty.clone()
    }

    pub fn set_type_(&mut self, value: String) {
        self.ty = value;
    }

    pub fn srcset(&self) -> String {
        self.ty.clone()
    }

    pub fn set_srcset(&mut self, value: String) {
        self.ty = value;
    }

    pub fn sizes(&self) -> String {
        self.ty.clone()
    }

    pub fn set_sizes(&mut self, value: String) {
        self.ty = value;
    }

    pub fn media(&self) -> String {
        self.ty.clone()
    }

    pub fn set_media(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct Node {
    deref_target: EventTarget,
    nodeValue: String,
    textContent: String,
}

impl ::std::ops::Deref for Node {
    type Target = EventTarget;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl Node {
    pub fn nodeValue(&self) -> String {
        self.ty.clone()
    }

    pub fn set_nodeValue(&mut self, value: String) {
        self.ty = value;
    }

    pub fn textContent(&self) -> String {
        self.ty.clone()
    }

    pub fn set_textContent(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLMetaElement {
    deref_target: HTMLElement,
    name: String,
    httpEquiv: String,
    content: String,
}

impl ::std::ops::Deref for HTMLMetaElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLMetaElement {
    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }

    pub fn httpEquiv(&self) -> String {
        self.ty.clone()
    }

    pub fn set_httpEquiv(&mut self, value: String) {
        self.ty = value;
    }

    pub fn content(&self) -> String {
        self.ty.clone()
    }

    pub fn set_content(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLBRElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLBRElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLBRElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLMarqueeElement {
    deref_target: HTMLElement,
    behavior: String,
    bgColor: String,
    direction: String,
    height: String,
    hspace: u64,
    scrollAmount: u64,
    scrollDelay: u64,
    trueSpeed: bool,
    vspace: u64,
    width: String,
}

impl ::std::ops::Deref for HTMLMarqueeElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLMarqueeElement {
    pub fn behavior(&self) -> String {
        self.ty.clone()
    }

    pub fn set_behavior(&mut self, value: String) {
        self.ty = value;
    }

    pub fn bgColor(&self) -> String {
        self.ty.clone()
    }

    pub fn set_bgColor(&mut self, value: String) {
        self.ty = value;
    }

    pub fn direction(&self) -> String {
        self.ty.clone()
    }

    pub fn set_direction(&mut self, value: String) {
        self.ty = value;
    }

    pub fn height(&self) -> String {
        self.ty.clone()
    }

    pub fn set_height(&mut self, value: String) {
        self.ty = value;
    }

    pub fn hspace(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_hspace(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn scrollAmount(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_scrollAmount(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn scrollDelay(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_scrollDelay(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn trueSpeed(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_trueSpeed(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn vspace(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_vspace(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn width(&self) -> String {
        self.ty.clone()
    }

    pub fn set_width(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLImageElement {
    deref_target: HTMLElement,
    alt: String,
    src: String,
    srcset: String,
    sizes: String,
    crossOrigin: String,
    useMap: String,
    isMap: bool,
    width: u64,
    height: u64,
}

impl ::std::ops::Deref for HTMLImageElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLImageElement {
    pub fn alt(&self) -> String {
        self.ty.clone()
    }

    pub fn set_alt(&mut self, value: String) {
        self.ty = value;
    }

    pub fn src(&self) -> String {
        self.ty.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.ty = value;
    }

    pub fn srcset(&self) -> String {
        self.ty.clone()
    }

    pub fn set_srcset(&mut self, value: String) {
        self.ty = value;
    }

    pub fn sizes(&self) -> String {
        self.ty.clone()
    }

    pub fn set_sizes(&mut self, value: String) {
        self.ty = value;
    }

    pub fn crossOrigin(&self) -> String {
        self.ty.clone()
    }

    pub fn set_crossOrigin(&mut self, value: String) {
        self.ty = value;
    }

    pub fn useMap(&self) -> String {
        self.ty.clone()
    }

    pub fn set_useMap(&mut self, value: String) {
        self.ty = value;
    }

    pub fn isMap(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_isMap(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn width(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_width(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn height(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_height(&mut self, value: u64) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLTimeElement {
    deref_target: HTMLElement,
    dateTime: String,
}

impl ::std::ops::Deref for HTMLTimeElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLTimeElement {
    pub fn dateTime(&self) -> String {
        self.ty.clone()
    }

    pub fn set_dateTime(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLAnchorElement {
    deref_target: HTMLElement,
    target: String,
    download: String,
    rel: String,
    hreflang: String,
    type_: String,
    text: String,
}

impl ::std::ops::Deref for HTMLAnchorElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLAnchorElement {
    pub fn target(&self) -> String {
        self.ty.clone()
    }

    pub fn set_target(&mut self, value: String) {
        self.ty = value;
    }

    pub fn download(&self) -> String {
        self.ty.clone()
    }

    pub fn set_download(&mut self, value: String) {
        self.ty = value;
    }

    pub fn rel(&self) -> String {
        self.ty.clone()
    }

    pub fn set_rel(&mut self, value: String) {
        self.ty = value;
    }

    pub fn hreflang(&self) -> String {
        self.ty.clone()
    }

    pub fn set_hreflang(&mut self, value: String) {
        self.ty = value;
    }

    pub fn type_(&self) -> String {
        self.ty.clone()
    }

    pub fn set_type_(&mut self, value: String) {
        self.ty = value;
    }

    pub fn text(&self) -> String {
        self.ty.clone()
    }

    pub fn set_text(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLAudioElement {
    deref_target: HTMLMediaElement,
}

impl ::std::ops::Deref for HTMLAudioElement {
    type Target = HTMLMediaElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLAudioElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct Text {
    deref_target: CharacterData,
}

impl ::std::ops::Deref for Text {
    type Target = CharacterData;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl Text {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLButtonElement {
    deref_target: HTMLElement,
    autofocus: bool,
    disabled: bool,
    formNoValidate: bool,
    formTarget: String,
    name: String,
    type_: String,
    value: String,
}

impl ::std::ops::Deref for HTMLButtonElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLButtonElement {
    pub fn autofocus(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_autofocus(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn disabled(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn formNoValidate(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_formNoValidate(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn formTarget(&self) -> String {
        self.ty.clone()
    }

    pub fn set_formTarget(&mut self, value: String) {
        self.ty = value;
    }

    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }

    pub fn type_(&self) -> String {
        self.ty.clone()
    }

    pub fn set_type_(&mut self, value: String) {
        self.ty = value;
    }

    pub fn value(&self) -> String {
        self.ty.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLModElement {
    deref_target: HTMLElement,
    cite: String,
    dateTime: String,
}

impl ::std::ops::Deref for HTMLModElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLModElement {
    pub fn cite(&self) -> String {
        self.ty.clone()
    }

    pub fn set_cite(&mut self, value: String) {
        self.ty = value;
    }

    pub fn dateTime(&self) -> String {
        self.ty.clone()
    }

    pub fn set_dateTime(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLMapElement {
    deref_target: HTMLElement,
    name: String,
}

impl ::std::ops::Deref for HTMLMapElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLMapElement {
    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct SVGGraphicsElement {
    deref_target: SVGElement,
}

impl ::std::ops::Deref for SVGGraphicsElement {
    type Target = SVGElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl SVGGraphicsElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLTableCellElement {
    deref_target: HTMLElement,
    colSpan: u64,
    rowSpan: u64,
    headers: String,
    scope: String,
    abbr: String,
}

impl ::std::ops::Deref for HTMLTableCellElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLTableCellElement {
    pub fn colSpan(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_colSpan(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn rowSpan(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_rowSpan(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn headers(&self) -> String {
        self.ty.clone()
    }

    pub fn set_headers(&mut self, value: String) {
        self.ty = value;
    }

    pub fn scope(&self) -> String {
        self.ty.clone()
    }

    pub fn set_scope(&mut self, value: String) {
        self.ty = value;
    }

    pub fn abbr(&self) -> String {
        self.ty.clone()
    }

    pub fn set_abbr(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLOptGroupElement {
    deref_target: HTMLElement,
    disabled: bool,
    label: String,
}

impl ::std::ops::Deref for HTMLOptGroupElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLOptGroupElement {
    pub fn disabled(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn label(&self) -> String {
        self.ty.clone()
    }

    pub fn set_label(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLTableCaptionElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLTableCaptionElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLTableCaptionElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLDListElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLDListElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLDListElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLOptionsCollection {
    deref_target: HTMLCollection,
    length: u64,
    selectedIndex: u64,
}

impl ::std::ops::Deref for HTMLOptionsCollection {
    type Target = HTMLCollection;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLOptionsCollection {
    pub fn length(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_length(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn selectedIndex(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_selectedIndex(&mut self, value: u64) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLFontElement {
    deref_target: HTMLElement,
    color: String,
    face: String,
    size: String,
}

impl ::std::ops::Deref for HTMLFontElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLFontElement {
    pub fn color(&self) -> String {
        self.ty.clone()
    }

    pub fn set_color(&mut self, value: String) {
        self.ty = value;
    }

    pub fn face(&self) -> String {
        self.ty.clone()
    }

    pub fn set_face(&mut self, value: String) {
        self.ty = value;
    }

    pub fn size(&self) -> String {
        self.ty.clone()
    }

    pub fn set_size(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct XMLDocument {
    deref_target: Document,
}

impl ::std::ops::Deref for XMLDocument {
    type Target = Document;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl XMLDocument {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLTableSectionElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLTableSectionElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLTableSectionElement {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLDialogElement {
    deref_target: HTMLElement,
    open: bool,
}

impl ::std::ops::Deref for HTMLDialogElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLDialogElement {
    pub fn open(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_open(&mut self, value: bool) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLDataElement {
    deref_target: HTMLElement,
    value: String,
}

impl ::std::ops::Deref for HTMLDataElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLDataElement {
    pub fn value(&self) -> String {
        self.ty.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct ShadowRoot {
    deref_target: DocumentFragment,
    innerHTML: String,
}

impl ::std::ops::Deref for ShadowRoot {
    type Target = DocumentFragment;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl ShadowRoot {
    pub fn innerHTML(&self) -> String {
        self.ty.clone()
    }

    pub fn set_innerHTML(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLTableElement {
    deref_target: HTMLElement,
    caption: HTMLTableCaptionElement,
    tHead: HTMLTableSectionElement,
    tFoot: HTMLTableSectionElement,
}

impl ::std::ops::Deref for HTMLTableElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLTableElement {
    pub fn caption(&self) -> HTMLTableCaptionElement {
        self.ty.clone()
    }

    pub fn set_caption(&mut self, value: HTMLTableCaptionElement) {
        self.ty = value;
    }

    pub fn tHead(&self) -> HTMLTableSectionElement {
        self.ty.clone()
    }

    pub fn set_tHead(&mut self, value: HTMLTableSectionElement) {
        self.ty = value;
    }

    pub fn tFoot(&self) -> HTMLTableSectionElement {
        self.ty.clone()
    }

    pub fn set_tFoot(&mut self, value: HTMLTableSectionElement) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLDetailsElement {
    deref_target: HTMLElement,
    open: bool,
}

impl ::std::ops::Deref for HTMLDetailsElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLDetailsElement {
    pub fn open(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_open(&mut self, value: bool) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct DOMStringMap {

}



impl DOMStringMap {
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLIFrameElement {
    deref_target: HTMLElement,
    src: String,
    srcdoc: String,
    name: String,
    allowFullscreen: bool,
    width: String,
    height: String,
}

impl ::std::ops::Deref for HTMLIFrameElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLIFrameElement {
    pub fn src(&self) -> String {
        self.ty.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.ty = value;
    }

    pub fn srcdoc(&self) -> String {
        self.ty.clone()
    }

    pub fn set_srcdoc(&mut self, value: String) {
        self.ty = value;
    }

    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }

    pub fn allowFullscreen(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_allowFullscreen(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn width(&self) -> String {
        self.ty.clone()
    }

    pub fn set_width(&mut self, value: String) {
        self.ty = value;
    }

    pub fn height(&self) -> String {
        self.ty.clone()
    }

    pub fn set_height(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct Element {
    deref_target: Node,
    id: String,
    className: String,
    slot: String,
}

impl ::std::ops::Deref for Element {
    type Target = Node;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl Element {
    pub fn id(&self) -> String {
        self.ty.clone()
    }

    pub fn set_id(&mut self, value: String) {
        self.ty = value;
    }

    pub fn className(&self) -> String {
        self.ty.clone()
    }

    pub fn set_className(&mut self, value: String) {
        self.ty = value;
    }

    pub fn slot(&self) -> String {
        self.ty.clone()
    }

    pub fn set_slot(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLParamElement {
    deref_target: HTMLElement,
    name: String,
    value: String,
}

impl ::std::ops::Deref for HTMLParamElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLParamElement {
    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }

    pub fn value(&self) -> String {
        self.ty.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLBaseElement {
    deref_target: HTMLElement,
    href: String,
    target: String,
}

impl ::std::ops::Deref for HTMLBaseElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLBaseElement {
    pub fn href(&self) -> String {
        self.ty.clone()
    }

    pub fn set_href(&mut self, value: String) {
        self.ty = value;
    }

    pub fn target(&self) -> String {
        self.ty.clone()
    }

    pub fn set_target(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLAreaElement {
    deref_target: HTMLElement,
    alt: String,
    coords: String,
    shape: String,
    target: String,
    rel: String,
}

impl ::std::ops::Deref for HTMLAreaElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLAreaElement {
    pub fn alt(&self) -> String {
        self.ty.clone()
    }

    pub fn set_alt(&mut self, value: String) {
        self.ty = value;
    }

    pub fn coords(&self) -> String {
        self.ty.clone()
    }

    pub fn set_coords(&mut self, value: String) {
        self.ty = value;
    }

    pub fn shape(&self) -> String {
        self.ty.clone()
    }

    pub fn set_shape(&mut self, value: String) {
        self.ty = value;
    }

    pub fn target(&self) -> String {
        self.ty.clone()
    }

    pub fn set_target(&mut self, value: String) {
        self.ty = value;
    }

    pub fn rel(&self) -> String {
        self.ty.clone()
    }

    pub fn set_rel(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLFieldSetElement {
    deref_target: HTMLElement,
    disabled: bool,
    name: String,
}

impl ::std::ops::Deref for HTMLFieldSetElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLFieldSetElement {
    pub fn disabled(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLVideoElement {
    deref_target: HTMLMediaElement,
    width: u64,
    height: u64,
    poster: String,
    playsInline: bool,
}

impl ::std::ops::Deref for HTMLVideoElement {
    type Target = HTMLMediaElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLVideoElement {
    pub fn width(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_width(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn height(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_height(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn poster(&self) -> String {
        self.ty.clone()
    }

    pub fn set_poster(&mut self, value: String) {
        self.ty = value;
    }

    pub fn playsInline(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_playsInline(&mut self, value: bool) {
        self.ty = value;
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct HTMLInputElement {
    deref_target: HTMLElement,
    accept: String,
    alt: String,
    autocomplete: String,
    autofocus: bool,
    defaultChecked: bool,
    checked: bool,
    dirName: String,
    disabled: bool,
    files: FileList,
    formNoValidate: bool,
    formTarget: String,
    indeterminate: bool,
    inputMode: String,
    max: String,
    maxLength: u64,
    min: String,
    minLength: u64,
    multiple: bool,
    name: String,
    pattern: String,
    placeholder: String,
    readOnly: bool,
    required: bool,
    size: u64,
    src: String,
    step: String,
    type_: String,
    defaultValue: String,
    value: String,
    valueAsNumber: f64,
    selectionStart: u64,
    selectionEnd: u64,
    selectionDirection: String,
}

impl ::std::ops::Deref for HTMLInputElement {
    type Target = HTMLElement;
    fn deref(&self) -> Self::Target {
        &self.deref_target
    }
}

impl HTMLInputElement {
    pub fn accept(&self) -> String {
        self.ty.clone()
    }

    pub fn set_accept(&mut self, value: String) {
        self.ty = value;
    }

    pub fn alt(&self) -> String {
        self.ty.clone()
    }

    pub fn set_alt(&mut self, value: String) {
        self.ty = value;
    }

    pub fn autocomplete(&self) -> String {
        self.ty.clone()
    }

    pub fn set_autocomplete(&mut self, value: String) {
        self.ty = value;
    }

    pub fn autofocus(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_autofocus(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn defaultChecked(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_defaultChecked(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn checked(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_checked(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn dirName(&self) -> String {
        self.ty.clone()
    }

    pub fn set_dirName(&mut self, value: String) {
        self.ty = value;
    }

    pub fn disabled(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn files(&self) -> FileList {
        self.ty.clone()
    }

    pub fn set_files(&mut self, value: FileList) {
        self.ty = value;
    }

    pub fn formNoValidate(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_formNoValidate(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn formTarget(&self) -> String {
        self.ty.clone()
    }

    pub fn set_formTarget(&mut self, value: String) {
        self.ty = value;
    }

    pub fn indeterminate(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_indeterminate(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn inputMode(&self) -> String {
        self.ty.clone()
    }

    pub fn set_inputMode(&mut self, value: String) {
        self.ty = value;
    }

    pub fn max(&self) -> String {
        self.ty.clone()
    }

    pub fn set_max(&mut self, value: String) {
        self.ty = value;
    }

    pub fn maxLength(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_maxLength(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn min(&self) -> String {
        self.ty.clone()
    }

    pub fn set_min(&mut self, value: String) {
        self.ty = value;
    }

    pub fn minLength(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_minLength(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn multiple(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_multiple(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn name(&self) -> String {
        self.ty.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.ty = value;
    }

    pub fn pattern(&self) -> String {
        self.ty.clone()
    }

    pub fn set_pattern(&mut self, value: String) {
        self.ty = value;
    }

    pub fn placeholder(&self) -> String {
        self.ty.clone()
    }

    pub fn set_placeholder(&mut self, value: String) {
        self.ty = value;
    }

    pub fn readOnly(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_readOnly(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn required(&self) -> bool {
        self.ty.clone()
    }

    pub fn set_required(&mut self, value: bool) {
        self.ty = value;
    }

    pub fn size(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_size(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn src(&self) -> String {
        self.ty.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.ty = value;
    }

    pub fn step(&self) -> String {
        self.ty.clone()
    }

    pub fn set_step(&mut self, value: String) {
        self.ty = value;
    }

    pub fn type_(&self) -> String {
        self.ty.clone()
    }

    pub fn set_type_(&mut self, value: String) {
        self.ty = value;
    }

    pub fn defaultValue(&self) -> String {
        self.ty.clone()
    }

    pub fn set_defaultValue(&mut self, value: String) {
        self.ty = value;
    }

    pub fn value(&self) -> String {
        self.ty.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.ty = value;
    }

    pub fn valueAsNumber(&self) -> f64 {
        self.ty.clone()
    }

    pub fn set_valueAsNumber(&mut self, value: f64) {
        self.ty = value;
    }

    pub fn selectionStart(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_selectionStart(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn selectionEnd(&self) -> u64 {
        self.ty.clone()
    }

    pub fn set_selectionEnd(&mut self, value: u64) {
        self.ty = value;
    }

    pub fn selectionDirection(&self) -> String {
        self.ty.clone()
    }

    pub fn set_selectionDirection(&mut self, value: String) {
        self.ty = value;
    }
}

