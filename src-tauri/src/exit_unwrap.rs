//! Изменённый 'unwrap()', чтобы программа завершела работу при ошибки в 'async fn'

pub trait ExitUnwrap<T> {
    fn exit_unwrap(self) -> T;
}

impl<T, E: std::fmt::Debug> ExitUnwrap<T> for Result<T, E> {
    fn exit_unwrap(self) -> T {
        match self {
            Ok(o) => o,
            Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", &e),
        }
    }
}

fn unwrap_failed(msg: &str, error: &dyn std::fmt::Debug) -> ! {
    log::error!("{msg}: {error:?}");
    std::process::exit(1)
}
