#[derive(Default, Debug, PartialEq, Clone)]
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
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLTableCellElement {
    pub fn colSpan(&self) -> u64 {
        self.colSpan.clone()
    }

    pub fn set_colSpan(&mut self, value: u64) {
        self.colSpan = value;
    }

    pub fn rowSpan(&self) -> u64 {
        self.rowSpan.clone()
    }

    pub fn set_rowSpan(&mut self, value: u64) {
        self.rowSpan = value;
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLLinkElement {
    deref_target: HTMLElement,
    href: String,
    crossOrigin: String,
    rel: String,
    media: String,
    hreflang: String,
    ty: String,
}

impl ::std::ops::Deref for HTMLLinkElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLLinkElement {
    pub fn href(&self) -> String {
        self.href.clone()
    }

    pub fn set_href(&mut self, value: String) {
        self.href = value;
    }

    pub fn crossOrigin(&self) -> String {
        self.crossOrigin.clone()
    }

    pub fn set_crossOrigin(&mut self, value: String) {
        self.crossOrigin = value;
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLParamElement {
    deref_target: HTMLElement,
    name: String,
    value: String,
}

impl ::std::ops::Deref for HTMLParamElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLParamElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLHRElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLHRElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLHRElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct DOMStringMap {

}



impl DOMStringMap {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableCaptionElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLTableCaptionElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLTableCaptionElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
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
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLTextAreaElement {
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

    pub fn dirName(&self) -> String {
        self.dirName.clone()
    }

    pub fn set_dirName(&mut self, value: String) {
        self.dirName = value;
    }

    pub fn disabled(&self) -> bool {
        self.disabled.clone()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.disabled = value;
    }

    pub fn inputMode(&self) -> String {
        self.inputMode.clone()
    }

    pub fn set_inputMode(&mut self, value: String) {
        self.inputMode = value;
    }

    pub fn maxLength(&self) -> u64 {
        self.maxLength.clone()
    }

    pub fn set_maxLength(&mut self, value: u64) {
        self.maxLength = value;
    }

    pub fn minLength(&self) -> u64 {
        self.minLength.clone()
    }

    pub fn set_minLength(&mut self, value: u64) {
        self.minLength = value;
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

    pub fn readOnly(&self) -> bool {
        self.readOnly.clone()
    }

    pub fn set_readOnly(&mut self, value: bool) {
        self.readOnly = value;
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

    pub fn defaultValue(&self) -> String {
        self.defaultValue.clone()
    }

    pub fn set_defaultValue(&mut self, value: String) {
        self.defaultValue = value;
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn selectionStart(&self) -> u64 {
        self.selectionStart.clone()
    }

    pub fn set_selectionStart(&mut self, value: u64) {
        self.selectionStart = value;
    }

    pub fn selectionEnd(&self) -> u64 {
        self.selectionEnd.clone()
    }

    pub fn set_selectionEnd(&mut self, value: u64) {
        self.selectionEnd = value;
    }

    pub fn selectionDirection(&self) -> String {
        self.selectionDirection.clone()
    }

    pub fn set_selectionDirection(&mut self, value: String) {
        self.selectionDirection = value;
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

impl SVGSVGElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLPreElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLPreElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLPreElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLVideoElement {
    deref_target: HTMLMediaElement,
    width: u64,
    height: u64,
    poster: String,
    playsInline: bool,
}

impl ::std::ops::Deref for HTMLVideoElement {
    type Target = HTMLMediaElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLVideoElement {
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

    pub fn playsInline(&self) -> bool {
        self.playsInline.clone()
    }

    pub fn set_playsInline(&mut self, value: bool) {
        self.playsInline = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMenuElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLMenuElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLMenuElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
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
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLSelectElement {
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

    pub fn selectedIndex(&self) -> u64 {
        self.selectedIndex.clone()
    }

    pub fn set_selectedIndex(&mut self, value: u64) {
        self.selectedIndex = value;
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLFrameSetElement {
    deref_target: HTMLElement,
    cols: String,
    rows: String,
}

impl ::std::ops::Deref for HTMLFrameSetElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLFrameSetElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLBodyElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLBodyElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLBodyElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLProgressElement {
    deref_target: HTMLElement,
    value: f64,
    max: f64,
}

impl ::std::ops::Deref for HTMLProgressElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLProgressElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLButtonElement {
    deref_target: HTMLElement,
    autofocus: bool,
    disabled: bool,
    formNoValidate: bool,
    formTarget: String,
    name: String,
    ty: String,
    value: String,
}

impl ::std::ops::Deref for HTMLButtonElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLButtonElement {
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

    pub fn formNoValidate(&self) -> bool {
        self.formNoValidate.clone()
    }

    pub fn set_formNoValidate(&mut self, value: bool) {
        self.formNoValidate = value;
    }

    pub fn formTarget(&self) -> String {
        self.formTarget.clone()
    }

    pub fn set_formTarget(&mut self, value: String) {
        self.formTarget = value;
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLSlotElement {
    deref_target: HTMLElement,
    name: String,
}

impl ::std::ops::Deref for HTMLSlotElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLSlotElement {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDetailsElement {
    deref_target: HTMLElement,
    open: bool,
}

impl ::std::ops::Deref for HTMLDetailsElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLDetailsElement {
    pub fn open(&self) -> bool {
        self.open.clone()
    }

    pub fn set_open(&mut self, value: bool) {
        self.open = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTitleElement {
    deref_target: HTMLElement,
    text: String,
}

impl ::std::ops::Deref for HTMLTitleElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLTitleElement {
    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, value: String) {
        self.text = value;
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

impl Text {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLOListElement {
    deref_target: HTMLElement,
    reversed: bool,
    start: u64,
    ty: String,
}

impl ::std::ops::Deref for HTMLOListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLOListElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLAudioElement {
    deref_target: HTMLMediaElement,
}

impl ::std::ops::Deref for HTMLAudioElement {
    type Target = HTMLMediaElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLAudioElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
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
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLMarqueeElement {
    pub fn behavior(&self) -> String {
        self.behavior.clone()
    }

    pub fn set_behavior(&mut self, value: String) {
        self.behavior = value;
    }

    pub fn bgColor(&self) -> String {
        self.bgColor.clone()
    }

    pub fn set_bgColor(&mut self, value: String) {
        self.bgColor = value;
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

    pub fn scrollAmount(&self) -> u64 {
        self.scrollAmount.clone()
    }

    pub fn set_scrollAmount(&mut self, value: u64) {
        self.scrollAmount = value;
    }

    pub fn scrollDelay(&self) -> u64 {
        self.scrollDelay.clone()
    }

    pub fn set_scrollDelay(&mut self, value: u64) {
        self.scrollDelay = value;
    }

    pub fn trueSpeed(&self) -> bool {
        self.trueSpeed.clone()
    }

    pub fn set_trueSpeed(&mut self, value: bool) {
        self.trueSpeed = value;
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDivElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLDivElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLDivElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLSpanElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLSpanElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLSpanElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDirectoryElement {
    deref_target: HTMLElement,
    compact: bool,
}

impl ::std::ops::Deref for HTMLDirectoryElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLDirectoryElement {
    pub fn compact(&self) -> bool {
        self.compact.clone()
    }

    pub fn set_compact(&mut self, value: bool) {
        self.compact = value;
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
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLImageElement {
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

    pub fn crossOrigin(&self) -> String {
        self.crossOrigin.clone()
    }

    pub fn set_crossOrigin(&mut self, value: String) {
        self.crossOrigin = value;
    }

    pub fn useMap(&self) -> String {
        self.useMap.clone()
    }

    pub fn set_useMap(&mut self, value: String) {
        self.useMap = value;
    }

    pub fn isMap(&self) -> bool {
        self.isMap.clone()
    }

    pub fn set_isMap(&mut self, value: bool) {
        self.isMap = value;
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableRowElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLTableRowElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLTableRowElement {
}

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

impl Comment {
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

impl Document {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLPictureElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLPictureElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLPictureElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLQuoteElement {
    deref_target: HTMLElement,
    cite: String,
}

impl ::std::ops::Deref for HTMLQuoteElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLQuoteElement {
    pub fn cite(&self) -> String {
        self.cite.clone()
    }

    pub fn set_cite(&mut self, value: String) {
        self.cite = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTemplateElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLTemplateElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLTemplateElement {
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

impl DocumentFragment {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLFontElement {
    deref_target: HTMLElement,
    color: String,
    face: String,
    size: String,
}

impl ::std::ops::Deref for HTMLFontElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLFontElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Node {
    deref_target: EventTarget,
    nodeValue: String,
    textContent: String,
}

impl ::std::ops::Deref for Node {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl Node {
    pub fn nodeValue(&self) -> String {
        self.nodeValue.clone()
    }

    pub fn set_nodeValue(&mut self, value: String) {
        self.nodeValue = value;
    }

    pub fn textContent(&self) -> String {
        self.textContent.clone()
    }

    pub fn set_textContent(&mut self, value: String) {
        self.textContent = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLUListElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLUListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLUListElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
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
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLMediaElement {
    pub fn src(&self) -> String {
        self.src.clone()
    }

    pub fn set_src(&mut self, value: String) {
        self.src = value;
    }

    pub fn crossOrigin(&self) -> String {
        self.crossOrigin.clone()
    }

    pub fn set_crossOrigin(&mut self, value: String) {
        self.crossOrigin = value;
    }

    pub fn preload(&self) -> String {
        self.preload.clone()
    }

    pub fn set_preload(&mut self, value: String) {
        self.preload = value;
    }

    pub fn currentTime(&self) -> f64 {
        self.currentTime.clone()
    }

    pub fn set_currentTime(&mut self, value: f64) {
        self.currentTime = value;
    }

    pub fn defaultPlaybackRate(&self) -> f64 {
        self.defaultPlaybackRate.clone()
    }

    pub fn set_defaultPlaybackRate(&mut self, value: f64) {
        self.defaultPlaybackRate = value;
    }

    pub fn playbackRate(&self) -> f64 {
        self.playbackRate.clone()
    }

    pub fn set_playbackRate(&mut self, value: f64) {
        self.playbackRate = value;
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

    pub fn defaultMuted(&self) -> bool {
        self.defaultMuted.clone()
    }

    pub fn set_defaultMuted(&mut self, value: bool) {
        self.defaultMuted = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMetaElement {
    deref_target: HTMLElement,
    name: String,
    httpEquiv: String,
    content: String,
}

impl ::std::ops::Deref for HTMLMetaElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLMetaElement {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }

    pub fn httpEquiv(&self) -> String {
        self.httpEquiv.clone()
    }

    pub fn set_httpEquiv(&mut self, value: String) {
        self.httpEquiv = value;
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn set_content(&mut self, value: String) {
        self.content = value;
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

impl SVGGraphicsElement {
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

impl SVGTitleElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDialogElement {
    deref_target: HTMLElement,
    open: bool,
}

impl ::std::ops::Deref for HTMLDialogElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLDialogElement {
    pub fn open(&self) -> bool {
        self.open.clone()
    }

    pub fn set_open(&mut self, value: bool) {
        self.open = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
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
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLMeterElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLModElement {
    deref_target: HTMLElement,
    cite: String,
    dateTime: String,
}

impl ::std::ops::Deref for HTMLModElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLModElement {
    pub fn cite(&self) -> String {
        self.cite.clone()
    }

    pub fn set_cite(&mut self, value: String) {
        self.cite = value;
    }

    pub fn dateTime(&self) -> String {
        self.dateTime.clone()
    }

    pub fn set_dateTime(&mut self, value: String) {
        self.dateTime = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLHeadElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLHeadElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLHeadElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLScriptElement {
    deref_target: HTMLElement,
    src: String,
    ty: String,
    defer: bool,
    crossOrigin: String,
    text: String,
}

impl ::std::ops::Deref for HTMLScriptElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLScriptElement {
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

    pub fn crossOrigin(&self) -> String {
        self.crossOrigin.clone()
    }

    pub fn set_crossOrigin(&mut self, value: String) {
        self.crossOrigin = value;
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn set_text(&mut self, value: String) {
        self.text = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLObjectElement {
    deref_target: HTMLElement,
    data: String,
    ty: String,
    name: String,
    useMap: String,
    width: String,
    height: String,
}

impl ::std::ops::Deref for HTMLObjectElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLObjectElement {
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

    pub fn useMap(&self) -> String {
        self.useMap.clone()
    }

    pub fn set_useMap(&mut self, value: String) {
        self.useMap = value;
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLBRElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLBRElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLBRElement {
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

impl XMLDocument {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLUnknownElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLUnknownElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLUnknownElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct ShadowRoot {
    deref_target: DocumentFragment,
    innerHTML: String,
}

impl ::std::ops::Deref for ShadowRoot {
    type Target = DocumentFragment;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ShadowRoot {
    pub fn innerHTML(&self) -> String {
        self.innerHTML.clone()
    }

    pub fn set_innerHTML(&mut self, value: String) {
        self.innerHTML = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
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
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLIFrameElement {
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

    pub fn allowFullscreen(&self) -> bool {
        self.allowFullscreen.clone()
    }

    pub fn set_allowFullscreen(&mut self, value: bool) {
        self.allowFullscreen = value;
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

#[derive(Default, Debug, PartialEq, Clone)]
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
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLTrackElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTimeElement {
    deref_target: HTMLElement,
    dateTime: String,
}

impl ::std::ops::Deref for HTMLTimeElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLTimeElement {
    pub fn dateTime(&self) -> String {
        self.dateTime.clone()
    }

    pub fn set_dateTime(&mut self, value: String) {
        self.dateTime = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLCanvasElement {
    deref_target: HTMLElement,
    width: u64,
    height: u64,
}

impl ::std::ops::Deref for HTMLCanvasElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLCanvasElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
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
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLAreaElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct DOMImplementation {

}



impl DOMImplementation {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDataElement {
    deref_target: HTMLElement,
    value: String,
}

impl ::std::ops::Deref for HTMLDataElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLDataElement {
    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLHeadingElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLHeadingElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLHeadingElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Element {
    deref_target: Node,
    id: String,
    className: String,
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

    pub fn className(&self) -> String {
        self.className.clone()
    }

    pub fn set_className(&mut self, value: String) {
        self.className = value;
    }

    pub fn slot(&self) -> String {
        self.slot.clone()
    }

    pub fn set_slot(&mut self, value: String) {
        self.slot = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLCollection {

}



impl HTMLCollection {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableSectionElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLTableSectionElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLTableSectionElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLEmbedElement {
    deref_target: HTMLElement,
    src: String,
    ty: String,
    width: String,
    height: String,
}

impl ::std::ops::Deref for HTMLEmbedElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLEmbedElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
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
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLFrameElement {
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

    pub fn frameBorder(&self) -> String {
        self.frameBorder.clone()
    }

    pub fn set_frameBorder(&mut self, value: String) {
        self.frameBorder = value;
    }

    pub fn longDesc(&self) -> String {
        self.longDesc.clone()
    }

    pub fn set_longDesc(&mut self, value: String) {
        self.longDesc = value;
    }

    pub fn noResize(&self) -> bool {
        self.noResize.clone()
    }

    pub fn set_noResize(&mut self, value: bool) {
        self.noResize = value;
    }

    pub fn marginHeight(&self) -> String {
        self.marginHeight.clone()
    }

    pub fn set_marginHeight(&mut self, value: String) {
        self.marginHeight = value;
    }

    pub fn marginWidth(&self) -> String {
        self.marginWidth.clone()
    }

    pub fn set_marginWidth(&mut self, value: String) {
        self.marginWidth = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLOutputElement {
    deref_target: HTMLElement,
    name: String,
    defaultValue: String,
    value: String,
}

impl ::std::ops::Deref for HTMLOutputElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLOutputElement {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
    }

    pub fn defaultValue(&self) -> String {
        self.defaultValue.clone()
    }

    pub fn set_defaultValue(&mut self, value: String) {
        self.defaultValue = value;
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLParagraphElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLParagraphElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLParagraphElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLLegendElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLLegendElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLLegendElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
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
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLFormElement {
    pub fn acceptCharset(&self) -> String {
        self.acceptCharset.clone()
    }

    pub fn set_acceptCharset(&mut self, value: String) {
        self.acceptCharset = value;
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

    pub fn noValidate(&self) -> bool {
        self.noValidate.clone()
    }

    pub fn set_noValidate(&mut self, value: bool) {
        self.noValidate = value;
    }

    pub fn target(&self) -> String {
        self.target.clone()
    }

    pub fn set_target(&mut self, value: String) {
        self.target = value;
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

impl SVGElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLFieldSetElement {
    deref_target: HTMLElement,
    disabled: bool,
    name: String,
}

impl ::std::ops::Deref for HTMLFieldSetElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLFieldSetElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLAnchorElement {
    deref_target: HTMLElement,
    target: String,
    download: String,
    rel: String,
    hreflang: String,
    ty: String,
    text: String,
}

impl ::std::ops::Deref for HTMLAnchorElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLAnchorElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLHtmlElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLHtmlElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLHtmlElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDListElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLDListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLDListElement {
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
pub struct HTMLTableElement {
    deref_target: HTMLElement,
    caption: HTMLTableCaptionElement,
    tHead: HTMLTableSectionElement,
    tFoot: HTMLTableSectionElement,
}

impl ::std::ops::Deref for HTMLTableElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLTableElement {
    pub fn caption(&self) -> HTMLTableCaptionElement {
        self.caption.clone()
    }

    pub fn set_caption(&mut self, value: HTMLTableCaptionElement) {
        self.caption = value;
    }

    pub fn tHead(&self) -> HTMLTableSectionElement {
        self.tHead.clone()
    }

    pub fn set_tHead(&mut self, value: HTMLTableSectionElement) {
        self.tHead = value;
    }

    pub fn tFoot(&self) -> HTMLTableSectionElement {
        self.tFoot.clone()
    }

    pub fn set_tFoot(&mut self, value: HTMLTableSectionElement) {
        self.tFoot = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct NodeList {

}



impl NodeList {
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

impl DocumentType {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDataListElement {
    deref_target: HTMLElement,
}

impl ::std::ops::Deref for HTMLDataListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLDataListElement {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMapElement {
    deref_target: HTMLElement,
    name: String,
}

impl ::std::ops::Deref for HTMLMapElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLMapElement {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, value: String) {
        self.name = value;
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

impl CDATASection {
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableColElement {
    deref_target: HTMLElement,
    span: u64,
}

impl ::std::ops::Deref for HTMLTableColElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLTableColElement {
    pub fn span(&self) -> u64 {
        self.span.clone()
    }

    pub fn set_span(&mut self, value: u64) {
        self.span = value;
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
    accessKey: String,
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

    pub fn accessKey(&self) -> String {
        self.accessKey.clone()
    }

    pub fn set_accessKey(&mut self, value: String) {
        self.accessKey = value;
    }

    pub fn draggable(&self) -> bool {
        self.draggable.clone()
    }

    pub fn set_draggable(&mut self, value: bool) {
        self.draggable = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLBaseElement {
    deref_target: HTMLElement,
    href: String,
    target: String,
}

impl ::std::ops::Deref for HTMLBaseElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLBaseElement {
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

impl ProcessingInstruction {
}

#[derive(Default, Debug, PartialEq, Clone)]
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
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLOptionElement {
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

    pub fn defaultSelected(&self) -> bool {
        self.defaultSelected.clone()
    }

    pub fn set_defaultSelected(&mut self, value: bool) {
        self.defaultSelected = value;
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLOptGroupElement {
    deref_target: HTMLElement,
    disabled: bool,
    label: String,
}

impl ::std::ops::Deref for HTMLOptGroupElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLOptGroupElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLLIElement {
    deref_target: HTMLElement,
    value: u64,
}

impl ::std::ops::Deref for HTMLLIElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLLIElement {
    pub fn value(&self) -> u64 {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: u64) {
        self.value = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLSourceElement {
    deref_target: HTMLElement,
    src: String,
    ty: String,
    srcset: String,
    sizes: String,
    media: String,
}

impl ::std::ops::Deref for HTMLSourceElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLSourceElement {
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLOptionsCollection {
    deref_target: HTMLCollection,
    length: u64,
    selectedIndex: u64,
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

    pub fn selectedIndex(&self) -> u64 {
        self.selectedIndex.clone()
    }

    pub fn set_selectedIndex(&mut self, value: u64) {
        self.selectedIndex = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLStyleElement {
    deref_target: HTMLElement,
    media: String,
}

impl ::std::ops::Deref for HTMLStyleElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLStyleElement {
    pub fn media(&self) -> String {
        self.media.clone()
    }

    pub fn set_media(&mut self, value: String) {
        self.media = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLLabelElement {
    deref_target: HTMLElement,
    htmlFor: String,
}

impl ::std::ops::Deref for HTMLLabelElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLLabelElement {
    pub fn htmlFor(&self) -> String {
        self.htmlFor.clone()
    }

    pub fn set_htmlFor(&mut self, value: String) {
        self.htmlFor = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
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
    ty: String,
    defaultValue: String,
    value: String,
    valueAsNumber: f64,
    selectionStart: u64,
    selectionEnd: u64,
    selectionDirection: String,
}

impl ::std::ops::Deref for HTMLInputElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl HTMLInputElement {
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

    pub fn defaultChecked(&self) -> bool {
        self.defaultChecked.clone()
    }

    pub fn set_defaultChecked(&mut self, value: bool) {
        self.defaultChecked = value;
    }

    pub fn checked(&self) -> bool {
        self.checked.clone()
    }

    pub fn set_checked(&mut self, value: bool) {
        self.checked = value;
    }

    pub fn dirName(&self) -> String {
        self.dirName.clone()
    }

    pub fn set_dirName(&mut self, value: String) {
        self.dirName = value;
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

    pub fn formNoValidate(&self) -> bool {
        self.formNoValidate.clone()
    }

    pub fn set_formNoValidate(&mut self, value: bool) {
        self.formNoValidate = value;
    }

    pub fn formTarget(&self) -> String {
        self.formTarget.clone()
    }

    pub fn set_formTarget(&mut self, value: String) {
        self.formTarget = value;
    }

    pub fn indeterminate(&self) -> bool {
        self.indeterminate.clone()
    }

    pub fn set_indeterminate(&mut self, value: bool) {
        self.indeterminate = value;
    }

    pub fn inputMode(&self) -> String {
        self.inputMode.clone()
    }

    pub fn set_inputMode(&mut self, value: String) {
        self.inputMode = value;
    }

    pub fn max(&self) -> String {
        self.max.clone()
    }

    pub fn set_max(&mut self, value: String) {
        self.max = value;
    }

    pub fn maxLength(&self) -> u64 {
        self.maxLength.clone()
    }

    pub fn set_maxLength(&mut self, value: u64) {
        self.maxLength = value;
    }

    pub fn min(&self) -> String {
        self.min.clone()
    }

    pub fn set_min(&mut self, value: String) {
        self.min = value;
    }

    pub fn minLength(&self) -> u64 {
        self.minLength.clone()
    }

    pub fn set_minLength(&mut self, value: u64) {
        self.minLength = value;
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

    pub fn readOnly(&self) -> bool {
        self.readOnly.clone()
    }

    pub fn set_readOnly(&mut self, value: bool) {
        self.readOnly = value;
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

    pub fn defaultValue(&self) -> String {
        self.defaultValue.clone()
    }

    pub fn set_defaultValue(&mut self, value: String) {
        self.defaultValue = value;
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn valueAsNumber(&self) -> f64 {
        self.valueAsNumber.clone()
    }

    pub fn set_valueAsNumber(&mut self, value: f64) {
        self.valueAsNumber = value;
    }

    pub fn selectionStart(&self) -> u64 {
        self.selectionStart.clone()
    }

    pub fn set_selectionStart(&mut self, value: u64) {
        self.selectionStart = value;
    }

    pub fn selectionEnd(&self) -> u64 {
        self.selectionEnd.clone()
    }

    pub fn set_selectionEnd(&mut self, value: u64) {
        self.selectionEnd = value;
    }

    pub fn selectionDirection(&self) -> String {
        self.selectionDirection.clone()
    }

    pub fn set_selectionDirection(&mut self, value: String) {
        self.selectionDirection = value;
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
        pub struct FileList {}
        
        #[derive(Default, Debug, PartialEq, Clone)]
        pub struct EventTarget {}
        