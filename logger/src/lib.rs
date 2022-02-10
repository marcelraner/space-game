#[macro_export]
macro_rules! timestamp {
    () => {
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros()
    };
}

#[macro_export]
macro_rules! trace {
    ($fmt:expr) => (println!("{} TRACE: {}", logger::timestamp!(), $fmt));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("{} TRACE: ", $fmt), logger::timestamp!(), $($arg)*));
}

#[macro_export]
macro_rules! debug {
    ($fmt:expr) => (println!("{} DEBUG: {}", logger::timestamp!(), $fmt));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("{} DEBUG: ", $fmt), logger::timestamp!(), $($arg)*));
}

#[macro_export]
macro_rules! info {
    ($fmt:expr) => (println!("{} INFO: {}", logger::timestamp!(), $fmt));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("{} INFO: ", $fmt), logger::timestamp!(), $($arg)*));
}

#[macro_export]
macro_rules! warn {
    ($fmt:expr) => (println!("{} WARN: {}", logger::timestamp!(), $fmt));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("{} WARN: ", $fmt), logger::timestamp!(), $($arg)*));
}

#[macro_export]
macro_rules! error {
    ($fmt:expr) => (println!("{} ERROR: {}", logger::timestamp!(), $fmt));
    ($fmt:expr, $($arg:tt)*) => (println!(concat!("{} ERROR: ", $fmt), logger::timestamp!(), $($arg)*));
}
