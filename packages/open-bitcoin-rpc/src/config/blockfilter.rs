// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/settings.cpp

use super::ConfigError;

/// Default-off BASIC selection, retaining whether the operator specified an option.
/// Supported operator input is CLI or bitcoin.conf; JSONC has no index field.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BasicFilterIndexSetting {
    #[default]
    Unspecified,
    Disabled,
    Basic,
}

impl BasicFilterIndexSetting {
    /// Whether BASIC indexing was selected.
    pub const fn is_enabled(self) -> bool {
        matches!(self, Self::Basic)
    }

    /// Whether durable startup should inspect an explicit enable or disable request.
    pub const fn is_explicit(self) -> bool {
        !matches!(self, Self::Unspecified)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FilterOptionValue {
    String(String),
    Negation(bool),
}

pub(super) fn resolve_filter_index(
    cli: &[FilterOptionValue],
    network: &[FilterOptionValue],
    defaults: &[FilterOptionValue],
) -> Result<BasicFilterIndexSetting, ConfigError> {
    let sources = [cli, network, defaults];
    let Some((source, values)) = sources
        .iter()
        .enumerate()
        .find(|(_, values)| !values.is_empty())
    else {
        return Ok(BasicFilterIndexSetting::Unspecified);
    };
    let remaining = after_last_negation(values);
    let maybe_scalar = if source == 0 {
        remaining.last()
    } else {
        remaining.first()
    };
    let scalar = maybe_scalar.map_or("0", FilterOptionValue::as_str);
    match scalar {
        "" | "1" => return Ok(BasicFilterIndexSetting::Basic),
        "0" => return Ok(BasicFilterIndexSetting::Disabled),
        _ => {}
    }

    let mut done = false;
    let mut negated_empty = false;
    let mut has_names = false;
    for (source, values) in sources.into_iter().enumerate() {
        let remaining = after_last_negation(values);
        // Knots revives lower-priority config unless negation left the merged list empty.
        if !done || (source != 0 && !negated_empty) {
            for value in remaining {
                match value.as_str() {
                    "basic" => has_names = true,
                    "v0" | "2" => {
                        return Err(ConfigError::new(
                            "-blockfilterindex v0/type 2 is excluded from BASIC-only scope",
                        ));
                    }
                    _ => {
                        return Err(ConfigError::new(
                            "unknown -blockfilterindex value: named mode requires basic; 0, 1 or empty select scalar mode only",
                        ));
                    }
                }
            }
        }
        done |= remaining.len() != values.len();
        negated_empty |= values.last() == Some(&FilterOptionValue::Negation(false)) && !has_names;
    }
    Ok(BasicFilterIndexSetting::Basic)
}

impl FilterOptionValue {
    fn as_str(&self) -> &str {
        match self {
            Self::String(value) => value,
            Self::Negation(false) => "0",
            Self::Negation(true) => "1",
        }
    }
}

fn after_last_negation(values: &[FilterOptionValue]) -> &[FilterOptionValue] {
    let start = values
        .iter()
        .rposition(|value| *value == FilterOptionValue::Negation(false))
        .map_or(0, |index| index + 1);
    &values[start..]
}
