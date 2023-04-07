/// The HTML `<script>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/script)
#[doc(alias = "script")]
#[non_exhaustive]
pub struct Script {
    
            /// Address of the resource
            pub src: std::option::Option<String>,

            /// Type of script
            pub type_: std::option::Option<String>,

            /// Prevents execution in user agents that support module scripts
            pub nomodule: std::option::Option<String>,

            /// Execute script when available, without blocking while fetching
            pub async_: std::option::Option<String>,

            /// Defer script execution
            pub defer: std::option::Option<String>,

            /// How the element handles crossorigin requests
            pub crossorigin: std::option::Option<String>,

            /// Integrity metadata used in Subresource Integrity checks [SRI]
            pub integrity: std::option::Option<String>,

            /// Referrer policy for fetches initiated by the element
            pub referrerpolicy: std::option::Option<String>,

            /// Whether the element is potentially render-blocking
            pub blocking: std::option::Option<String>,

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
