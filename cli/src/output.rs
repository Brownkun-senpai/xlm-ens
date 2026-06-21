use clap::ValueEnum;
use serde_json::Value;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Human,
    Json,
    #[value(name = "csv")]
    Csv,
}

pub fn emit(format: OutputFormat, human: &str, json: Value) {
    match format {
        OutputFormat::Human => println!("{human}"),
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&json).expect("json output should always serialize")
            );
        }
        OutputFormat::Csv => {
            emit_csv(&json, &mut std::io::stdout());
        }
    }
}

pub fn emit_error(format: OutputFormat, human: &str, json: Value) {
    match format {
        OutputFormat::Human => eprintln!("{human}"),
        OutputFormat::Json => {
            eprintln!(
                "{}",
                serde_json::to_string_pretty(&json).expect("json output should always serialize")
            );
        }
        OutputFormat::Csv => {
            emit_csv(&json, &mut std::io::stderr());
        }
    }
}

fn emit_csv(json: &Value, writer: &mut dyn std::io::Write) {
    if let Value::Object(map) = json {
        let keys: Vec<&str> = map.keys().map(|k| k.as_str()).collect();
        let values: Vec<String> = map
            .values()
            .map(|v| match v {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .collect();
        let _ = writeln!(writer, "{}", keys.join(","));
        let _ = writeln!(writer, "{}", values.join(","));
    }
}

#[cfg(test)]
mod tests {
    use super::OutputFormat;
    use clap::ValueEnum;

    #[test]
    fn test_invalid_output_mode_exits_with_error() {
        assert!(OutputFormat::from_str("tsv", false).is_err());
    }
}
