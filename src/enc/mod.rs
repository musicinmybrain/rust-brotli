pub mod fast_log;
pub mod brotli_bit_stream;
pub mod constants;
pub mod entropy_encode;
pub mod utf8_util;
static mut kInsExtra
    : [u32; 24]
    = [   0u32,
          0u32,
          0u32,
          0u32,
          0u32,
          0u32,
          1u32,
          1u32,
          2u32,
          2u32,
          3u32,
          3u32,
          4u32,
          4u32,
          5u32,
          5u32,
          6u32,
          7u32,
          8u32,
          9u32,
          10u32,
          12u32,
          14u32,
          24u32
      ];