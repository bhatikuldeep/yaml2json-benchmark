# yaml2json

**yaml2json** is a lightweight, fast command-line tool to convert YAML files to JSON while preserving the original YAML values.

## Executive Summary

Many YAML→JSON conversions in CI/CD pipelines or data processing are slow or inconsistent, especially for large YAML files. This tool is built in Go for **speed**, simplicity, and multi-platform compatibility.

## Features

* Converts multi-document YAML (`---` separators) to JSON.
* Preserves YAML values **without type coercion**.
* Outputs indented JSON for readability.
* Supports Linux & macOS, AMD64 & ARM64.

## Performance Comparison

| File Size | `yaml2json` (Go) | Python `pyyaml` |
|-----------|------------------|-----------------|
| 35 MB     | 0.08s           | 1.2s            |
| 167 MB    | 0.35s           | 5.6s            |

*Tested on MacBook M1; Go version keeps processing memory efficient.*

## Installation

Download the latest release for your OS/architecture from the [Releases](https://github.com/your-username/yaml2json/releases) page.

```bash
# Make it executable
chmod +x yaml2json-darwin-arm64
```

## Usage

```bash
# Convert YAML to JSON
./yaml2json-darwin-arm64 <input.yaml> <output.json>

# Example
./yaml2json-darwin-arm64 kong35.yml kong.json
```

**Output:** `output.json` will contain all YAML documents as JSON objects, maintaining values as-is.

## Why yaml2json?

* **Faster:** Optimized Go implementation handles large YAML files quickly.
* **Reliable:** Preserves original YAML values without unwanted type conversions.
* **Portable:** Multi-platform binaries with single executable.

## Contributing

Issues and pull requests are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.