use super::ScriptError;

pub(super) const MAX_STACK_SIZE: usize = 1_000;

#[derive(Default)]
pub(super) struct ConditionStack(Vec<bool>);

impl ConditionStack {
    pub(super) fn all_true(&self) -> bool {
        self.0.iter().all(|value| *value)
    }

    pub(super) fn push(&mut self, value: bool) {
        self.0.push(value);
    }

    pub(super) fn pop(&mut self) -> Option<bool> {
        self.0.pop()
    }

    pub(super) fn toggle_top(&mut self) -> Result<(), ScriptError> {
        let Some(top) = self.0.last_mut() else {
            return Err(ScriptError::UnbalancedConditional);
        };
        *top = !*top;
        Ok(())
    }

    pub(super) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(super) fn outer_all_true(&self) -> bool {
        self.0
            .get(..self.0.len().saturating_sub(1))
            .is_none_or(|values| values.iter().all(|value| *value))
    }
}

pub(super) fn push_stack(stack: &mut Vec<Vec<u8>>, value: Vec<u8>) -> Result<(), ScriptError> {
    stack.push(value);
    if stack.len() > MAX_STACK_SIZE {
        return Err(ScriptError::StackOverflow(stack.len()));
    }
    Ok(())
}

pub(super) fn pop_bytes(stack: &mut Vec<Vec<u8>>) -> Result<Vec<u8>, ScriptError> {
    stack.pop().ok_or(ScriptError::InvalidStackOperation)
}

pub(super) fn pop_num(stack: &mut Vec<Vec<u8>>) -> Result<i64, ScriptError> {
    let value = pop_bytes(stack)?;
    decode_script_num(&value)
}

pub(super) fn unary_num_op(
    stack: &mut Vec<Vec<u8>>,
    operation: impl FnOnce(i64) -> i64,
) -> Result<(), ScriptError> {
    let value = pop_num(stack)?;
    push_stack(stack, encode_script_num(operation(value)))
}

pub(super) fn binary_num_op(
    stack: &mut Vec<Vec<u8>>,
    operation: impl FnOnce(i64, i64) -> i64,
) -> Result<(), ScriptError> {
    let right = pop_num(stack)?;
    let left = pop_num(stack)?;
    push_stack(stack, encode_script_num(operation(left, right)))
}

pub(super) fn script_booland(left: i64, right: i64) -> i64 {
    if left != 0 && right != 0 { 1 } else { 0 }
}

pub(super) fn script_boolor(left: i64, right: i64) -> i64 {
    if left != 0 || right != 0 { 1 } else { 0 }
}

pub(super) fn encode_bool(value: bool) -> Vec<u8> {
    if value { vec![1_u8] } else { Vec::new() }
}

pub(super) fn cast_to_bool(value: &[u8]) -> bool {
    for (index, byte) in value.iter().enumerate() {
        if *byte == 0 {
            continue;
        }
        if index == value.len() - 1 && *byte == 0x80 {
            return false;
        }
        return true;
    }
    false
}

pub(super) fn decode_small_num(bytes: &[u8]) -> Result<usize, ScriptError> {
    let value = decode_script_num(bytes)?;
    if value < 0 {
        return Err(ScriptError::InvalidStackOperation);
    }
    Ok(value as usize)
}

pub(super) fn decode_script_num(bytes: &[u8]) -> Result<i64, ScriptError> {
    if bytes.len() > 4 {
        return Err(ScriptError::NumOverflow(bytes.len()));
    }
    if bytes.is_empty() {
        return Ok(0);
    }

    let mut value = 0_i64;
    for (index, byte) in bytes.iter().enumerate() {
        value |= i64::from(*byte) << (8 * index);
    }

    let last = bytes[bytes.len() - 1];
    if (last & 0x80) != 0 {
        let mask = !(0x80_i64 << (8 * (bytes.len() - 1)));
        Ok(-(value & mask))
    } else {
        Ok(value)
    }
}

pub(super) fn encode_script_num(value: i64) -> Vec<u8> {
    if value == 0 {
        return Vec::new();
    }

    let negative = value < 0;
    let mut magnitude = value.unsigned_abs();
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    if encoded.last().is_some_and(|byte| (byte & 0x80) != 0) {
        encoded.push(if negative { 0x80 } else { 0x00 });
    } else if negative && let Some(last) = encoded.last_mut() {
        *last |= 0x80;
    }

    encoded
}
