#![cfg(test)]
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]
#![cfg_attr(coverage_nightly, coverage(off))]

mod advanced_service;
mod bool_expressions;
mod bytes_service;
mod expressions;
mod flattened;
mod floats;
mod message_cel;
mod nested;
mod oneof;
mod optional_fields;
mod recursive;
mod renamed;
mod sfixed_sint;
mod simple;
mod simple_enum;
mod simple_service;
mod visibility;
mod well_known;
