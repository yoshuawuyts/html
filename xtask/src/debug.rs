use async_std::fs;
use async_std::path::PathBuf;
use html_bindgen::merge::MergedElement;
use html_bindgen::parse::ParsedElement;
use html_bindgen::scrape::ScrapedElement;

pub(crate) async fn debug(parent_name: String, child_name: String) -> crate::Result<()> {
    debug_scraped_elements(&parent_name, &child_name).await?;
    debug_parsed_elements(&parent_name, &child_name).await?;
    debug_merged_element(&parent_name, &child_name).await?;
    println!("=========== end ==========\n");

    Ok(())
}

async fn debug_scraped_elements(parent_name: &str, child_name: &str) -> crate::Result<()> {
    let mut parent = PathBuf::from("resources/scraped/elements");
    let mut child = parent.clone();

    parent.push(&parent_name);
    parent.set_extension("json");
    child.push(&child_name);
    child.set_extension("json");

    let parent = fs::read_to_string(parent).await?;
    let child = fs::read_to_string(child).await?;

    let parent: ScrapedElement = serde_json::from_str(&parent)?;
    let child: ScrapedElement = serde_json::from_str(&child)?;

    println!("===== SCRAPED ELEMENTS =====\n");

    println!("[parent ({parent_name})] content model");
    for item in parent.content_model {
        print!("{item}, ")
    }
    println!("\n");

    println!("[child ({child_name})] categories");
    for item in child.categories {
        print!("{item}, ")
    }
    println!("\n");

    println!("[child ({child_name})] contexts");
    for item in child.contexts {
        print!("{item}, ")
    }
    println!("\n");

    Ok(())
}

async fn debug_parsed_elements(parent_name: &str, child_name: &str) -> crate::Result<()> {
    let mut parent = PathBuf::from("resources/parsed/elements");
    let mut child = parent.clone();

    parent.push(&parent_name);
    parent.set_extension("json");
    child.push(&child_name);
    child.set_extension("json");

    let parent = fs::read_to_string(parent).await?;
    let child = fs::read_to_string(child).await?;
    let parent: ParsedElement = serde_json::from_str(&parent)?;
    let child: ParsedElement = serde_json::from_str(&child)?;

    println!("===== PARSED ELEMENTS =====\n");

    println!("[parent ({parent_name})] permitted content");
    for content in parent.permitted_content {
        print!("{content:?}, ")
    }
    println!("\n");

    println!("[child ({child_name})] content categories");
    for content in child.content_categories {
        print!("{content:?}, ")
    }
    println!("\n");

    println!("[child ({child_name})] permitted parents");
    for content in child.permitted_parents {
        print!("{content:?}, ")
    }
    println!("\n");

    Ok(())
}

async fn debug_merged_element(parent_name: &str, child_name: &str) -> crate::Result<()> {
    let mut parent = PathBuf::from("resources/merged/elements");
    let mut child = parent.clone();

    parent.push(&parent_name);
    parent.set_extension("json");
    child.push(&child_name);
    child.set_extension("json");

    let parent = fs::read_to_string(parent).await?;
    let child = fs::read_to_string(child).await?;
    let parent: MergedElement = serde_json::from_str(&parent)?;
    let child: MergedElement = serde_json::from_str(&child)?;

    println!("===== MERGED ELEMENTS =====\n");

    println!("[parent ({parent_name})] permitted child elements");
    for item in parent.permitted_child_elements {
        print!("{item:?}, ")
    }
    println!("\n");

    println!("[child ({child_name})] content categories");
    for content in child.content_categories {
        print!("{content:?}, ")
    }
    println!("\n");

    Ok(())
}
