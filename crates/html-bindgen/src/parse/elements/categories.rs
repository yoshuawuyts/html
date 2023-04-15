use regex::Regex;

pub(crate) fn parse_categories(line: &str) -> Vec<String> {
    parse_category(line)
}

/// Parses individual lines of category prose into categories and elements.
/// As a guiding principle allowing invalid elements is less of a big deal
/// than disallowing valid elements. We'll work to make things more type-safe
/// over time. But it's a far bigger deal if a basic construct just isn't
/// expressible.
///
/// The parsing algorithm currently works as follows:
///   1. Start by stripping away the *negative* matches; the things
///      we explicitly shouldn't include. For now we just ignore those.
///   2. Without any negative matches we can move to find the *positive*
///      matches. This inclcludes both elements and categories.
fn parse_category(line: &str) -> Vec<String> {
    let line = line.replace('\n', "").to_lowercase();
    let mut output = vec![];

    let re = Regex::new("no .*content.*descendants").unwrap();
    let line = re.replace_all(&line, "");

    let re = Regex::new("no .*element.*descendants").unwrap();
    let line = re.replace_all(&line, "");

    let re = Regex::new("no .*descendants").unwrap();
    let line = re.replace_all(&line, "").to_string();

    let re = Regex::new(r"([\w-]+) and ([\w-]+) elements").unwrap();
    for captures in re.captures_iter(&line) {
        output.push(captures[1].to_owned());
        output.push(captures[2].to_owned());
    }

    let re = Regex::new(r"([\w-]+) element").unwrap();
    for captures in re.captures_iter(&line) {
        output.push(captures[1].to_owned());
    }

    let re = Regex::new(r"([\w-]+) and ([\w-]+) elements").unwrap();
    for captures in re.captures_iter(&line) {
        output.push(captures[1].to_owned());
        output.push(captures[2].to_owned());
    }

    let re = Regex::new(r"([\w-]+), ([\w-]+), (and )?([\w-]+) elements").unwrap();
    for captures in re.captures_iter(&line) {
        output.push(captures[1].to_owned());
        output.push(captures[2].to_owned());
        output.push(captures[4].to_owned());
    }

    let re = Regex::new(r"([\w-]+) content").unwrap();
    for captures in re.captures_iter(&line) {
        output.push(captures[1].to_owned());
    }

    // We just try and find this string and insert all the right headers.
    let re = Regex::new(r"h1, h2,[\s]+h3, h4, h5, or h6 element").unwrap();
    if re.find(&line).is_some() {
        for header in 1..=6 {
            output.push(format!("h{header}"));
        }
    }

    output.dedup();
    output.sort();
    output
}
