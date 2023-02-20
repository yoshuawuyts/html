/// An HTML Element
        pub trait HtmlElement: ::std::fmt::Display {}

        #[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLBodyElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLBodyElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLBodyElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLBodyElement {
    
}

impl HtmlElement for HTMLBodyElement {}

impl ::std::fmt::Display for HTMLBodyElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<body>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</body>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLInputElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::Deref for HTMLInputElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLInputElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLInputElement {
    pub fn accept(&self) -> String {
    String::clone(&self.accept)
}

pub fn set_accept(&mut self, value: String) {
    self.accept = value;
}
pub fn alt(&self) -> String {
    String::clone(&self.alt)
}

pub fn set_alt(&mut self, value: String) {
    self.alt = value;
}
pub fn autocomplete(&self) -> String {
    String::clone(&self.autocomplete)
}

pub fn set_autocomplete(&mut self, value: String) {
    self.autocomplete = value;
}
pub fn autofocus(&self) -> bool {
    bool::clone(&self.autofocus)
}

pub fn set_autofocus(&mut self, value: bool) {
    self.autofocus = value;
}
pub fn default_checked(&self) -> bool {
    bool::clone(&self.default_checked)
}

pub fn set_default_checked(&mut self, value: bool) {
    self.default_checked = value;
}
pub fn checked(&self) -> bool {
    bool::clone(&self.checked)
}

pub fn set_checked(&mut self, value: bool) {
    self.checked = value;
}
pub fn dir_name(&self) -> String {
    String::clone(&self.dir_name)
}

pub fn set_dir_name(&mut self, value: String) {
    self.dir_name = value;
}
pub fn disabled(&self) -> bool {
    bool::clone(&self.disabled)
}

pub fn set_disabled(&mut self, value: bool) {
    self.disabled = value;
}
pub fn files(&self) -> FileList {
    FileList::clone(&self.files)
}

pub fn set_files(&mut self, value: FileList) {
    self.files = value;
}
pub fn form_no_validate(&self) -> bool {
    bool::clone(&self.form_no_validate)
}

pub fn set_form_no_validate(&mut self, value: bool) {
    self.form_no_validate = value;
}
pub fn form_target(&self) -> String {
    String::clone(&self.form_target)
}

pub fn set_form_target(&mut self, value: String) {
    self.form_target = value;
}
pub fn indeterminate(&self) -> bool {
    bool::clone(&self.indeterminate)
}

pub fn set_indeterminate(&mut self, value: bool) {
    self.indeterminate = value;
}
pub fn input_mode(&self) -> String {
    String::clone(&self.input_mode)
}

pub fn set_input_mode(&mut self, value: String) {
    self.input_mode = value;
}
pub fn max(&self) -> String {
    String::clone(&self.max)
}

pub fn set_max(&mut self, value: String) {
    self.max = value;
}
pub fn max_length(&self) -> u64 {
    u64::clone(&self.max_length)
}

pub fn set_max_length(&mut self, value: u64) {
    self.max_length = value;
}
pub fn min(&self) -> String {
    String::clone(&self.min)
}

pub fn set_min(&mut self, value: String) {
    self.min = value;
}
pub fn min_length(&self) -> u64 {
    u64::clone(&self.min_length)
}

pub fn set_min_length(&mut self, value: u64) {
    self.min_length = value;
}
pub fn multiple(&self) -> bool {
    bool::clone(&self.multiple)
}

pub fn set_multiple(&mut self, value: bool) {
    self.multiple = value;
}
pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
pub fn pattern(&self) -> String {
    String::clone(&self.pattern)
}

pub fn set_pattern(&mut self, value: String) {
    self.pattern = value;
}
pub fn placeholder(&self) -> String {
    String::clone(&self.placeholder)
}

pub fn set_placeholder(&mut self, value: String) {
    self.placeholder = value;
}
pub fn read_only(&self) -> bool {
    bool::clone(&self.read_only)
}

pub fn set_read_only(&mut self, value: bool) {
    self.read_only = value;
}
pub fn required(&self) -> bool {
    bool::clone(&self.required)
}

pub fn set_required(&mut self, value: bool) {
    self.required = value;
}
pub fn size(&self) -> u64 {
    u64::clone(&self.size)
}

pub fn set_size(&mut self, value: u64) {
    self.size = value;
}
pub fn src(&self) -> String {
    String::clone(&self.src)
}

pub fn set_src(&mut self, value: String) {
    self.src = value;
}
pub fn step(&self) -> String {
    String::clone(&self.step)
}

pub fn set_step(&mut self, value: String) {
    self.step = value;
}
pub fn ty(&self) -> String {
    String::clone(&self.ty)
}

pub fn set_ty(&mut self, value: String) {
    self.ty = value;
}
pub fn default_value(&self) -> String {
    String::clone(&self.default_value)
}

pub fn set_default_value(&mut self, value: String) {
    self.default_value = value;
}
pub fn value(&self) -> String {
    String::clone(&self.value)
}

pub fn set_value(&mut self, value: String) {
    self.value = value;
}
pub fn value_as_number(&self) -> f64 {
    f64::clone(&self.value_as_number)
}

pub fn set_value_as_number(&mut self, value: f64) {
    self.value_as_number = value;
}
pub fn selection_start(&self) -> u64 {
    u64::clone(&self.selection_start)
}

pub fn set_selection_start(&mut self, value: u64) {
    self.selection_start = value;
}
pub fn selection_end(&self) -> u64 {
    u64::clone(&self.selection_end)
}

pub fn set_selection_end(&mut self, value: u64) {
    self.selection_end = value;
}
pub fn selection_direction(&self) -> String {
    String::clone(&self.selection_direction)
}

pub fn set_selection_direction(&mut self, value: String) {
    self.selection_direction = value;
}
}

impl HtmlElement for HTMLInputElement {}

impl ::std::fmt::Display for HTMLInputElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<input>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</input>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLLinkElement {
    deref_target: HTMLElement,
children: usize,
    href: String,
    cross_origin: String,
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

impl ::std::ops::DerefMut for HTMLLinkElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLLinkElement {
    pub fn href(&self) -> String {
    String::clone(&self.href)
}

pub fn set_href(&mut self, value: String) {
    self.href = value;
}
pub fn cross_origin(&self) -> String {
    String::clone(&self.cross_origin)
}

pub fn set_cross_origin(&mut self, value: String) {
    self.cross_origin = value;
}
pub fn rel(&self) -> String {
    String::clone(&self.rel)
}

pub fn set_rel(&mut self, value: String) {
    self.rel = value;
}
pub fn media(&self) -> String {
    String::clone(&self.media)
}

pub fn set_media(&mut self, value: String) {
    self.media = value;
}
pub fn hreflang(&self) -> String {
    String::clone(&self.hreflang)
}

pub fn set_hreflang(&mut self, value: String) {
    self.hreflang = value;
}
pub fn ty(&self) -> String {
    String::clone(&self.ty)
}

pub fn set_ty(&mut self, value: String) {
    self.ty = value;
}
}

impl HtmlElement for HTMLLinkElement {}

impl ::std::fmt::Display for HTMLLinkElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<link>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</link>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTrackElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::DerefMut for HTMLTrackElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLTrackElement {
    pub fn kind(&self) -> String {
    String::clone(&self.kind)
}

pub fn set_kind(&mut self, value: String) {
    self.kind = value;
}
pub fn src(&self) -> String {
    String::clone(&self.src)
}

pub fn set_src(&mut self, value: String) {
    self.src = value;
}
pub fn srclang(&self) -> String {
    String::clone(&self.srclang)
}

pub fn set_srclang(&mut self, value: String) {
    self.srclang = value;
}
pub fn label(&self) -> String {
    String::clone(&self.label)
}

pub fn set_label(&mut self, value: String) {
    self.label = value;
}
pub fn default(&self) -> bool {
    bool::clone(&self.default)
}

pub fn set_default(&mut self, value: bool) {
    self.default = value;
}
}

impl HtmlElement for HTMLTrackElement {}

impl ::std::fmt::Display for HTMLTrackElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<track>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</track>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTitleElement {
    deref_target: HTMLElement,
children: usize,
    text: String,
}

impl ::std::ops::Deref for HTMLTitleElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLTitleElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLTitleElement {
    pub fn text(&self) -> String {
    String::clone(&self.text)
}

pub fn set_text(&mut self, value: String) {
    self.text = value;
}
}

impl HtmlElement for HTMLTitleElement {}

impl ::std::fmt::Display for HTMLTitleElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<title>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</title>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDivElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLDivElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLDivElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLDivElement {
    
}

impl HtmlElement for HTMLDivElement {}

impl ::std::fmt::Display for HTMLDivElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<div>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</div>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLHeadingElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLHeadingElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLHeadingElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLHeadingElement {
    
}

impl HtmlElement for HTMLHeadingElement {}

impl ::std::fmt::Display for HTMLHeadingElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<heading>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</heading>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLAreaElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::DerefMut for HTMLAreaElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLAreaElement {
    pub fn alt(&self) -> String {
    String::clone(&self.alt)
}

pub fn set_alt(&mut self, value: String) {
    self.alt = value;
}
pub fn coords(&self) -> String {
    String::clone(&self.coords)
}

pub fn set_coords(&mut self, value: String) {
    self.coords = value;
}
pub fn shape(&self) -> String {
    String::clone(&self.shape)
}

pub fn set_shape(&mut self, value: String) {
    self.shape = value;
}
pub fn target(&self) -> String {
    String::clone(&self.target)
}

pub fn set_target(&mut self, value: String) {
    self.target = value;
}
pub fn rel(&self) -> String {
    String::clone(&self.rel)
}

pub fn set_rel(&mut self, value: String) {
    self.rel = value;
}
}

impl HtmlElement for HTMLAreaElement {}

impl ::std::fmt::Display for HTMLAreaElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<area>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</area>")?;
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

impl ::std::ops::DerefMut for SVGElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl SVGElement {
    
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMeterElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::DerefMut for HTMLMeterElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLMeterElement {
    pub fn value(&self) -> f64 {
    f64::clone(&self.value)
}

pub fn set_value(&mut self, value: f64) {
    self.value = value;
}
pub fn min(&self) -> f64 {
    f64::clone(&self.min)
}

pub fn set_min(&mut self, value: f64) {
    self.min = value;
}
pub fn max(&self) -> f64 {
    f64::clone(&self.max)
}

pub fn set_max(&mut self, value: f64) {
    self.max = value;
}
pub fn low(&self) -> f64 {
    f64::clone(&self.low)
}

pub fn set_low(&mut self, value: f64) {
    self.low = value;
}
pub fn high(&self) -> f64 {
    f64::clone(&self.high)
}

pub fn set_high(&mut self, value: f64) {
    self.high = value;
}
pub fn optimum(&self) -> f64 {
    f64::clone(&self.optimum)
}

pub fn set_optimum(&mut self, value: f64) {
    self.optimum = value;
}
}

impl HtmlElement for HTMLMeterElement {}

impl ::std::fmt::Display for HTMLMeterElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<meter>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</meter>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableColElement {
    deref_target: HTMLElement,
children: usize,
    span: u64,
}

impl ::std::ops::Deref for HTMLTableColElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLTableColElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLTableColElement {
    pub fn span(&self) -> u64 {
    u64::clone(&self.span)
}

pub fn set_span(&mut self, value: u64) {
    self.span = value;
}
}

impl HtmlElement for HTMLTableColElement {}

impl ::std::fmt::Display for HTMLTableColElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<tablecol>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</tablecol>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDirectoryElement {
    deref_target: HTMLElement,
children: usize,
    compact: bool,
}

impl ::std::ops::Deref for HTMLDirectoryElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLDirectoryElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLDirectoryElement {
    pub fn compact(&self) -> bool {
    bool::clone(&self.compact)
}

pub fn set_compact(&mut self, value: bool) {
    self.compact = value;
}
}

impl HtmlElement for HTMLDirectoryElement {}

impl ::std::fmt::Display for HTMLDirectoryElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<directory>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</directory>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLImageElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::Deref for HTMLImageElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLImageElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLImageElement {
    pub fn alt(&self) -> String {
    String::clone(&self.alt)
}

pub fn set_alt(&mut self, value: String) {
    self.alt = value;
}
pub fn src(&self) -> String {
    String::clone(&self.src)
}

pub fn set_src(&mut self, value: String) {
    self.src = value;
}
pub fn srcset(&self) -> String {
    String::clone(&self.srcset)
}

pub fn set_srcset(&mut self, value: String) {
    self.srcset = value;
}
pub fn sizes(&self) -> String {
    String::clone(&self.sizes)
}

pub fn set_sizes(&mut self, value: String) {
    self.sizes = value;
}
pub fn cross_origin(&self) -> String {
    String::clone(&self.cross_origin)
}

pub fn set_cross_origin(&mut self, value: String) {
    self.cross_origin = value;
}
pub fn use_map(&self) -> String {
    String::clone(&self.use_map)
}

pub fn set_use_map(&mut self, value: String) {
    self.use_map = value;
}
pub fn is_map(&self) -> bool {
    bool::clone(&self.is_map)
}

pub fn set_is_map(&mut self, value: bool) {
    self.is_map = value;
}
pub fn width(&self) -> u64 {
    u64::clone(&self.width)
}

pub fn set_width(&mut self, value: u64) {
    self.width = value;
}
pub fn height(&self) -> u64 {
    u64::clone(&self.height)
}

pub fn set_height(&mut self, value: u64) {
    self.height = value;
}
}

impl HtmlElement for HTMLImageElement {}

impl ::std::fmt::Display for HTMLImageElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<image>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</image>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLObjectElement {
    deref_target: HTMLElement,
children: usize,
    data: String,
    ty: String,
    name: String,
    use_map: String,
    width: String,
    height: String,
}

impl ::std::ops::Deref for HTMLObjectElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLObjectElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLObjectElement {
    pub fn data(&self) -> String {
    String::clone(&self.data)
}

pub fn set_data(&mut self, value: String) {
    self.data = value;
}
pub fn ty(&self) -> String {
    String::clone(&self.ty)
}

pub fn set_ty(&mut self, value: String) {
    self.ty = value;
}
pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
pub fn use_map(&self) -> String {
    String::clone(&self.use_map)
}

pub fn set_use_map(&mut self, value: String) {
    self.use_map = value;
}
pub fn width(&self) -> String {
    String::clone(&self.width)
}

pub fn set_width(&mut self, value: String) {
    self.width = value;
}
pub fn height(&self) -> String {
    String::clone(&self.height)
}

pub fn set_height(&mut self, value: String) {
    self.height = value;
}
}

impl HtmlElement for HTMLObjectElement {}

impl ::std::fmt::Display for HTMLObjectElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<object>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</object>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLCanvasElement {
    deref_target: HTMLElement,
children: usize,
    width: u64,
    height: u64,
}

impl ::std::ops::Deref for HTMLCanvasElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLCanvasElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLCanvasElement {
    pub fn width(&self) -> u64 {
    u64::clone(&self.width)
}

pub fn set_width(&mut self, value: u64) {
    self.width = value;
}
pub fn height(&self) -> u64 {
    u64::clone(&self.height)
}

pub fn set_height(&mut self, value: u64) {
    self.height = value;
}
}

impl HtmlElement for HTMLCanvasElement {}

impl ::std::fmt::Display for HTMLCanvasElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<canvas>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</canvas>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDetailsElement {
    deref_target: HTMLElement,
children: usize,
    open: bool,
}

impl ::std::ops::Deref for HTMLDetailsElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLDetailsElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLDetailsElement {
    pub fn open(&self) -> bool {
    bool::clone(&self.open)
}

pub fn set_open(&mut self, value: bool) {
    self.open = value;
}
}

impl HtmlElement for HTMLDetailsElement {}

impl ::std::fmt::Display for HTMLDetailsElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<details>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</details>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLSelectElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::Deref for HTMLSelectElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLSelectElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLSelectElement {
    pub fn autofocus(&self) -> bool {
    bool::clone(&self.autofocus)
}

pub fn set_autofocus(&mut self, value: bool) {
    self.autofocus = value;
}
pub fn disabled(&self) -> bool {
    bool::clone(&self.disabled)
}

pub fn set_disabled(&mut self, value: bool) {
    self.disabled = value;
}
pub fn multiple(&self) -> bool {
    bool::clone(&self.multiple)
}

pub fn set_multiple(&mut self, value: bool) {
    self.multiple = value;
}
pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
pub fn required(&self) -> bool {
    bool::clone(&self.required)
}

pub fn set_required(&mut self, value: bool) {
    self.required = value;
}
pub fn size(&self) -> u64 {
    u64::clone(&self.size)
}

pub fn set_size(&mut self, value: u64) {
    self.size = value;
}
pub fn length(&self) -> u64 {
    u64::clone(&self.length)
}

pub fn set_length(&mut self, value: u64) {
    self.length = value;
}
pub fn selected_index(&self) -> u64 {
    u64::clone(&self.selected_index)
}

pub fn set_selected_index(&mut self, value: u64) {
    self.selected_index = value;
}
pub fn value(&self) -> String {
    String::clone(&self.value)
}

pub fn set_value(&mut self, value: String) {
    self.value = value;
}
}

impl HtmlElement for HTMLSelectElement {}

impl ::std::fmt::Display for HTMLSelectElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<select>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</select>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLFrameSetElement {
    deref_target: HTMLElement,
children: usize,
    cols: String,
    rows: String,
}

impl ::std::ops::Deref for HTMLFrameSetElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLFrameSetElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLFrameSetElement {
    pub fn cols(&self) -> String {
    String::clone(&self.cols)
}

pub fn set_cols(&mut self, value: String) {
    self.cols = value;
}
pub fn rows(&self) -> String {
    String::clone(&self.rows)
}

pub fn set_rows(&mut self, value: String) {
    self.rows = value;
}
}

impl HtmlElement for HTMLFrameSetElement {}

impl ::std::fmt::Display for HTMLFrameSetElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<frameset>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</frameset>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLModElement {
    deref_target: HTMLElement,
children: usize,
    cite: String,
    date_time: String,
}

impl ::std::ops::Deref for HTMLModElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLModElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLModElement {
    pub fn cite(&self) -> String {
    String::clone(&self.cite)
}

pub fn set_cite(&mut self, value: String) {
    self.cite = value;
}
pub fn date_time(&self) -> String {
    String::clone(&self.date_time)
}

pub fn set_date_time(&mut self, value: String) {
    self.date_time = value;
}
}

impl HtmlElement for HTMLModElement {}

impl ::std::fmt::Display for HTMLModElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<mod>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</mod>")?;
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

impl ::std::ops::DerefMut for HTMLOptionsCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLOptionsCollection {
    pub fn length(&self) -> u64 {
    u64::clone(&self.length)
}

pub fn set_length(&mut self, value: u64) {
    self.length = value;
}
pub fn selected_index(&self) -> u64 {
    u64::clone(&self.selected_index)
}

pub fn set_selected_index(&mut self, value: u64) {
    self.selected_index = value;
}
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLBaseElement {
    deref_target: HTMLElement,
children: usize,
    href: String,
    target: String,
}

impl ::std::ops::Deref for HTMLBaseElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLBaseElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLBaseElement {
    pub fn href(&self) -> String {
    String::clone(&self.href)
}

pub fn set_href(&mut self, value: String) {
    self.href = value;
}
pub fn target(&self) -> String {
    String::clone(&self.target)
}

pub fn set_target(&mut self, value: String) {
    self.target = value;
}
}

impl HtmlElement for HTMLBaseElement {}

impl ::std::fmt::Display for HTMLBaseElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<base>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</base>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct DOMStringMap {
    
}



impl DOMStringMap {
    
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDListElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLDListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLDListElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLDListElement {
    
}

impl HtmlElement for HTMLDListElement {}

impl ::std::fmt::Display for HTMLDListElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<dlist>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</dlist>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLProgressElement {
    deref_target: HTMLElement,
children: usize,
    value: f64,
    max: f64,
}

impl ::std::ops::Deref for HTMLProgressElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLProgressElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLProgressElement {
    pub fn value(&self) -> f64 {
    f64::clone(&self.value)
}

pub fn set_value(&mut self, value: f64) {
    self.value = value;
}
pub fn max(&self) -> f64 {
    f64::clone(&self.max)
}

pub fn set_max(&mut self, value: f64) {
    self.max = value;
}
}

impl HtmlElement for HTMLProgressElement {}

impl ::std::fmt::Display for HTMLProgressElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<progress>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</progress>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLParamElement {
    deref_target: HTMLElement,
children: usize,
    name: String,
    value: String,
}

impl ::std::ops::Deref for HTMLParamElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLParamElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLParamElement {
    pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
pub fn value(&self) -> String {
    String::clone(&self.value)
}

pub fn set_value(&mut self, value: String) {
    self.value = value;
}
}

impl HtmlElement for HTMLParamElement {}

impl ::std::fmt::Display for HTMLParamElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<param>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</param>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLUListElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLUListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLUListElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLUListElement {
    
}

impl HtmlElement for HTMLUListElement {}

impl ::std::fmt::Display for HTMLUListElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<ulist>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</ulist>")?;
        Ok(())
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

impl ::std::ops::DerefMut for SVGSVGElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl SVGSVGElement {
    
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLHRElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLHRElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLHRElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLHRElement {
    
}

impl HtmlElement for HTMLHRElement {}

impl ::std::fmt::Display for HTMLHRElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<hr>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</hr>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableCellElement {
    deref_target: HTMLElement,
children: usize,
    col_span: u64,
    row_span: u64,
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

impl ::std::ops::DerefMut for HTMLTableCellElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLTableCellElement {
    pub fn col_span(&self) -> u64 {
    u64::clone(&self.col_span)
}

pub fn set_col_span(&mut self, value: u64) {
    self.col_span = value;
}
pub fn row_span(&self) -> u64 {
    u64::clone(&self.row_span)
}

pub fn set_row_span(&mut self, value: u64) {
    self.row_span = value;
}
pub fn headers(&self) -> String {
    String::clone(&self.headers)
}

pub fn set_headers(&mut self, value: String) {
    self.headers = value;
}
pub fn scope(&self) -> String {
    String::clone(&self.scope)
}

pub fn set_scope(&mut self, value: String) {
    self.scope = value;
}
pub fn abbr(&self) -> String {
    String::clone(&self.abbr)
}

pub fn set_abbr(&mut self, value: String) {
    self.abbr = value;
}
}

impl HtmlElement for HTMLTableCellElement {}

impl ::std::fmt::Display for HTMLTableCellElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<tablecell>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</tablecell>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDataElement {
    deref_target: HTMLElement,
children: usize,
    value: String,
}

impl ::std::ops::Deref for HTMLDataElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLDataElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLDataElement {
    pub fn value(&self) -> String {
    String::clone(&self.value)
}

pub fn set_value(&mut self, value: String) {
    self.value = value;
}
}

impl HtmlElement for HTMLDataElement {}

impl ::std::fmt::Display for HTMLDataElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<data>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</data>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLLegendElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLLegendElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLLegendElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLLegendElement {
    
}

impl HtmlElement for HTMLLegendElement {}

impl ::std::fmt::Display for HTMLLegendElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<legend>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</legend>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLCollection {
    
}



impl HTMLCollection {
    
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

impl ::std::ops::DerefMut for XMLDocument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl XMLDocument {
    
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLOutputElement {
    deref_target: HTMLElement,
children: usize,
    name: String,
    default_value: String,
    value: String,
}

impl ::std::ops::Deref for HTMLOutputElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLOutputElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLOutputElement {
    pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
pub fn default_value(&self) -> String {
    String::clone(&self.default_value)
}

pub fn set_default_value(&mut self, value: String) {
    self.default_value = value;
}
pub fn value(&self) -> String {
    String::clone(&self.value)
}

pub fn set_value(&mut self, value: String) {
    self.value = value;
}
}

impl HtmlElement for HTMLOutputElement {}

impl ::std::fmt::Display for HTMLOutputElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<output>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</output>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLAudioElement {
    deref_target: HTMLMediaElement,
children: usize,
}

impl ::std::ops::Deref for HTMLAudioElement {
    type Target = HTMLMediaElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLAudioElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLAudioElement {
    
}

impl HtmlElement for HTMLAudioElement {}

impl ::std::fmt::Display for HTMLAudioElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<audio>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</audio>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLVideoElement {
    deref_target: HTMLMediaElement,
children: usize,
    width: u64,
    height: u64,
    poster: String,
    plays_inline: bool,
}

impl ::std::ops::Deref for HTMLVideoElement {
    type Target = HTMLMediaElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLVideoElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLVideoElement {
    pub fn width(&self) -> u64 {
    u64::clone(&self.width)
}

pub fn set_width(&mut self, value: u64) {
    self.width = value;
}
pub fn height(&self) -> u64 {
    u64::clone(&self.height)
}

pub fn set_height(&mut self, value: u64) {
    self.height = value;
}
pub fn poster(&self) -> String {
    String::clone(&self.poster)
}

pub fn set_poster(&mut self, value: String) {
    self.poster = value;
}
pub fn plays_inline(&self) -> bool {
    bool::clone(&self.plays_inline)
}

pub fn set_plays_inline(&mut self, value: bool) {
    self.plays_inline = value;
}
}

impl HtmlElement for HTMLVideoElement {}

impl ::std::fmt::Display for HTMLVideoElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<video>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</video>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLFrameElement {
    deref_target: HTMLElement,
children: usize,
    name: String,
    scrolling: String,
    src: String,
    frame_border: String,
    long_desc: String,
    no_resize: bool,
    margin_height: String,
    margin_width: String,
}

impl ::std::ops::Deref for HTMLFrameElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLFrameElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLFrameElement {
    pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
pub fn scrolling(&self) -> String {
    String::clone(&self.scrolling)
}

pub fn set_scrolling(&mut self, value: String) {
    self.scrolling = value;
}
pub fn src(&self) -> String {
    String::clone(&self.src)
}

pub fn set_src(&mut self, value: String) {
    self.src = value;
}
pub fn frame_border(&self) -> String {
    String::clone(&self.frame_border)
}

pub fn set_frame_border(&mut self, value: String) {
    self.frame_border = value;
}
pub fn long_desc(&self) -> String {
    String::clone(&self.long_desc)
}

pub fn set_long_desc(&mut self, value: String) {
    self.long_desc = value;
}
pub fn no_resize(&self) -> bool {
    bool::clone(&self.no_resize)
}

pub fn set_no_resize(&mut self, value: bool) {
    self.no_resize = value;
}
pub fn margin_height(&self) -> String {
    String::clone(&self.margin_height)
}

pub fn set_margin_height(&mut self, value: String) {
    self.margin_height = value;
}
pub fn margin_width(&self) -> String {
    String::clone(&self.margin_width)
}

pub fn set_margin_width(&mut self, value: String) {
    self.margin_width = value;
}
}

impl HtmlElement for HTMLFrameElement {}

impl ::std::fmt::Display for HTMLFrameElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<frame>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</frame>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTextAreaElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::Deref for HTMLTextAreaElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLTextAreaElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLTextAreaElement {
    pub fn autocomplete(&self) -> String {
    String::clone(&self.autocomplete)
}

pub fn set_autocomplete(&mut self, value: String) {
    self.autocomplete = value;
}
pub fn autofocus(&self) -> bool {
    bool::clone(&self.autofocus)
}

pub fn set_autofocus(&mut self, value: bool) {
    self.autofocus = value;
}
pub fn cols(&self) -> u64 {
    u64::clone(&self.cols)
}

pub fn set_cols(&mut self, value: u64) {
    self.cols = value;
}
pub fn dir_name(&self) -> String {
    String::clone(&self.dir_name)
}

pub fn set_dir_name(&mut self, value: String) {
    self.dir_name = value;
}
pub fn disabled(&self) -> bool {
    bool::clone(&self.disabled)
}

pub fn set_disabled(&mut self, value: bool) {
    self.disabled = value;
}
pub fn input_mode(&self) -> String {
    String::clone(&self.input_mode)
}

pub fn set_input_mode(&mut self, value: String) {
    self.input_mode = value;
}
pub fn max_length(&self) -> u64 {
    u64::clone(&self.max_length)
}

pub fn set_max_length(&mut self, value: u64) {
    self.max_length = value;
}
pub fn min_length(&self) -> u64 {
    u64::clone(&self.min_length)
}

pub fn set_min_length(&mut self, value: u64) {
    self.min_length = value;
}
pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
pub fn placeholder(&self) -> String {
    String::clone(&self.placeholder)
}

pub fn set_placeholder(&mut self, value: String) {
    self.placeholder = value;
}
pub fn read_only(&self) -> bool {
    bool::clone(&self.read_only)
}

pub fn set_read_only(&mut self, value: bool) {
    self.read_only = value;
}
pub fn required(&self) -> bool {
    bool::clone(&self.required)
}

pub fn set_required(&mut self, value: bool) {
    self.required = value;
}
pub fn rows(&self) -> u64 {
    u64::clone(&self.rows)
}

pub fn set_rows(&mut self, value: u64) {
    self.rows = value;
}
pub fn wrap(&self) -> String {
    String::clone(&self.wrap)
}

pub fn set_wrap(&mut self, value: String) {
    self.wrap = value;
}
pub fn default_value(&self) -> String {
    String::clone(&self.default_value)
}

pub fn set_default_value(&mut self, value: String) {
    self.default_value = value;
}
pub fn value(&self) -> String {
    String::clone(&self.value)
}

pub fn set_value(&mut self, value: String) {
    self.value = value;
}
pub fn selection_start(&self) -> u64 {
    u64::clone(&self.selection_start)
}

pub fn set_selection_start(&mut self, value: u64) {
    self.selection_start = value;
}
pub fn selection_end(&self) -> u64 {
    u64::clone(&self.selection_end)
}

pub fn set_selection_end(&mut self, value: u64) {
    self.selection_end = value;
}
pub fn selection_direction(&self) -> String {
    String::clone(&self.selection_direction)
}

pub fn set_selection_direction(&mut self, value: String) {
    self.selection_direction = value;
}
}

impl HtmlElement for HTMLTextAreaElement {}

impl ::std::fmt::Display for HTMLTextAreaElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<textarea>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</textarea>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLOListElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::DerefMut for HTMLOListElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLOListElement {
    pub fn reversed(&self) -> bool {
    bool::clone(&self.reversed)
}

pub fn set_reversed(&mut self, value: bool) {
    self.reversed = value;
}
pub fn start(&self) -> u64 {
    u64::clone(&self.start)
}

pub fn set_start(&mut self, value: u64) {
    self.start = value;
}
pub fn ty(&self) -> String {
    String::clone(&self.ty)
}

pub fn set_ty(&mut self, value: String) {
    self.ty = value;
}
}

impl HtmlElement for HTMLOListElement {}

impl ::std::fmt::Display for HTMLOListElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<olist>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</olist>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct NodeList {
    
}



impl NodeList {
    
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

impl ::std::ops::DerefMut for Document {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl Document {
    
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

impl ::std::ops::DerefMut for HTMLElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLElement {
    pub fn title(&self) -> String {
    String::clone(&self.title)
}

pub fn set_title(&mut self, value: String) {
    self.title = value;
}
pub fn lang(&self) -> String {
    String::clone(&self.lang)
}

pub fn set_lang(&mut self, value: String) {
    self.lang = value;
}
pub fn translate(&self) -> bool {
    bool::clone(&self.translate)
}

pub fn set_translate(&mut self, value: bool) {
    self.translate = value;
}
pub fn dir(&self) -> String {
    String::clone(&self.dir)
}

pub fn set_dir(&mut self, value: String) {
    self.dir = value;
}
pub fn hidden(&self) -> bool {
    bool::clone(&self.hidden)
}

pub fn set_hidden(&mut self, value: bool) {
    self.hidden = value;
}
pub fn access_key(&self) -> String {
    String::clone(&self.access_key)
}

pub fn set_access_key(&mut self, value: String) {
    self.access_key = value;
}
pub fn draggable(&self) -> bool {
    bool::clone(&self.draggable)
}

pub fn set_draggable(&mut self, value: bool) {
    self.draggable = value;
}
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMediaElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::Deref for HTMLMediaElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLMediaElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLMediaElement {
    pub fn src(&self) -> String {
    String::clone(&self.src)
}

pub fn set_src(&mut self, value: String) {
    self.src = value;
}
pub fn cross_origin(&self) -> String {
    String::clone(&self.cross_origin)
}

pub fn set_cross_origin(&mut self, value: String) {
    self.cross_origin = value;
}
pub fn preload(&self) -> String {
    String::clone(&self.preload)
}

pub fn set_preload(&mut self, value: String) {
    self.preload = value;
}
pub fn current_time(&self) -> f64 {
    f64::clone(&self.current_time)
}

pub fn set_current_time(&mut self, value: f64) {
    self.current_time = value;
}
pub fn default_playback_rate(&self) -> f64 {
    f64::clone(&self.default_playback_rate)
}

pub fn set_default_playback_rate(&mut self, value: f64) {
    self.default_playback_rate = value;
}
pub fn playback_rate(&self) -> f64 {
    f64::clone(&self.playback_rate)
}

pub fn set_playback_rate(&mut self, value: f64) {
    self.playback_rate = value;
}
pub fn autoplay(&self) -> bool {
    bool::clone(&self.autoplay)
}

pub fn set_autoplay(&mut self, value: bool) {
    self.autoplay = value;
}
pub fn loop_(&self) -> bool {
    bool::clone(&self.loop_)
}

pub fn set_loop_(&mut self, value: bool) {
    self.loop_ = value;
}
pub fn controls(&self) -> bool {
    bool::clone(&self.controls)
}

pub fn set_controls(&mut self, value: bool) {
    self.controls = value;
}
pub fn volume(&self) -> f64 {
    f64::clone(&self.volume)
}

pub fn set_volume(&mut self, value: f64) {
    self.volume = value;
}
pub fn muted(&self) -> bool {
    bool::clone(&self.muted)
}

pub fn set_muted(&mut self, value: bool) {
    self.muted = value;
}
pub fn default_muted(&self) -> bool {
    bool::clone(&self.default_muted)
}

pub fn set_default_muted(&mut self, value: bool) {
    self.default_muted = value;
}
}

impl HtmlElement for HTMLMediaElement {}

impl ::std::fmt::Display for HTMLMediaElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<media>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</media>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLEmbedElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::DerefMut for HTMLEmbedElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLEmbedElement {
    pub fn src(&self) -> String {
    String::clone(&self.src)
}

pub fn set_src(&mut self, value: String) {
    self.src = value;
}
pub fn ty(&self) -> String {
    String::clone(&self.ty)
}

pub fn set_ty(&mut self, value: String) {
    self.ty = value;
}
pub fn width(&self) -> String {
    String::clone(&self.width)
}

pub fn set_width(&mut self, value: String) {
    self.width = value;
}
pub fn height(&self) -> String {
    String::clone(&self.height)
}

pub fn set_height(&mut self, value: String) {
    self.height = value;
}
}

impl HtmlElement for HTMLEmbedElement {}

impl ::std::fmt::Display for HTMLEmbedElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<embed>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</embed>")?;
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

impl ::std::ops::DerefMut for CharacterData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl CharacterData {
    pub fn data(&self) -> String {
    String::clone(&self.data)
}

pub fn set_data(&mut self, value: String) {
    self.data = value;
}
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLStyleElement {
    deref_target: HTMLElement,
children: usize,
    media: String,
}

impl ::std::ops::Deref for HTMLStyleElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLStyleElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLStyleElement {
    pub fn media(&self) -> String {
    String::clone(&self.media)
}

pub fn set_media(&mut self, value: String) {
    self.media = value;
}
}

impl HtmlElement for HTMLStyleElement {}

impl ::std::fmt::Display for HTMLStyleElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<style>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</style>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLFormElement {
    deref_target: HTMLElement,
children: usize,
    accept_charset: String,
    action: String,
    enctype: String,
    method: String,
    name: String,
    no_validate: bool,
    target: String,
}

impl ::std::ops::Deref for HTMLFormElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLFormElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLFormElement {
    pub fn accept_charset(&self) -> String {
    String::clone(&self.accept_charset)
}

pub fn set_accept_charset(&mut self, value: String) {
    self.accept_charset = value;
}
pub fn action(&self) -> String {
    String::clone(&self.action)
}

pub fn set_action(&mut self, value: String) {
    self.action = value;
}
pub fn enctype(&self) -> String {
    String::clone(&self.enctype)
}

pub fn set_enctype(&mut self, value: String) {
    self.enctype = value;
}
pub fn method(&self) -> String {
    String::clone(&self.method)
}

pub fn set_method(&mut self, value: String) {
    self.method = value;
}
pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
pub fn no_validate(&self) -> bool {
    bool::clone(&self.no_validate)
}

pub fn set_no_validate(&mut self, value: bool) {
    self.no_validate = value;
}
pub fn target(&self) -> String {
    String::clone(&self.target)
}

pub fn set_target(&mut self, value: String) {
    self.target = value;
}
}

impl HtmlElement for HTMLFormElement {}

impl ::std::fmt::Display for HTMLFormElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<form>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</form>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableSectionElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLTableSectionElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLTableSectionElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLTableSectionElement {
    
}

impl HtmlElement for HTMLTableSectionElement {}

impl ::std::fmt::Display for HTMLTableSectionElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<tablesection>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</tablesection>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDialogElement {
    deref_target: HTMLElement,
children: usize,
    open: bool,
}

impl ::std::ops::Deref for HTMLDialogElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLDialogElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLDialogElement {
    pub fn open(&self) -> bool {
    bool::clone(&self.open)
}

pub fn set_open(&mut self, value: bool) {
    self.open = value;
}
}

impl HtmlElement for HTMLDialogElement {}

impl ::std::fmt::Display for HTMLDialogElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<dialog>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</dialog>")?;
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

impl ::std::ops::DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl Text {
    
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMapElement {
    deref_target: HTMLElement,
children: usize,
    name: String,
}

impl ::std::ops::Deref for HTMLMapElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLMapElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLMapElement {
    pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
}

impl HtmlElement for HTMLMapElement {}

impl ::std::fmt::Display for HTMLMapElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<map>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</map>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLFontElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::DerefMut for HTMLFontElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLFontElement {
    pub fn color(&self) -> String {
    String::clone(&self.color)
}

pub fn set_color(&mut self, value: String) {
    self.color = value;
}
pub fn face(&self) -> String {
    String::clone(&self.face)
}

pub fn set_face(&mut self, value: String) {
    self.face = value;
}
pub fn size(&self) -> String {
    String::clone(&self.size)
}

pub fn set_size(&mut self, value: String) {
    self.size = value;
}
}

impl HtmlElement for HTMLFontElement {}

impl ::std::fmt::Display for HTMLFontElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<font>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</font>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLSlotElement {
    deref_target: HTMLElement,
children: usize,
    name: String,
}

impl ::std::ops::Deref for HTMLSlotElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLSlotElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLSlotElement {
    pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
}

impl HtmlElement for HTMLSlotElement {}

impl ::std::fmt::Display for HTMLSlotElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<slot>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</slot>")?;
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

impl ::std::ops::DerefMut for ProcessingInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl ProcessingInstruction {
    
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

impl ::std::ops::DerefMut for CDATASection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl CDATASection {
    
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

impl ::std::ops::DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl Node {
    pub fn node_value(&self) -> String {
    String::clone(&self.node_value)
}

pub fn set_node_value(&mut self, value: String) {
    self.node_value = value;
}
pub fn text_content(&self) -> String {
    String::clone(&self.text_content)
}

pub fn set_text_content(&mut self, value: String) {
    self.text_content = value;
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

impl ::std::ops::DerefMut for SVGTitleElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl SVGTitleElement {
    
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

impl ::std::ops::DerefMut for DocumentFragment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl DocumentFragment {
    
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTimeElement {
    deref_target: HTMLElement,
children: usize,
    date_time: String,
}

impl ::std::ops::Deref for HTMLTimeElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLTimeElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLTimeElement {
    pub fn date_time(&self) -> String {
    String::clone(&self.date_time)
}

pub fn set_date_time(&mut self, value: String) {
    self.date_time = value;
}
}

impl HtmlElement for HTMLTimeElement {}

impl ::std::fmt::Display for HTMLTimeElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<time>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</time>")?;
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

impl ::std::ops::DerefMut for SVGGraphicsElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl SVGGraphicsElement {
    
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLHeadElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLHeadElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLHeadElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLHeadElement {
    
}

impl HtmlElement for HTMLHeadElement {}

impl ::std::fmt::Display for HTMLHeadElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<head>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</head>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLLIElement {
    deref_target: HTMLElement,
children: usize,
    value: u64,
}

impl ::std::ops::Deref for HTMLLIElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLLIElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLLIElement {
    pub fn value(&self) -> u64 {
    u64::clone(&self.value)
}

pub fn set_value(&mut self, value: u64) {
    self.value = value;
}
}

impl HtmlElement for HTMLLIElement {}

impl ::std::fmt::Display for HTMLLIElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<li>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</li>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLHtmlElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLHtmlElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLHtmlElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLHtmlElement {
    
}

impl HtmlElement for HTMLHtmlElement {}

impl ::std::fmt::Display for HTMLHtmlElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<html>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</html>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableElement {
    deref_target: HTMLElement,
children: usize,
    caption: HTMLTableCaptionElement,
    t_head: HTMLTableSectionElement,
    t_foot: HTMLTableSectionElement,
}

impl ::std::ops::Deref for HTMLTableElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLTableElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLTableElement {
    pub fn caption(&self) -> HTMLTableCaptionElement {
    HTMLTableCaptionElement::clone(&self.caption)
}

pub fn set_caption(&mut self, value: HTMLTableCaptionElement) {
    self.caption = value;
}
pub fn t_head(&self) -> HTMLTableSectionElement {
    HTMLTableSectionElement::clone(&self.t_head)
}

pub fn set_t_head(&mut self, value: HTMLTableSectionElement) {
    self.t_head = value;
}
pub fn t_foot(&self) -> HTMLTableSectionElement {
    HTMLTableSectionElement::clone(&self.t_foot)
}

pub fn set_t_foot(&mut self, value: HTMLTableSectionElement) {
    self.t_foot = value;
}
}

impl HtmlElement for HTMLTableElement {}

impl ::std::fmt::Display for HTMLTableElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<table>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</table>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLIFrameElement {
    deref_target: HTMLElement,
children: usize,
    src: String,
    srcdoc: String,
    name: String,
    allow_fullscreen: bool,
    width: String,
    height: String,
}

impl ::std::ops::Deref for HTMLIFrameElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLIFrameElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLIFrameElement {
    pub fn src(&self) -> String {
    String::clone(&self.src)
}

pub fn set_src(&mut self, value: String) {
    self.src = value;
}
pub fn srcdoc(&self) -> String {
    String::clone(&self.srcdoc)
}

pub fn set_srcdoc(&mut self, value: String) {
    self.srcdoc = value;
}
pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
pub fn allow_fullscreen(&self) -> bool {
    bool::clone(&self.allow_fullscreen)
}

pub fn set_allow_fullscreen(&mut self, value: bool) {
    self.allow_fullscreen = value;
}
pub fn width(&self) -> String {
    String::clone(&self.width)
}

pub fn set_width(&mut self, value: String) {
    self.width = value;
}
pub fn height(&self) -> String {
    String::clone(&self.height)
}

pub fn set_height(&mut self, value: String) {
    self.height = value;
}
}

impl HtmlElement for HTMLIFrameElement {}

impl ::std::fmt::Display for HTMLIFrameElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<iframe>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</iframe>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMetaElement {
    deref_target: HTMLElement,
children: usize,
    name: String,
    http_equiv: String,
    content: String,
}

impl ::std::ops::Deref for HTMLMetaElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLMetaElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLMetaElement {
    pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
pub fn http_equiv(&self) -> String {
    String::clone(&self.http_equiv)
}

pub fn set_http_equiv(&mut self, value: String) {
    self.http_equiv = value;
}
pub fn content(&self) -> String {
    String::clone(&self.content)
}

pub fn set_content(&mut self, value: String) {
    self.content = value;
}
}

impl HtmlElement for HTMLMetaElement {}

impl ::std::fmt::Display for HTMLMetaElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<meta>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</meta>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLOptGroupElement {
    deref_target: HTMLElement,
children: usize,
    disabled: bool,
    label: String,
}

impl ::std::ops::Deref for HTMLOptGroupElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLOptGroupElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLOptGroupElement {
    pub fn disabled(&self) -> bool {
    bool::clone(&self.disabled)
}

pub fn set_disabled(&mut self, value: bool) {
    self.disabled = value;
}
pub fn label(&self) -> String {
    String::clone(&self.label)
}

pub fn set_label(&mut self, value: String) {
    self.label = value;
}
}

impl HtmlElement for HTMLOptGroupElement {}

impl ::std::fmt::Display for HTMLOptGroupElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<optgroup>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</optgroup>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLOptionElement {
    deref_target: HTMLElement,
children: usize,
    disabled: bool,
    label: String,
    default_selected: bool,
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

impl ::std::ops::DerefMut for HTMLOptionElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLOptionElement {
    pub fn disabled(&self) -> bool {
    bool::clone(&self.disabled)
}

pub fn set_disabled(&mut self, value: bool) {
    self.disabled = value;
}
pub fn label(&self) -> String {
    String::clone(&self.label)
}

pub fn set_label(&mut self, value: String) {
    self.label = value;
}
pub fn default_selected(&self) -> bool {
    bool::clone(&self.default_selected)
}

pub fn set_default_selected(&mut self, value: bool) {
    self.default_selected = value;
}
pub fn selected(&self) -> bool {
    bool::clone(&self.selected)
}

pub fn set_selected(&mut self, value: bool) {
    self.selected = value;
}
pub fn value(&self) -> String {
    String::clone(&self.value)
}

pub fn set_value(&mut self, value: String) {
    self.value = value;
}
pub fn text(&self) -> String {
    String::clone(&self.text)
}

pub fn set_text(&mut self, value: String) {
    self.text = value;
}
}

impl HtmlElement for HTMLOptionElement {}

impl ::std::fmt::Display for HTMLOptionElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<option>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</option>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLAnchorElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::DerefMut for HTMLAnchorElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLAnchorElement {
    pub fn target(&self) -> String {
    String::clone(&self.target)
}

pub fn set_target(&mut self, value: String) {
    self.target = value;
}
pub fn download(&self) -> String {
    String::clone(&self.download)
}

pub fn set_download(&mut self, value: String) {
    self.download = value;
}
pub fn rel(&self) -> String {
    String::clone(&self.rel)
}

pub fn set_rel(&mut self, value: String) {
    self.rel = value;
}
pub fn hreflang(&self) -> String {
    String::clone(&self.hreflang)
}

pub fn set_hreflang(&mut self, value: String) {
    self.hreflang = value;
}
pub fn ty(&self) -> String {
    String::clone(&self.ty)
}

pub fn set_ty(&mut self, value: String) {
    self.ty = value;
}
pub fn text(&self) -> String {
    String::clone(&self.text)
}

pub fn set_text(&mut self, value: String) {
    self.text = value;
}
}

impl HtmlElement for HTMLAnchorElement {}

impl ::std::fmt::Display for HTMLAnchorElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<anchor>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</anchor>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTemplateElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLTemplateElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLTemplateElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLTemplateElement {
    
}

impl HtmlElement for HTMLTemplateElement {}

impl ::std::fmt::Display for HTMLTemplateElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<template>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</template>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMenuElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLMenuElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLMenuElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLMenuElement {
    
}

impl HtmlElement for HTMLMenuElement {}

impl ::std::fmt::Display for HTMLMenuElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<menu>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</menu>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLSourceElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::DerefMut for HTMLSourceElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLSourceElement {
    pub fn src(&self) -> String {
    String::clone(&self.src)
}

pub fn set_src(&mut self, value: String) {
    self.src = value;
}
pub fn ty(&self) -> String {
    String::clone(&self.ty)
}

pub fn set_ty(&mut self, value: String) {
    self.ty = value;
}
pub fn srcset(&self) -> String {
    String::clone(&self.srcset)
}

pub fn set_srcset(&mut self, value: String) {
    self.srcset = value;
}
pub fn sizes(&self) -> String {
    String::clone(&self.sizes)
}

pub fn set_sizes(&mut self, value: String) {
    self.sizes = value;
}
pub fn media(&self) -> String {
    String::clone(&self.media)
}

pub fn set_media(&mut self, value: String) {
    self.media = value;
}
}

impl HtmlElement for HTMLSourceElement {}

impl ::std::fmt::Display for HTMLSourceElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<source>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</source>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLMarqueeElement {
    deref_target: HTMLElement,
children: usize,
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

impl ::std::ops::Deref for HTMLMarqueeElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLMarqueeElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLMarqueeElement {
    pub fn behavior(&self) -> String {
    String::clone(&self.behavior)
}

pub fn set_behavior(&mut self, value: String) {
    self.behavior = value;
}
pub fn bg_color(&self) -> String {
    String::clone(&self.bg_color)
}

pub fn set_bg_color(&mut self, value: String) {
    self.bg_color = value;
}
pub fn direction(&self) -> String {
    String::clone(&self.direction)
}

pub fn set_direction(&mut self, value: String) {
    self.direction = value;
}
pub fn height(&self) -> String {
    String::clone(&self.height)
}

pub fn set_height(&mut self, value: String) {
    self.height = value;
}
pub fn hspace(&self) -> u64 {
    u64::clone(&self.hspace)
}

pub fn set_hspace(&mut self, value: u64) {
    self.hspace = value;
}
pub fn scroll_amount(&self) -> u64 {
    u64::clone(&self.scroll_amount)
}

pub fn set_scroll_amount(&mut self, value: u64) {
    self.scroll_amount = value;
}
pub fn scroll_delay(&self) -> u64 {
    u64::clone(&self.scroll_delay)
}

pub fn set_scroll_delay(&mut self, value: u64) {
    self.scroll_delay = value;
}
pub fn true_speed(&self) -> bool {
    bool::clone(&self.true_speed)
}

pub fn set_true_speed(&mut self, value: bool) {
    self.true_speed = value;
}
pub fn vspace(&self) -> u64 {
    u64::clone(&self.vspace)
}

pub fn set_vspace(&mut self, value: u64) {
    self.vspace = value;
}
pub fn width(&self) -> String {
    String::clone(&self.width)
}

pub fn set_width(&mut self, value: String) {
    self.width = value;
}
}

impl HtmlElement for HTMLMarqueeElement {}

impl ::std::fmt::Display for HTMLMarqueeElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<marquee>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</marquee>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLScriptElement {
    deref_target: HTMLElement,
children: usize,
    src: String,
    ty: String,
    defer: bool,
    cross_origin: String,
    text: String,
}

impl ::std::ops::Deref for HTMLScriptElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLScriptElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLScriptElement {
    pub fn src(&self) -> String {
    String::clone(&self.src)
}

pub fn set_src(&mut self, value: String) {
    self.src = value;
}
pub fn ty(&self) -> String {
    String::clone(&self.ty)
}

pub fn set_ty(&mut self, value: String) {
    self.ty = value;
}
pub fn defer(&self) -> bool {
    bool::clone(&self.defer)
}

pub fn set_defer(&mut self, value: bool) {
    self.defer = value;
}
pub fn cross_origin(&self) -> String {
    String::clone(&self.cross_origin)
}

pub fn set_cross_origin(&mut self, value: String) {
    self.cross_origin = value;
}
pub fn text(&self) -> String {
    String::clone(&self.text)
}

pub fn set_text(&mut self, value: String) {
    self.text = value;
}
}

impl HtmlElement for HTMLScriptElement {}

impl ::std::fmt::Display for HTMLScriptElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<script>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</script>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct DOMTokenList {
    
    value: String,
}



impl DOMTokenList {
    pub fn value(&self) -> String {
    String::clone(&self.value)
}

pub fn set_value(&mut self, value: String) {
    self.value = value;
}
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLPreElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLPreElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLPreElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLPreElement {
    
}

impl HtmlElement for HTMLPreElement {}

impl ::std::fmt::Display for HTMLPreElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<pre>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</pre>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLQuoteElement {
    deref_target: HTMLElement,
children: usize,
    cite: String,
}

impl ::std::ops::Deref for HTMLQuoteElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLQuoteElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLQuoteElement {
    pub fn cite(&self) -> String {
    String::clone(&self.cite)
}

pub fn set_cite(&mut self, value: String) {
    self.cite = value;
}
}

impl HtmlElement for HTMLQuoteElement {}

impl ::std::fmt::Display for HTMLQuoteElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<quote>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</quote>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLBRElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLBRElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLBRElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLBRElement {
    
}

impl HtmlElement for HTMLBRElement {}

impl ::std::fmt::Display for HTMLBRElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<br>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</br>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLFieldSetElement {
    deref_target: HTMLElement,
children: usize,
    disabled: bool,
    name: String,
}

impl ::std::ops::Deref for HTMLFieldSetElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLFieldSetElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLFieldSetElement {
    pub fn disabled(&self) -> bool {
    bool::clone(&self.disabled)
}

pub fn set_disabled(&mut self, value: bool) {
    self.disabled = value;
}
pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
}

impl HtmlElement for HTMLFieldSetElement {}

impl ::std::fmt::Display for HTMLFieldSetElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<fieldset>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</fieldset>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableRowElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLTableRowElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLTableRowElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLTableRowElement {
    
}

impl HtmlElement for HTMLTableRowElement {}

impl ::std::fmt::Display for HTMLTableRowElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<tablerow>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</tablerow>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLPictureElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLPictureElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLPictureElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLPictureElement {
    
}

impl HtmlElement for HTMLPictureElement {}

impl ::std::fmt::Display for HTMLPictureElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<picture>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</picture>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLButtonElement {
    deref_target: HTMLElement,
children: usize,
    autofocus: bool,
    disabled: bool,
    form_no_validate: bool,
    form_target: String,
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

impl ::std::ops::DerefMut for HTMLButtonElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLButtonElement {
    pub fn autofocus(&self) -> bool {
    bool::clone(&self.autofocus)
}

pub fn set_autofocus(&mut self, value: bool) {
    self.autofocus = value;
}
pub fn disabled(&self) -> bool {
    bool::clone(&self.disabled)
}

pub fn set_disabled(&mut self, value: bool) {
    self.disabled = value;
}
pub fn form_no_validate(&self) -> bool {
    bool::clone(&self.form_no_validate)
}

pub fn set_form_no_validate(&mut self, value: bool) {
    self.form_no_validate = value;
}
pub fn form_target(&self) -> String {
    String::clone(&self.form_target)
}

pub fn set_form_target(&mut self, value: String) {
    self.form_target = value;
}
pub fn name(&self) -> String {
    String::clone(&self.name)
}

pub fn set_name(&mut self, value: String) {
    self.name = value;
}
pub fn ty(&self) -> String {
    String::clone(&self.ty)
}

pub fn set_ty(&mut self, value: String) {
    self.ty = value;
}
pub fn value(&self) -> String {
    String::clone(&self.value)
}

pub fn set_value(&mut self, value: String) {
    self.value = value;
}
}

impl HtmlElement for HTMLButtonElement {}

impl ::std::fmt::Display for HTMLButtonElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<button>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</button>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct DOMImplementation {
    
}



impl DOMImplementation {
    
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLSpanElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLSpanElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLSpanElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLSpanElement {
    
}

impl HtmlElement for HTMLSpanElement {}

impl ::std::fmt::Display for HTMLSpanElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<span>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</span>")?;
        Ok(())
    }
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

impl ::std::ops::DerefMut for Comment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl Comment {
    
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

impl ::std::ops::DerefMut for Element {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl Element {
    pub fn id(&self) -> String {
    String::clone(&self.id)
}

pub fn set_id(&mut self, value: String) {
    self.id = value;
}
pub fn class_name(&self) -> String {
    String::clone(&self.class_name)
}

pub fn set_class_name(&mut self, value: String) {
    self.class_name = value;
}
pub fn slot(&self) -> String {
    String::clone(&self.slot)
}

pub fn set_slot(&mut self, value: String) {
    self.slot = value;
}
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLParagraphElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLParagraphElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLParagraphElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLParagraphElement {
    
}

impl HtmlElement for HTMLParagraphElement {}

impl ::std::fmt::Display for HTMLParagraphElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<paragraph>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</paragraph>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLTableCaptionElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLTableCaptionElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLTableCaptionElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLTableCaptionElement {
    
}

impl HtmlElement for HTMLTableCaptionElement {}

impl ::std::fmt::Display for HTMLTableCaptionElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<tablecaption>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</tablecaption>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLDataListElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLDataListElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLDataListElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLDataListElement {
    
}

impl HtmlElement for HTMLDataListElement {}

impl ::std::fmt::Display for HTMLDataListElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<datalist>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</datalist>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLLabelElement {
    deref_target: HTMLElement,
children: usize,
    html_for: String,
}

impl ::std::ops::Deref for HTMLLabelElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLLabelElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLLabelElement {
    pub fn html_for(&self) -> String {
    String::clone(&self.html_for)
}

pub fn set_html_for(&mut self, value: String) {
    self.html_for = value;
}
}

impl HtmlElement for HTMLLabelElement {}

impl ::std::fmt::Display for HTMLLabelElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<label>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</label>")?;
        Ok(())
    }
}
#[derive(Default, Debug, PartialEq, Clone)]
pub struct HTMLUnknownElement {
    deref_target: HTMLElement,
children: usize,
}

impl ::std::ops::Deref for HTMLUnknownElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.deref_target
    }
}

impl ::std::ops::DerefMut for HTMLUnknownElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl HTMLUnknownElement {
    
}

impl HtmlElement for HTMLUnknownElement {}

impl ::std::fmt::Display for HTMLUnknownElement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "<unknown>")?;
        write!(f, "<{}>", self.children)?;
        write!(f, "</unknown>")?;
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

impl ::std::ops::DerefMut for ShadowRoot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl ShadowRoot {
    pub fn inner_html(&self) -> String {
    String::clone(&self.inner_html)
}

pub fn set_inner_html(&mut self, value: String) {
    self.inner_html = value;
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

impl ::std::ops::DerefMut for DocumentType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.deref_target
    }
}


impl DocumentType {
    
}

#[derive(Default, Debug, PartialEq, Clone)]
        pub struct FileList {}
        
        #[derive(Default, Debug, PartialEq, Clone)]
        pub struct EventTarget {}
        