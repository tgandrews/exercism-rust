pub fn build_proverb(list: &[&str]) -> String {
    (0..list.len())
        .map(|i: usize| {
            let is_last = i == (list.len() - 1);
            match i {
                _i if is_last => format!("And all for the want of a {}.", list[0]),
                item_index => format!(
                    "For want of a {} the {} was lost.",
                    list[item_index],
                    list[item_index + 1]
                ),
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
