pub fn fmt(input: &str) -> crate::types::Result<String> {
    let syntax_tree = syn::parse_file(&input)?;
    Ok(prettyplease::unparse(&syntax_tree))
}
