use std::fmt::Display;

pub trait OptionExt<T> {
    fn as_format_arg(&self) -> &dyn Display;
}

impl<T: Display> OptionExt<T> for Option<T> {
    fn as_format_arg(&self) -> &dyn Display {
        self.as_ref().map(|x| -> &dyn Display { x }).unwrap_or(&"")
    }
}
