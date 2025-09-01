use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};

/// Recursively convert YAML values to JSON-compatible values
fn convert_node(node: &YamlValue) -> JsonValue {
    match node {
        YamlValue::Mapping(map) => {
            let mut m = serde_json::Map::with_capacity(map.len());
            for (k, v) in map {
                // YAML keys must be strings for JSON, so stringify non-string keys
                let key = match k {
                    YamlValue::String(s) => s.clone(),
                    other => format!("{:?}", other),
                };
                m.insert(key, convert_node(v));
            }
            JsonValue::Object(m)
        }
        YamlValue::Sequence(seq) => {
            JsonValue::Array(seq.iter().map(convert_node).collect())
        }
        YamlValue::Null => JsonValue::Null,
        YamlValue::Bool(b) => JsonValue::Bool(*b),
        YamlValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                JsonValue::from(i)
            } else if let Some(f) = n.as_f64() {
                JsonValue::from(f)
            } else {
                JsonValue::Null
            }
        }
        YamlValue::String(s) => JsonValue::String(s.clone()),
        _ => JsonValue::Null,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: yaml2json <input.yaml> <output.json>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let in_file = File::open(input_file)?;
    let reader = BufReader::new(in_file);

    let out_file = File::create(output_file)?;
    let writer = BufWriter::new(out_file);

    let mut encoder = serde_json::Serializer::pretty(writer);

    // Stream through multiple YAML documents
    let docs = serde_yaml::Deserializer::from_reader(reader);
    for doc in docs {
        let yaml_value: YamlValue = YamlValue::deserialize(doc)?;
        let json_value = convert_node(&yaml_value);
        json_value.serialize(&mut encoder)?;
    }

    println!("Conversion complete.");
    Ok(())
}
