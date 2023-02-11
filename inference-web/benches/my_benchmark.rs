use criterion::{black_box, criterion_group, criterion_main, Criterion};
use inference_web::{extract_outputs, State};
use tch::IValue;

fn criterion_benchmark(c: &mut Criterion) {
    let model_file_path = "../model_traced.pt";
    let model = tch::CModule::load(model_file_path).unwrap();
    let state: State = [0f32, 0f32, -1f32, 0f32];

    c.bench_function("cartpole-inference", |b| {
        b.iter(|| {
            let _ = black_box(
                model
                    .forward_is(&[IValue::Tensor(tch::Tensor::of_slice(&state))])
                    .map(extract_outputs)
                    .unwrap(),
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
