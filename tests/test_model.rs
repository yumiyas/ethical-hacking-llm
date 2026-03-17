use ethical_hacking_llm::model::local_model::LocalModel;
use ethical_hacking_llm::model::ModelTrait;

#[tokio::test]
async fn test_model_loading() {
    // Skip if model files don't exist
    if !std::path::Path::new("models/phi-2-q4/phi-2-q4.gguf").exists() {
        println!("Skipping test: model files not found");
        return;
    }

    let model = LocalModel::new(
        "models/phi-2-q4/phi-2-q4.gguf",
        "models/phi-2-q4/tokenizer.json",
        0.7,
    );
    
    assert!(model.is_ok());
}
