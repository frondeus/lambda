/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "lambda",

  extras: $ => [
    /\s|\r?\n/,
    $.comment
  ],
  word: $ => $.ident,
  rules: {
    source_file: $ => $._expr,
    comment: $ => token(seq("#", /.*/)),
    _expr: $ => choice(
      seq("(", $._expr,  ")"),
      "true",
      "false",
      $.ident,
      $.def,
      $.call,
      $["let"]
    ),
    ident: $ => /[a-zA-Z_0-9]+/,
    def: $ => seq( field("arg", $.ident), ":", field("body", $._expr) ),
    call: $ => prec.left(seq(
      field("func", $._expr), 
      field("arg", $._expr), 
    )),
    "let": $ => seq(
      "let",
      field("key", $.ident),
      "=",
      field("value", $._expr),
      ";",
      field("in", $._expr)
    )
  },
  conflicts: $ => [
    [$.def, $.call],
    [$.call, $.let]
  ]
});
