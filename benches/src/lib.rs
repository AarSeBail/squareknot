// Thank you bevy! <3
#[macro_export]
macro_rules! bench {
    ($name:expr) => {
        format!("{}{}{}", module_path!(), "::", $name)
    };
}
