package main

import (
	"encoding/json"
	"fmt"
	"os"

	"gopkg.in/yaml.v3"
)

// convertNode recursively converts YAML nodes to JSON-compatible Go values.
func convertNode(n *yaml.Node) any {
	switch n.Kind {
	case yaml.DocumentNode:
		if len(n.Content) > 0 {
			return convertNode(n.Content[0])
		}
	case yaml.MappingNode:
		m := make(map[string]any, len(n.Content)/2)
		for i := 0; i < len(n.Content); i += 2 {
			key := n.Content[i]
			value := n.Content[i+1]
			m[key.Value] = convertNode(value)
		}
		return m
	case yaml.SequenceNode:
		arr := make([]any, len(n.Content))
		for i, c := range n.Content {
			arr[i] = convertNode(c)
		}
		return arr
	case yaml.ScalarNode:
		return n.Value
	}
	return nil
}

func main() {
	if len(os.Args) != 3 {
		fmt.Println("Usage: yaml2json <input.yaml> <output.json>")
		os.Exit(1)
	}

	inputFile := os.Args[1]
	outputFile := os.Args[2]

	in, err := os.Open(inputFile)
	if err != nil {
		panic(err)
	}
	defer in.Close()

	out, err := os.Create(outputFile)
	if err != nil {
		panic(err)
	}
	defer out.Close()

	decoder := yaml.NewDecoder(in)
	encoder := json.NewEncoder(out)
	encoder.SetIndent("", " ")

	// Handle multiple YAML documents separated by ---
	for {
		var root yaml.Node
		err := decoder.Decode(&root)
		if err != nil {
			if err.Error() == "EOF" {
				break
			}
			panic(err)
		}

		converted := convertNode(&root)
		if err := encoder.Encode(converted); err != nil {
			panic(err)
		}
	}

	fmt.Println("Conversion complete.")
}
