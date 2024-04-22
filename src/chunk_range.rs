#[allow(unused)]
use log::{debug, info, trace, warn};

use reqwest::header::HeaderValue;

pub struct ChunkRangeIter {
    chunk_start: u64,
    chunk_end: u64,
    buffer_size: u32,
}

pub fn chunk_range_iter(chunk_start: u64, chunk_end: u64, buffer_size: u32) -> ChunkRangeIter {
    ChunkRangeIter {
        chunk_start,
        chunk_end,
        buffer_size,
    }
}

impl Iterator for ChunkRangeIter {
    type Item = HeaderValue;
    fn next(&mut self) -> Option<Self::Item> {
        let start_chunk = self.chunk_start;
        let end_chunk = self.chunk_end;
        let size_chunks = self.buffer_size;
        if start_chunk > end_chunk {
            None
        } else {
            let prev_start = start_chunk;
            let ending_size = end_chunk - start_chunk + 1;
            let cur_size_buffer = std::cmp::min(size_chunks as u64, ending_size);
            self.chunk_start += cur_size_buffer;
            let bytes_start_stop = &format!("bytes={}-{}", prev_start, self.chunk_start - 1);
            let the_range = HeaderValue::from_str(bytes_start_stop).expect("chunk_range_iter");
            Some(the_range)
        }
    }
}
