use crate::runtime::exceptions::RuntimeError;

pub fn arg_check(arg_count: usize, expected_count: usize, name: &str) -> Result<(), RuntimeError> {
    (arg_count == expected_count)
        .then(|| ())
        .ok_or(RuntimeError::ArgumentCount(format!(
            "{} takes {} argument ({} given)",
            name, expected_count, arg_count
        )))
}
