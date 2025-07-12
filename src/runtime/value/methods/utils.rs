use crate::runtime::value::RuntimeException;
use crate::runtime::value::exception;

pub fn arg_check(
    arg_count: usize,
    expected_count: usize,
    name: &str,
) -> Result<(), RuntimeException> {
    (arg_count == expected_count)
        .then(|| ())
        .ok_or(exception::ARGUMENT.runtime(format!(
            "{} takes {} argument ({} given)",
            name, expected_count, arg_count
        )))
}
