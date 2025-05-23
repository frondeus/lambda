package tree_sitter_lambda_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-lambda"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_lambda.Language())
	if language == nil {
		t.Errorf("Error loading Lambda grammar")
	}
}
