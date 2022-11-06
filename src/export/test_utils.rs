pub fn assert_eq_lines_trim_whitespace(left: String, right: String) {
    let mut right_lines_iter = right.lines();

    for line in left.lines() {
        let Some(expected_line) = right_lines_iter.next() else {
            panic!(
                "assert_eq_lines_trim_whitespace: Left string has more lines than right string"
            );
        };
        assert_eq!(line.trim(), expected_line.trim());
    }
    if right_lines_iter.next().is_some() {
        panic!("assert_eq_lines_trim_whitespace: Left string has less lines than right string");
    }
}
