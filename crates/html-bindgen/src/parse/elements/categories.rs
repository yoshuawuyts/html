use regex::Regex;

pub(crate) fn parse_categories(mut line: &str) -> Vec<String> {
    if line.contains("Zero or more") {
        let mut iter = line.split("Zero or more");
        let _ = iter.next().unwrap();
        line = iter.next().unwrap();
        let mut output = vec![];
        for word in line.split("and") {
            output.extend(parse_category(word));
        }
        output
    } else if line.starts_with("Inside") {
        line = line.strip_prefix("Inside").unwrap();
        parse_category(line)
    } else {
        parse_category(line)
    }
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
    let line = line.to_lowercase();
    let mut line = line.as_str();
    line = maybe_strip_suffix(line, ".");

    let mut output = vec![];

    if line.starts_with("if the") {
        if line.contains(":") {
            let mut iter = line.split(":");
            _ = iter.next().unwrap();
            line = iter.next().unwrap();
        }
    }

    let re = Regex::new("no .*content.*descendants").unwrap();
    let line = re.replace_all(line, "");

    let re = Regex::new("no .*element.*descendants").unwrap();
    let line = re.replace_all(&line, "");

    let re = Regex::new("no .*descendants").unwrap();
    let line = re.replace_all(&line, "").to_string();

    // let re = Regex::new(r"where (\w+) content is expected").unwrap();
    // if let Some(captures) = re.captures_iter(&line).into_iter().next() {
    //     output.push(captures[1].to_owned());
    // }

    // let re = Regex::new(r"in a (\w+) element").unwrap();
    // if let Some(captures) = re.captures_iter(&line).into_iter().next() {
    //     output.push(captures[1].to_owned());
    // }

    let re = Regex::new(r"([\w-]+) and ([\w-]+) elements").unwrap();
    for captures in re.captures_iter(&line) {
        output.push(dbg!(captures[1].to_owned()));
        output.push(dbg!(captures[2].to_owned()));
    }

    let re = Regex::new(r"([\w-]+) element").unwrap();
    for captures in re.captures_iter(&line) {
        output.push(captures[1].to_owned());
    }

    let re = Regex::new(r"([\w-]+) and ([\w-]+) elements").unwrap();
    for captures in re.captures_iter(&line) {
        output.push(dbg!(captures[1].to_owned()));
        output.push(dbg!(captures[2].to_owned()));
    }

    let re = Regex::new(r"([\w-]+) content").unwrap();
    for captures in re.captures_iter(&line) {
        output.push(captures[1].to_owned());
    }

    // let regex = Regex::new(r"as the first [element]? [child]? [of a|in an] (\w+) element").unwrap();
    // if let Some(captures) = regex.captures_iter(line).into_iter().next() {
    //     return captures[1].to_owned();
    // }

    // line = maybe_strip_prefix(line, "as the first");
    // line = maybe_strip_prefix(line, "element");
    // line = maybe_strip_prefix(line, "child");
    // line = maybe_strip_prefix(line, "of a");
    // line = maybe_strip_prefix(line, "in an");
    // line = maybe_strip_suffix(line, "element");

    // line = maybe_strip_prefix(line, "where");
    // line = maybe_strip_suffix(line, "is expected");
    // line = maybe_strip_suffix(line, "are expected");

    // if line.contains(",") {
    //     let mut iter = line.split(",");
    //     line = iter.next().unwrap();
    // }
    // if line.contains(".") {
    //     let mut iter = line.split(".");
    //     line = iter.next().unwrap();
    // }

    // if line.contains("content") {
    //     let mut iter = line.split("content");
    //     line = iter.next().unwrap();
    // }

    // if line.contains("elements") {
    //     let mut iter = line.split("elements");
    //     line = iter.next().unwrap();
    // }

    // output.push(line.trim().to_lowercase().to_owned());
    output.dedup();
    output
}

// fn maybe_strip_prefix<'a>(mut s: &'a str, pattern: &str) -> &'a str {
//     if s.trim().starts_with(pattern) {
//         s = s.strip_prefix(pattern).unwrap();
//     }
//     s.trim()
// }

fn maybe_strip_suffix<'a>(mut s: &'a str, pattern: &str) -> &'a str {
    if s.trim().starts_with(pattern) {
        s = s.strip_suffix(pattern).unwrap();
    }
    s.trim()
}
