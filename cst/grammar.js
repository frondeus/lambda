/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

const PREC = {
  "call": 2,
  "let": 1,
}

module.exports = grammar({
  name: "lambda",

  extras: $ => [
    /\s/,
    $.comment
  ],
  word: $ => $.ident,
  rules: {
    source_file: $ => $._expr,
    comment: $ => token(seq("#", /.*/)),
    _expr: $ => choice(
      seq("(", $._expr,  ")"),
      $.bool,
      $.ident,
      $.def,
      $.call,
      $.ifElse,
      $["let"]
    ),
    bool: $ => choice("true", "false"),
    ident: $ => /[a-zA-Z_0-9]+/,
    def: $ => seq( field("arg", $.ident), ":", field("body", $._expr) ),
    call: $ => prec.left(PREC.call, seq(
      field("func", $._expr), 
      field("arg", $._expr), 
    )),
    ifElse: $ => seq(
      "if",
      field("cond", $._expr),
      "then",
      field("then", $._expr),
      "else",
      field("else", $._expr)
    ),
    "let": $ => prec.right(PREC.let, seq(
      "let",
      field("key", $.ident),
      "=",
      field("value", $._expr),
      ";",
      field("in", $._expr)
    ))
  }
});
