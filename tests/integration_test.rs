use assert_cmd::Command;
use predicates::prelude::*;
use std::fmt::Write as _;

fn cmd() -> Command {
    Command::cargo_bin("rtsort").unwrap()
}

mod normal_sort {
    use super::*;

    #[test]
    fn basic_alphabetical() {
        cmd()
            .write_stdin("banana\napple\ncherry\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("apple\nbanana\ncherry\n"));
    }

    #[test]
    fn already_sorted() {
        cmd()
            .write_stdin("alpha\nbeta\ngamma\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("alpha\nbeta\ngamma\n"));
    }

    #[test]
    fn numbers_as_strings() {
        cmd()
            .write_stdin("20\n2\n10\n3\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("10\n2\n20\n3\n"));
    }

    #[test]
    fn mixed_case() {
        cmd()
            .write_stdin("banana\nApple\nCherry\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("Apple\nCherry\nbanana\n"));
    }

    #[test]
    fn duplicates_preserved() {
        cmd()
            .write_stdin("b\na\nb\na\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("a\na\nb\nb\n"));
    }
}

mod reverse_sort {
    use super::*;

    #[test]
    fn basic_reverse() {
        cmd()
            .arg("-r")
            .write_stdin("apple\ncherry\nbanana\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("cherry\nbanana\napple\n"));
    }

    #[test]
    fn long_flag() {
        cmd()
            .arg("--reverse")
            .write_stdin("a\nc\nb\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("c\nb\na\n"));
    }

    #[test]
    fn combined_with_human_numeric() {
        cmd()
            .args(["-h", "-r"])
            .write_stdin("1K\n1G\n1M\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("1G\n1M\n1K\n"));
    }
}

mod human_numeric_sort {
    use super::*;

    #[test]
    fn basic_size_suffixes() {
        cmd()
            .arg("-h")
            .write_stdin("1G\n1M\n1K\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("1K\n1M\n1G\n"));
    }

    #[test]
    fn same_suffix_different_magnitude() {
        cmd()
            .arg("-h")
            .write_stdin("10K\n2K\n1K\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("1K\n2K\n10K\n"));
    }

    #[test]
    fn fractional_values() {
        cmd()
            .arg("-h")
            .write_stdin("1.5M\n1M\n1023K\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("1023K\n1M\n1.5M\n"));
    }

    #[test]
    fn cross_suffix_boundary() {
        cmd()
            .arg("-h")
            .write_stdin("1025K\n1M\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("1M\n1025K\n"));
    }

    #[test]
    fn non_numeric_before_numeric() {
        cmd()
            .arg("-h")
            .write_stdin("1K\nfoo\n2K\nbar\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("bar\nfoo\n1K\n2K\n"));
    }

    #[test]
    fn du_style_output() {
        cmd()
            .arg("-h")
            .write_stdin("4.0K\t/boot\n0\t/dev\n528K\t/tmp\n12K\t/mnt\n")
            .assert()
            .success()
            .stdout(predicate::str::diff(
                "0\t/dev\n4.0K\t/boot\n12K\t/mnt\n528K\t/tmp\n",
            ));
    }

    #[test]
    fn iec_suffixes() {
        cmd()
            .arg("-h")
            .write_stdin("1GiB\n1MiB\n1KiB\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("1KiB\n1MiB\n1GiB\n"));
    }

    #[test]
    fn negative_values() {
        cmd()
            .arg("-h")
            .write_stdin("-1G\n-1K\n-1M\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("-1G\n-1M\n-1K\n"));
    }

    #[test]
    fn long_flag() {
        cmd()
            .arg("--human-numeric-sort")
            .write_stdin("1G\n1K\n1M\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("1K\n1M\n1G\n"));
    }
}

mod edge_cases {
    use super::*;

    #[test]
    fn empty_input() {
        cmd()
            .write_stdin("")
            .assert()
            .success()
            .stdout(predicate::str::diff(""));
    }

    #[test]
    fn single_line() {
        cmd()
            .write_stdin("hello\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("hello\n"));
    }

    #[test]
    fn single_line_no_trailing_newline() {
        cmd()
            .write_stdin("hello")
            .assert()
            .success()
            .stdout(predicate::str::diff("hello\n"));
    }

    #[test]
    fn empty_lines_sort_first() {
        cmd()
            .write_stdin("b\n\na\n\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("\n\na\nb\n"));
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

        cmd()
            .write_stdin(input)
            .assert()
            .success()
            .stdout(predicate::str::diff(expected));
    }
}

mod help {
    use super::*;

    #[test]
    fn help_flag_exits_successfully() {
        cmd().arg("--help").assert().success();
    }
}

mod line_endings {
    use super::*;

    #[test]
    fn crlf_stripped() {
        cmd()
            .write_stdin("banana\r\napple\r\ncherry\r\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("apple\nbanana\ncherry\n"));
    }

    #[test]
    fn mixed_line_endings() {
        cmd()
            .write_stdin("c\r\nb\na\r\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("a\nb\nc\n"));
    }

    #[test]
    fn crlf_with_human_numeric() {
        cmd()
            .arg("-h")
            .write_stdin("1M\r\n1K\r\n1G\r\n")
            .assert()
            .success()
            .stdout(predicate::str::diff("1K\n1M\n1G\n"));
    }
}
