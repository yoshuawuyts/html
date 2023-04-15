pub(crate) fn parse_categories(mut line: &str) -> Vec<String> {
    if line.contains("Zero or more") {
        let mut iter = line.split("Zero or more");
        let _ = iter.next().unwrap();
        line = iter.next().unwrap();
        let mut output = vec![];
        for word in line.split("and") {
            output.push(parse_category(word));
        }
        output
    } else if line.starts_with("Inside") {
        line = line.strip_prefix("Inside").unwrap();
        vec![parse_category(line)]
    } else {
        vec![parse_category(line)]
    }
}

fn parse_category(mut line: &str) -> String {
    line = maybe_strip_suffix(line, ".");
    if line.starts_with("If the") {
        if line.contains(":") {
            let mut iter = line.split(":");
            _ = iter.next().unwrap();
            line = iter.next().unwrap();
        }
    }

    line = maybe_strip_prefix(line, "As the first ");
    line = maybe_strip_prefix(line, "element ");
    line = maybe_strip_prefix(line, "child ");
    line = maybe_strip_prefix(line, "of a ");
    line = maybe_strip_prefix(line, "in an ");
    line = maybe_strip_suffix(line, " element");

    line = maybe_strip_prefix(line, "Where ");
    line = maybe_strip_suffix(line, " is expected");
    line = maybe_strip_suffix(line, " are expected");

    if line.contains(",") {
        let mut iter = line.split(",");
        line = iter.next().unwrap();
    }
    if line.contains(".") {
        let mut iter = line.split(".");
        line = iter.next().unwrap();
    }

    if line.contains("content") {
        let mut iter = line.split("content");
        line = iter.next().unwrap();
    }

    if line.contains("elements") {
        let mut iter = line.split("elements");
        line = iter.next().unwrap();
    }

    line.trim().to_lowercase().to_owned()
}

fn maybe_strip_prefix<'a>(mut s: &'a str, pattern: &str) -> &'a str {
    if s.starts_with(pattern) {
        s = s.strip_prefix(pattern).unwrap();
    }
    s
}

fn maybe_strip_suffix<'a>(mut s: &'a str, pattern: &str) -> &'a str {
    if s.starts_with(pattern) {
        s = s.strip_suffix(pattern).unwrap();
    }
    s
}
