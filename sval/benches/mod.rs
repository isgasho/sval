#![feature(test)]

extern crate sval;
extern crate test;

use sval::{
    stream,
    value,
};

use std::fmt;

use test::{
    black_box,
    Bencher,
};

struct EmptyStream;

impl stream::Stream for EmptyStream {
    #[inline(never)]
    fn fmt(&mut self, _: fmt::Arguments) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn u64(&mut self, _: u64) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn begin(&mut self) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn end(&mut self) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn seq_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn seq_elem(&mut self) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn seq_end(&mut self) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn map_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn map_key(&mut self) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn map_value(&mut self) -> Result<(), stream::Error> {
        Ok(())
    }

    #[inline(never)]
    fn map_end(&mut self) -> Result<(), stream::Error> {
        Ok(())
    }
}

#[bench]
fn stack_map(b: &mut Bencher) {
    b.iter(|| {
        let mut stack = stream::Stack::new();

        stack.map_begin().unwrap();

        stack.map_key().unwrap();
        stack.primitive().unwrap();

        stack.map_value().unwrap();
        stack.map_begin().unwrap();

        stack.map_key().unwrap();
        stack.primitive().unwrap();

        stack.map_value().unwrap();
        stack.primitive().unwrap();

        stack.map_end().unwrap();
        stack.map_end().unwrap();

        stack.end().unwrap();

        black_box(stack);
    })
}

#[bench]
fn stack_primitive(b: &mut Bencher) {
    b.iter(|| {
        let mut stack = stream::Stack::new();

        stack.primitive().unwrap();
        stack.end().unwrap();

        black_box(stack);
    })
}

#[bench]
fn unchecked_stream_map(b: &mut Bencher) {
    b.iter(|| {
        let stream: &mut dyn stream::Stream = &mut EmptyStream;

        stream.map_begin(None).unwrap();

        stream.map_key().unwrap();
        stream.u64(1).unwrap();

        stream.map_value().unwrap();
        stream.map_begin(None).unwrap();

        stream.map_key().unwrap();
        stream.u64(2).unwrap();

        stream.map_value().unwrap();
        stream.u64(42).unwrap();

        stream.map_end().unwrap();

        stream.map_end().unwrap();

        stream.end().unwrap();

        black_box(stream);
    })
}

#[bench]
fn unchecked_stream_map_collect(b: &mut Bencher) {
    b.iter(|| {
        let stream: &mut dyn stream::Stream = &mut EmptyStream;

        stream.map_begin(None).unwrap();

        stream.map_key_collect(1).unwrap();

        stream.map_value().unwrap();
        stream.map_begin(None).unwrap();

        stream.map_key_collect(2).unwrap();

        stream.map_value_collect(42).unwrap();

        stream.map_end().unwrap();

        stream.map_end().unwrap();

        stream.end().unwrap();

        black_box(stream);
    })
}
