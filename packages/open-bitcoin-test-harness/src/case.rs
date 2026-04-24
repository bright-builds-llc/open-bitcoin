use serde_json::Value;

use crate::target::HarnessTarget;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionalCase {
    pub name: &'static str,
    pub method: &'static str,
    pub params: Value,
    pub expected: ExpectedOutcome,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpectedOutcome {
    ResultHasKeys(Vec<&'static str>),
    ErrorCode(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CaseOutcome {
    pub case_name: String,
    pub passed: bool,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SuiteReport {
    pub suite: String,
    pub target: String,
    pub skipped: bool,
    pub skip_reason: Option<String>,
    pub outcomes: Vec<CaseOutcome>,
}

impl SuiteReport {
    pub fn passed(&self) -> bool {
        !self.skipped && self.outcomes.iter().all(|outcome| outcome.passed)
    }

    pub fn passed_count(&self) -> usize {
        self.outcomes
            .iter()
            .filter(|outcome| outcome.passed)
            .count()
    }
}

pub fn run_suite<T: HarnessTarget>(
    suite: &str,
    target: &mut T,
    cases: &[FunctionalCase],
) -> SuiteReport {
    let outcomes = cases
        .iter()
        .map(|case| run_case(target, case))
        .collect::<Vec<_>>();
    SuiteReport {
        suite: suite.to_string(),
        target: target.name().to_string(),
        skipped: false,
        skip_reason: None,
        outcomes,
    }
}

pub fn skipped_suite(suite: &str, target: &str, reason: impl Into<String>) -> SuiteReport {
    SuiteReport {
        suite: suite.to_string(),
        target: target.to_string(),
        skipped: true,
        skip_reason: Some(reason.into()),
        outcomes: Vec::new(),
    }
}

fn run_case<T: HarnessTarget>(target: &mut T, case: &FunctionalCase) -> CaseOutcome {
    match target.request(case.method, case.params.clone()) {
        Ok(response) => evaluate_response(case, &response),
        Err(error) => CaseOutcome {
            case_name: case.name.to_string(),
            passed: false,
            detail: error.to_string(),
        },
    }
}

fn evaluate_response(case: &FunctionalCase, response: &Value) -> CaseOutcome {
    match &case.expected {
        ExpectedOutcome::ResultHasKeys(keys) => result_has_keys(case.name, response, keys),
        ExpectedOutcome::ErrorCode(expected_code) => {
            error_has_code(case.name, response, *expected_code)
        }
    }
}

fn result_has_keys(case_name: &str, response: &Value, keys: &[&str]) -> CaseOutcome {
    let Some(result) = response.get("result") else {
        return failed(case_name, "missing result field");
    };
    let Some(object) = result.as_object() else {
        return failed(case_name, "result field is not an object");
    };
    let missing = keys
        .iter()
        .filter(|key| !object.contains_key(**key))
        .copied()
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return failed(
            case_name,
            format!("missing result keys: {}", missing.join(", ")),
        );
    }
    passed(
        case_name,
        format!("result contains keys: {}", keys.join(", ")),
    )
}

fn error_has_code(case_name: &str, response: &Value, expected_code: i64) -> CaseOutcome {
    let Some(code) = response
        .get("error")
        .and_then(|error| error.get("code"))
        .and_then(Value::as_i64)
    else {
        return failed(case_name, "missing error.code field");
    };
    if code != expected_code {
        return failed(
            case_name,
            format!("expected error code {expected_code}, got {code}"),
        );
    }
    passed(case_name, format!("error code matched {expected_code}"))
}

fn passed(case_name: &str, detail: impl Into<String>) -> CaseOutcome {
    CaseOutcome {
        case_name: case_name.to_string(),
        passed: true,
        detail: detail.into(),
    }
}

fn failed(case_name: &str, detail: impl Into<String>) -> CaseOutcome {
    CaseOutcome {
        case_name: case_name.to_string(),
        passed: false,
        detail: detail.into(),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{ExpectedOutcome, FunctionalCase, run_suite};
    use crate::{HarnessError, HarnessTarget};

    struct StaticTarget {
        response: serde_json::Value,
    }

    impl HarnessTarget for StaticTarget {
        fn name(&self) -> &str {
            "static"
        }

        fn request(
            &mut self,
            _method: &str,
            _params: serde_json::Value,
        ) -> Result<serde_json::Value, HarnessError> {
            Ok(self.response.clone())
        }
    }

    #[test]
    fn suite_passes_when_expected_result_keys_exist() {
        // Arrange
        let mut target = StaticTarget {
            response: json!({"result": {"chain": "regtest", "blocks": 0}, "id": 1}),
        };
        let cases = [FunctionalCase {
            name: "chain info shape",
            method: "getblockchaininfo",
            params: json!([]),
            expected: ExpectedOutcome::ResultHasKeys(vec!["chain", "blocks"]),
        }];

        // Act
        let report = run_suite("rpc", &mut target, &cases);

        // Assert
        assert!(report.passed());
        assert_eq!(report.passed_count(), 1);
    }
}
