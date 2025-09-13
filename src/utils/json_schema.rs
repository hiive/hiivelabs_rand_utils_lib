use schemars::_private::serde_json;
use schemars::{schema_for, JsonSchema, Schema, SchemaGenerator};
// https://graham.cool/schemars/deriving/attributes/#regex
#[derive(JsonSchema)]
#[schemars(deny_unknown_fields)]
// #[schemars(title = "NameAndDoomingArray")]
struct NameAndDooming {
    #[schemars(title = "Character Name", regex(pattern = r"^[A-Za-z]+$"))]
    character_name: String,
    #[schemars(title = "Dooming")]
    doom: String,
}

#[test]
fn test_json_schema() {
    let schema = schema_for!(Vec<NameAndDooming>);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}

// #[test]
// fn test_speech() {
//     use natural_tts::{models::tts_rs::TtsModel, *};
//     use std::error::Error;
//     // Create the NaturalTts struct using the builder pattern.
//     let mut natural = NaturalTtsBuilder::default()
//         .tts_model(TtsModel::default())
//         .default_model(Model::TTS)
//         .build()
//         .unwrap();
//
//     // Use the pre-included function to say a message using the default_model.
//     let _ = natural.say_auto("Hello, World!".to_string());
// }
