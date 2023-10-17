mod common;

#[cfg(test)]
mod test {
    use super::common::TestCase;

    #[test]
    fn parser_success() {
        println!("Running parser success tests:");

        let mut success = 0;
        let mut failed = 0;

        for test_case in TestCase::list("tests/parser/success") {
            if test_case.run() {
                success += 1;
            } else {
                failed += 1;
            }
        }

        println!("-----");
        println!("{}/{} tests passed\n", success, success + failed);
        assert_eq!(0, failed, "one or more tests failed");
    }

    #[test]
    fn parser_error() {
        println!("Running parser error tests:");

        let mut success = 0;
        let mut failed = 0;
        for test_case in TestCase::list("tests/parser/error") {
            if test_case.run() {
                failed += 1;
            } else {
                success += 1;
            }
        }

        println!("{}/{} tests passed", success, success + failed);
        println!("-----");
        assert_eq!(0, failed, "one or more tests failed");
    }
}
