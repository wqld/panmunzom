#![no_std]

// This file exists to enable the library target.
pub fn record_type_to_str(record_type: u16) -> &'static str {
    match record_type {
        1 => "A",
        2 => "NS",
        5 => "CNAME",
        6 => "SOA",
        12 => "PTR",
        15 => "MX",
        16 => "TXT",
        28 => "AAAA",
        33 => "SRV",
        255 => "ANY",
        _ => "UNKNOWN",
    }
}

/// Convert class to string
pub fn class_to_str(class: u16) -> &'static str {
    match class {
        1 => "IN",
        2 => "CS",
        3 => "CH",
        4 => "HS",
        _ => "UNKNOWN",
    }
}
