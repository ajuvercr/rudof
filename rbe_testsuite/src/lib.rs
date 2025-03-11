pub mod match_result;
pub mod rbe_test;
pub mod rbe_test_result;
pub mod rbe_test_results;
pub mod rbe_tests;

pub use match_result::*;
pub use rbe_test::*;
pub use rbe_test_result::*;
pub use rbe_test_results::*;
pub use rbe_tests::*;

/// TODO: I would prefer this type to be a String or &str, but it must be Eq, Hash, Clone and with &str I have some lifetime issues...
type TestType = String;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rbe::{rbe::Rbe, Bag, Max};

    #[test]
    fn basic_test() {
        let str = indoc! {
        r#"name: basic
             rbe: !Symbol
              value: foo
              card:
                min: 1
                max: !IntMax 1
             bag:
               - - foo
                 - 1
             open: false
             match_result: !Pass
            "#};
        let rbe_test: RbeTest = serde_yaml_ng::from_str(str).unwrap();
        assert_eq!(rbe_test.run(), RbeTestResult::passed("basic".to_string()))
    }

    // The following test is useful to generate YAML serializations but is not a proper test
    #[test]
    fn check_serialization() {
        let values = vec![
            Rbe::symbol("a".to_string(), 1, Max::IntMax(1)),
            Rbe::symbol("b".to_string(), 2, Max::IntMax(3)),
        ];
        let mut rbe_test = RbeTest::default();
        rbe_test.set_group("test".to_string());
        rbe_test.set_name("basic".to_string());
        rbe_test.set_full_name("test/basic".to_string());
        rbe_test.set_rbe(Rbe::and(values));
        rbe_test.set_bag(Bag::from(["a".to_string(), "b".to_string()]));
        let ts = vec![rbe_test];
        let mut rbe_tests = RbeTests::default();
        rbe_tests.with_tests(ts);
        let serialized = serde_yaml_ng::to_string(&rbe_tests).unwrap();
        println!("---\n{serialized}");
        assert!(!serialized.is_empty());
    }

    #[test]
    fn load_slice_1() {
        let str = indoc! {r#"
        tests:
        - name: basic
          rbe: !Symbol
            value: foo
            card:
              min: 1
              max: !IntMax 1
          bag:
          - - foo
            - 1
          open: false
          match_result: !Pass
        "#};
        let mut tests = RbeTests::new();
        tests.load_slice("test", str.as_bytes()).unwrap();
        let t0 = &tests.tests().next().unwrap();
        assert_eq!("test", t0.group());
        assert_eq!(RbeTestResult::passed("basic".to_string()), t0.run());
    }

    // Runs all the tests
    #[test]
    fn run_test_suite() {
        let data = include_bytes!("../tests/basic.yaml");
        let mut rbe_tests = RbeTests::new();
        rbe_tests.load_slice("basic", data).unwrap();
        let results = rbe_tests.run();
        for t in results.failed() {
            tracing::info!("Failed: {}: error: {}", t.name(), t.err());
        }
        assert_eq!(results.count_passed(), rbe_tests.total());
        assert_eq!(results.count_failed(), 0);
    }

    // The following test can be use to check a single test case
    #[test]
    fn run_single() {
        let name = "a_1_1_with_a_2_fail".to_string();
        println!("Running single test: {name}");
        let data = include_bytes!("../tests/basic.yaml");
        let mut rbe_tests = RbeTests::new();
        rbe_tests.load_slice("basic", data).unwrap();
        let results = rbe_tests.run_by_name(name);
        for t in results.failed() {
            println!("Failed: {}: error: {}", t.name(), t.err());
        }
        assert_eq!(results.count_passed(), 1);
        assert_eq!(results.count_failed(), 0);
    }
}
