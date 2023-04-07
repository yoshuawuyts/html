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
