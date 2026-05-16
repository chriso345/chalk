#[cfg(debug_assertions)]
#[macro_export]
macro_rules! icon {
    ($path:literal) => {
        concat!("/public/icons/", $path)
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! icon {
    ($path:literal) => {
        concat!("/chalk/public/icons/", $path)
    };
}
