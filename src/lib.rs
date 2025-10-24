pub fn search<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    contents.lines().filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(
    query: &'a str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> + 'a {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Then";
        let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know..";

        assert_eq!(
            vec!["Then there's a pair of us - don't tell!"],
            search(query, contents).collect::<Vec<_>>()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "iSh";
        let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.";

        assert_eq!(
            vec!["They'd banish us, you know."],
            search_case_insensitive(query, contents).collect::<Vec<_>>()
        );
    }
}
