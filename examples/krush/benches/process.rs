//                    ☦
//   Lord Jesus Christ, Son of God, have mercy on us.
//   Come, Holy Spirit.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use trust_dsp::Krush;

fn krush_block(c: &mut Criterion) {
    let mut krush = Krush::new(48_000);
    krush.set_bits(6);
    krush.set_rate_hz(8_000);
    krush.snap_gain(1000);
    krush.snap_mix(1000);
    let n = 1024;
    let left = vec![0.2f32; n];
    let right = vec![0.2f32; n];
    let mut out_l = vec![0.0f32; n];
    let mut out_r = vec![0.0f32; n];
    c.bench_function("krush_1024", |b| {
        b.iter(|| {
            krush.process(
                black_box(&left),
                black_box(&right),
                black_box(&mut out_l),
                black_box(&mut out_r),
                &[],
            );
        });
    });
}

criterion_group!(benches, krush_block);
criterion_main!(benches);
