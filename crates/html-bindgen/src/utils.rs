use crate::Result;

/// Format generated Rust code prior to writing it.
pub fn fmt(input: &str) -> Result<String> {
    let syntax_tree = syn::parse_file(&input)?;
    Ok(prettyplease::unparse(&syntax_tree))
}

/// Extract the interface name from a webidl definition.
///
/// This tries to find the `interface` types only. It does not
/// yet capture `enum`-based types. In the future we should probably
/// also extract those to generate the right input enums.
// NOTE: if this stops working or becomes erroneous, replace it
// with a proper `weedle`-based extractor
pub(crate) fn extract_webidl_name(idl: &str) -> Option<String> {
    let name = (&idl).lines().find(|line| line.contains("interface"))?;
    let mut iter = name.split("interface");
    iter.next()?;
    let mut name = iter.next()?;

    if name.contains("mixin") {
        let mut iter = name.split("mixin");
        let _ = iter.next()?;
        name = iter.next()?;
    }

    if name.contains("{") {
        name = name.split("{").next()?;
    }
    if name.contains(":") {
        name = name.split(":").next()?;
    }
    Some(name.trim().to_owned())
}
