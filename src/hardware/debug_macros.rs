macro_rules! debug_system {
    ($formatted_str:expr, $debug_mode:expr) => {
        if $debug_mode {
            print!("{}", $formatted_str);
        }
    };
}

macro_rules! debug_mode {
    () => {
        match ::std::env::var("DEBUG") {
            Ok(_value) => true,
            Err(_error) => false,
        };
    }
}

macro_rules! debug_until {
    () => {
        match ::std::env::var("UNTIL") {
            Ok(value) => match u32::from_str_radix(value.trim_start_matches("0x"), 16) {
                Ok(value) => value as u16,
                Err(_error) => 0xFFFF,
            },
            Err(_error) => 0xFFFF,
        };
    }
}
