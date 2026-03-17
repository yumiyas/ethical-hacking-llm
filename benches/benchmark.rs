use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use ethical_hacking_llm::model::local_model::LocalModel;
use ethical_hacking_llm::model::ModelTrait;
use tokio::runtime::Runtime;

fn bench_model_inference(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    // Skip if model files don't exist
    if !std::path::Path::new("models/phi-2-q4/phi-2-q4.gguf").exists() {
        println!("Skipping benchmark: model files not found");
        return;
    }

    // Load model
    let model = rt.block_on(async {
        LocalModel::new(
            "models/phi-2-q4/phi-2-q4.gguf",
            "models/phi-2-q4/tokenizer.json",
            0.7,
        ).unwrap()
    });

    let queries = vec![
        ("short", "nmap scan"),
        ("medium", "How to use nmap for port scanning?"),
        ("long", "Explain in detail how to use nmap for comprehensive port scanning including SYN scan, version detection, and OS fingerprinting with examples"),
    ];

    let mut group = c.benchmark_group("model_inference");
    group.sample_size(10);

    for (size, query) in queries {
        group.bench_with_input(BenchmarkId::new("response_time", size), &query, |b, q| {
            b.to_async(&rt).iter(|| async {
                let _ = model.generate(q, 50).await;
            })
        });
    }

    group.finish();
}

fn bench_api_endpoint(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let client = reqwest::Client::new();

    c.bench_function("api_query", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = client
                .post("http://localhost:3000/query")
                .json(&serde_json::json!({
                    "query": "nmap port scanning",
                    "max_tokens": 50
                }))
                .send()
                .await;
        })
    });
}

criterion_group!(benches, bench_model_inference, bench_api_endpoint);
criterion_main!(benches);
