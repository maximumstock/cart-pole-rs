use criterion::{black_box, criterion_group, criterion_main, Criterion};
use inference_web::{get_model_file_path, infer, InferenceInput};

fn criterion_benchmark(c: &mut Criterion) {
    let model = tch::CModule::load(get_model_file_path()).unwrap();
    let state: InferenceInput = [0f32, 0f32, -1f32, 0f32];

    c.bench_function("cartpole-inference", |b| {
        b.iter(|| {
            let _ = black_box(infer(&model, &state).unwrap());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
