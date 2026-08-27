use criterion::{criterion_group, criterion_main, Criterion};
use wc_awf::render::VectorShape;
use wc_awf::Color;

fn benchmark_shape_creation(c: &mut Criterion) {
    c.bench_function("vector_shape_batch", |b| {
        b.iter(|| {
            let _shapes = vec![
                VectorShape::Rectangle {
                    x: 10.0,
                    y: 10.0,
                    w: 100.0,
                    h: 100.0,
                    color: Color { r: 255, g: 0, b: 0, a: 1.0 },
                };
                1000
            ];
        })
    });
}

criterion_group!(benches, benchmark_shape_creation);
criterion_main!(benches);