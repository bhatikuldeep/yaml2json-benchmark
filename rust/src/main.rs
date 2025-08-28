use serde::{Deserialize};
use serde_yaml::Value;
use serde_json::{Value as JsonValue, to_writer};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: yaml2json <input.yaml> <output.json>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let file = File::open(input_file).expect("Cannot open input file");
    let reader = BufReader::new(file);

    let out_file = File::create(output_file).expect("Cannot create output file");
    let mut writer = BufWriter::new(out_file);

    let stream = serde_yaml::Deserializer::from_reader(reader);

    // Write opening of JSON array
    writer.write_all(b"[").expect("Failed to write JSON array start");

    let mut first = true;
    for doc in stream {
        // Correctly deserialize each YAML document
        let yaml_value: Value = Value::deserialize(doc).expect("Failed to parse YAML document");
        let json_value: JsonValue = yaml_to_json(yaml_value);

        if !first {
            writer.write_all(b",").expect("Failed to write comma");
        }
        first = false;

        // Write JSON object incrementally
        to_writer(&mut writer, &json_value).expect("Failed to write JSON object");
    }

    // Write closing of JSON array
    writer.write_all(b"]").expect("Failed to write JSON array end");

    writer.flush().expect("Failed to flush writer");

    println!("Conversion complete.");
}

fn yaml_to_json(value: Value) -> JsonValue {
    match value {
        Value::Null => JsonValue::Null,
        Value::Bool(b) => JsonValue::Bool(b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                JsonValue::from(i)
            } else if let Some(f) = n.as_f64() {
                JsonValue::from(f)
            } else {
                JsonValue::Null
            }
        }
        Value::String(s) => JsonValue::String(s),
        Value::Sequence(seq) => JsonValue::Array(seq.into_iter().map(yaml_to_json).collect()),
        Value::Mapping(map) => {
            let mut m = serde_json::Map::new();
            for (k, v) in map {
                let key = match k {
                    Value::String(s) => s,
                    other => format!("{:?}", other), // debug formatting instead of Display
                };
                m.insert(key, yaml_to_json(v));
            }
            JsonValue::Object(m)
        }
        _ => JsonValue::Null,
    }
}
