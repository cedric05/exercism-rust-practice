pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        String::from("")
    } else {
        list.windows(2)
            .map(|y| format!("For want of a {} the {} was lost.", y[0], y[1]))
            .chain(std::iter::once(format!(
                "And all for the want of a {}.",
                list[0]
            )))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
