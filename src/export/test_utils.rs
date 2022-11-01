use std::collections::HashSet;

pub fn assert_all_lines_arbitrary_order(left: String, right: String) {
    let mut expected_lines: HashSet<&str> = HashSet::new();

    for line in right.lines() {
        expected_lines.insert(line.trim());
    }

    for line in left.lines() {
        if !expected_lines.contains(line.trim()) {
            panic!(
                "Line not present in dot output: {}\nExpected output: {}",
                line, right
            );
        }
    }
}
