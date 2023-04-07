use crate::Result;

pub fn fmt(input: &str) -> Result<String> {
    let syntax_tree = syn::parse_file(&input)?;
    Ok(prettyplease::unparse(&syntax_tree))
}
