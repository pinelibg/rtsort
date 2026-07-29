use assert_cmd::Command;
use predicates::prelude::*;
use std::fmt::Write as _;

fn cmd() -> Command {
    Command::cargo_bin("rtsort").unwrap()
}

/// Asserts that running rtsort with `args` on `input` succeeds and prints
/// exactly `expected` on stdout.
fn assert_sorts(args: &[&str], input: &str, expected: &str) {
    cmd()
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::diff(expected.to_string()));
}

/// Asserts that running rtsort with `args` on `input` exits with a failure.
fn assert_fails(args: &[&str], input: &str) {
    cmd().args(args).write_stdin(input).assert().failure();
}

mod normal_sort {
    use super::*;

    #[test]
    fn basic_alphabetical() {
        assert_sorts(&[], "banana\napple\ncherry\n", "apple\nbanana\ncherry\n");
    }

    #[test]
    fn already_sorted() {
        assert_sorts(&[], "alpha\nbeta\ngamma\n", "alpha\nbeta\ngamma\n");
    }

    #[test]
    fn numbers_as_strings() {
        assert_sorts(&[], "20\n2\n10\n3\n", "10\n2\n20\n3\n");
    }

    #[test]
    fn mixed_case() {
        assert_sorts(&[], "banana\nApple\nCherry\n", "Apple\nCherry\nbanana\n");
    }

    #[test]
    fn duplicates_preserved() {
        assert_sorts(&[], "b\na\nb\na\n", "a\na\nb\nb\n");
    }
}

mod reverse_sort {
    use super::*;

    #[test]
    fn basic_reverse() {
        assert_sorts(
            &["-r"],
            "apple\ncherry\nbanana\n",
            "cherry\nbanana\napple\n",
        );
    }

    #[test]
    fn long_flag() {
        assert_sorts(&["--reverse"], "a\nc\nb\n", "c\nb\na\n");
    }

    #[test]
    fn combined_with_human_numeric() {
        assert_sorts(&["-h", "-r"], "1K\n1G\n1M\n", "1G\n1M\n1K\n");
    }
}

mod numeric_sort {
    use super::*;

    #[test]
    fn basic_numeric_order() {
        assert_sorts(&["-n"], "20\n2\n10\n3\n", "2\n3\n10\n20\n");
    }

    #[test]
    fn long_flag() {
        assert_sorts(&["--numeric-sort"], "20\n2\n10\n", "2\n10\n20\n");
    }

    #[test]
    fn non_numeric_before_numeric() {
        assert_sorts(&["-n"], "10\nfoo\n2\nbar\n", "bar\nfoo\n2\n10\n");
    }

    #[test]
    fn suffix_ignored() {
        assert_sorts(&["-n"], "10K\n2M\n5G\n", "2M\n5G\n10K\n");
    }

    #[test]
    fn negative_values() {
        assert_sorts(&["-n"], "3\n-1\n2\n", "-1\n2\n3\n");
    }

    #[test]
    fn human_numeric_and_numeric_are_mutually_exclusive() {
        assert_fails(&["-n", "-h"], "1G\n1K\n1M\n");
    }
}

mod human_numeric_sort {
    use super::*;

    #[test]
    fn basic_size_suffixes() {
        assert_sorts(&["-h"], "1G\n1M\n1K\n", "1K\n1M\n1G\n");
    }

    #[test]
    fn same_suffix_different_magnitude() {
        assert_sorts(&["-h"], "10K\n2K\n1K\n", "1K\n2K\n10K\n");
    }

    #[test]
    fn fractional_values() {
        assert_sorts(&["-h"], "1.5M\n1M\n1023K\n", "1023K\n1M\n1.5M\n");
    }

    #[test]
    fn cross_suffix_boundary() {
        assert_sorts(&["-h"], "1025K\n1M\n", "1M\n1025K\n");
    }

    #[test]
    fn non_numeric_before_numeric() {
        assert_sorts(&["-h"], "1K\nfoo\n2K\nbar\n", "bar\nfoo\n1K\n2K\n");
    }

    #[test]
    fn du_style_output() {
        assert_sorts(
            &["-h"],
            "4.0K\t/boot\n0\t/dev\n528K\t/tmp\n12K\t/mnt\n",
            "0\t/dev\n4.0K\t/boot\n12K\t/mnt\n528K\t/tmp\n",
        );
    }

    #[test]
    fn iec_suffixes() {
        assert_sorts(&["-h"], "1GiB\n1MiB\n1KiB\n", "1KiB\n1MiB\n1GiB\n");
    }

    #[test]
    fn negative_values() {
        assert_sorts(&["-h"], "-1G\n-1K\n-1M\n", "-1G\n-1M\n-1K\n");
    }

    #[test]
    fn long_flag() {
        assert_sorts(&["--human-numeric-sort"], "1G\n1K\n1M\n", "1K\n1M\n1G\n");
    }
}

mod edge_cases {
    use super::*;

    #[test]
    fn empty_input() {
        assert_sorts(&[], "", "");
    }

    #[test]
    fn single_line() {
        assert_sorts(&[], "hello\n", "hello\n");
    }

    #[test]
    fn single_line_no_trailing_newline() {
        assert_sorts(&[], "hello", "hello\n");
    }

    #[test]
    fn empty_lines_sort_first() {
        assert_sorts(&[], "b\n\na\n\n", "\n\na\nb\n");
    }

    #[test]
    fn large_input() {
        let input = (1..=1000).rev().fold(String::new(), |mut s, i| {
            let _ = writeln!(s, "{i}");
            s
        });
        let mut expected: Vec<String> = (1..=1000).map(|i| i.to_string()).collect();
        expected.sort();
        let expected = expected.join("\n") + "\n";

        assert_sorts(&[], &input, &expected);
    }
}

mod ignore_case_sort {
    use super::*;

    #[test]
    fn basic_ignore_case() {
        assert_sorts(
            &["-f"],
            "banana\nApple\nCherry\n",
            "Apple\nbanana\nCherry\n",
        );
    }

    #[test]
    fn long_flag() {
        assert_sorts(
            &["--ignore-case"],
            "banana\nApple\nCherry\n",
            "Apple\nbanana\nCherry\n",
        );
    }

    #[test]
    fn combined_with_reverse() {
        assert_sorts(
            &["-f", "-r"],
            "banana\nApple\nCherry\n",
            "Cherry\nbanana\nApple\n",
        );
    }

    #[test]
    fn tiebreak_uppercase_before_lowercase() {
        // When case-insensitively equal, byte order decides: 'A' (65) < 'a' (97)
        assert_sorts(&["-f"], "apple\nApple\n", "Apple\napple\n");
    }

    #[test]
    fn unicode_uppercase_fold() {
        // "ß".to_uppercase() == "SS" (0x53 0x53) which sorts before "T" (0x54)
        assert_sorts(&["-f"], "t\nß\n", "ß\nt\n");
    }
}

mod version_sort {
    use super::*;

    #[test]
    fn basic_version_order() {
        assert_sorts(
            &["-V"],
            "v1.10\nv1.9\nv2.0\nv1.0\n",
            "v1.0\nv1.9\nv1.10\nv2.0\n",
        );
    }

    #[test]
    fn long_flag() {
        assert_sorts(
            &["--version-sort"],
            "v1.10\nv1.9\nv2.0\nv1.0\n",
            "v1.0\nv1.9\nv1.10\nv2.0\n",
        );
    }
}

mod sort_mode_conflicts {
    use super::*;

    #[test]
    fn numeric_and_version_conflicts() {
        assert_fails(&["-n", "-V"], "1\n2\n");
    }

    #[test]
    fn human_numeric_and_numeric_conflicts() {
        assert_fails(&["-h", "-n"], "1\n2\n");
    }
}

mod help {
    use super::*;

    #[test]
    fn help_flag_exits_successfully() {
        cmd().arg("--help").assert().success();
    }
}

mod fps_validation {
    use super::*;

    #[test]
    fn negative_fps_is_rejected() {
        assert_fails(&["--fps", "-1"], "a\n");
    }

    #[test]
    fn non_numeric_fps_is_rejected() {
        assert_fails(&["--fps", "abc"], "a\n");
    }

    #[test]
    fn nan_fps_is_rejected() {
        assert_fails(&["--fps", "NaN"], "a\n");
    }

    #[test]
    fn zero_fps_is_accepted() {
        assert_sorts(&["--fps", "0"], "b\na\n", "a\nb\n");
    }
}

mod line_endings {
    use super::*;

    #[test]
    fn crlf_stripped() {
        assert_sorts(
            &[],
            "banana\r\napple\r\ncherry\r\n",
            "apple\nbanana\ncherry\n",
        );
    }

    #[test]
    fn mixed_line_endings() {
        assert_sorts(&[], "c\r\nb\na\r\n", "a\nb\nc\n");
    }

    #[test]
    fn crlf_with_human_numeric() {
        assert_sorts(&["-h"], "1M\r\n1K\r\n1G\r\n", "1K\n1M\n1G\n");
    }
}

mod top_output {
    use super::*;

    #[test]
    fn basic_top() {
        assert_sorts(
            &["--top", "3"],
            "banana\napple\ncherry\ndate\nelderberry\n",
            "apple\nbanana\ncherry\n",
        );
    }

    #[test]
    fn field_sep_without_key_is_rejected() {
        // -t requires -k; providing -t alone should cause a CLI error
        assert_fails(&["-t", "3"], "banana\napple\ncherry\n");
    }

    #[test]
    fn with_reverse() {
        assert_sorts(
            &["-r", "--top", "3"],
            "banana\napple\ncherry\ndate\nelderberry\n",
            "elderberry\ndate\ncherry\n",
        );
    }

    #[test]
    fn with_numeric_sort() {
        assert_sorts(&["-n", "--top", "3"], "10\n2\n30\n5\n20\n", "2\n5\n10\n");
    }

    #[test]
    fn with_human_numeric_sort() {
        assert_sorts(&["-h", "--top", "2"], "1G\n1K\n1M\n", "1K\n1M\n");
    }

    #[test]
    fn n_greater_than_total() {
        assert_sorts(
            &["--top", "10"],
            "cherry\napple\nbanana\n",
            "apple\nbanana\ncherry\n",
        );
    }

    #[test]
    fn n_zero() {
        assert_sorts(&["--top", "0"], "banana\napple\ncherry\n", "");
    }
}

mod no_preview {
    use super::*;

    #[test]
    fn sorts_correctly_without_preview() {
        assert_sorts(
            &["--no-preview"],
            "banana\napple\ncherry\n",
            "apple\nbanana\ncherry\n",
        );
    }

    #[test]
    fn combined_with_reverse() {
        assert_sorts(
            &["--no-preview", "-r"],
            "banana\napple\ncherry\n",
            "cherry\nbanana\napple\n",
        );
    }

    #[test]
    fn combined_with_top() {
        assert_sorts(
            &["--no-preview", "--top", "2"],
            "banana\napple\ncherry\n",
            "apple\nbanana\n",
        );
    }
}

mod key_sort {
    use super::*;

    #[test]
    fn sort_by_second_field_whitespace() {
        assert_sorts(
            &["-k", "2"],
            "banana 2\napple 3\ncherry 1\n",
            "cherry 1\nbanana 2\napple 3\n",
        );
    }

    #[test]
    fn sort_by_second_field_with_separator() {
        assert_sorts(
            &["-k", "2", "-t", ":"],
            "banana:2\napple:3\ncherry:1\n",
            "cherry:1\nbanana:2\napple:3\n",
        );
    }

    #[test]
    fn missing_field_sorts_as_empty_string() {
        // Lines with fewer fields than N use empty string as key (sorts first)
        assert_sorts(
            &["-k", "2"],
            "banana 2\napple\ncherry 1\n",
            "apple\ncherry 1\nbanana 2\n",
        );
    }

    #[test]
    fn long_flags() {
        assert_sorts(
            &["--key", "2", "--field-separator", "|"],
            "b|z\na|a\nc|m\n",
            "a|a\nc|m\nb|z\n",
        );
    }

    #[test]
    fn key_zero_is_rejected() {
        assert_fails(&["-k", "0"], "a\nb\n");
    }
}

mod unique_sort {
    use super::*;

    #[test]
    fn removes_duplicates() {
        assert_sorts(&["-u"], "b\na\nb\na\n", "a\nb\n");
    }

    #[test]
    fn top_with_interleaved_duplicates() {
        // Without correct fix, dedup after windowed top-2 could yield fewer than 2 unique lines.
        // Input sorted: a a b b c c — top 2 unique should be a, b.
        assert_sorts(&["-u", "--top", "2"], "b\na\nb\nc\na\nc\n", "a\nb\n");
    }

    #[test]
    fn bottom_with_interleaved_duplicates() {
        // Input sorted: a a b b c c — bottom 2 unique should be b, c.
        assert_sorts(&["-u", "--bottom", "2"], "b\na\nb\nc\na\nc\n", "b\nc\n");
    }
}

mod bottom_output {
    use super::*;

    #[test]
    fn basic_bottom() {
        assert_sorts(
            &["--bottom", "3"],
            "banana\napple\ncherry\ndate\nelderberry\n",
            "cherry\ndate\nelderberry\n",
        );
    }

    #[test]
    fn with_reverse() {
        assert_sorts(
            &["-r", "--bottom", "3"],
            "banana\napple\ncherry\ndate\nelderberry\n",
            "cherry\nbanana\napple\n",
        );
    }

    #[test]
    fn with_numeric_sort() {
        assert_sorts(
            &["-n", "--bottom", "3"],
            "10\n2\n30\n5\n20\n",
            "10\n20\n30\n",
        );
    }

    #[test]
    fn with_human_numeric_sort() {
        assert_sorts(&["-h", "--bottom", "2"], "1G\n1K\n1M\n", "1M\n1G\n");
    }

    #[test]
    fn n_greater_than_total() {
        assert_sorts(
            &["--bottom", "10"],
            "cherry\napple\nbanana\n",
            "apple\nbanana\ncherry\n",
        );
    }

    #[test]
    fn n_zero() {
        assert_sorts(&["--bottom", "0"], "banana\napple\ncherry\n", "");
    }

    #[test]
    fn conflicts_with_top() {
        assert_fails(&["--top", "2", "--bottom", "2"], "banana\napple\n");
    }
}
