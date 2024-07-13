#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 24
#define LARGE_STATE_COUNT 21
#define SYMBOL_COUNT 21
#define ALIAS_COUNT 0
#define TOKEN_COUNT 14
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 9
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 5

enum ts_symbol_identifiers {
  sym_ident = 1,
  sym_comment = 2,
  anon_sym_LPAREN = 3,
  anon_sym_RPAREN = 4,
  anon_sym_true = 5,
  anon_sym_false = 6,
  anon_sym_COLON = 7,
  anon_sym_if = 8,
  anon_sym_then = 9,
  anon_sym_else = 10,
  anon_sym_let = 11,
  anon_sym_EQ = 12,
  anon_sym_SEMI = 13,
  sym_source_file = 14,
  sym__expr = 15,
  sym_bool = 16,
  sym_def = 17,
  sym_call = 18,
  sym_ifElse = 19,
  sym_let = 20,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_ident] = "ident",
  [sym_comment] = "comment",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [anon_sym_COLON] = ":",
  [anon_sym_if] = "if",
  [anon_sym_then] = "then",
  [anon_sym_else] = "else",
  [anon_sym_let] = "let",
  [anon_sym_EQ] = "=",
  [anon_sym_SEMI] = ";",
  [sym_source_file] = "source_file",
  [sym__expr] = "_expr",
  [sym_bool] = "bool",
  [sym_def] = "def",
  [sym_call] = "call",
  [sym_ifElse] = "ifElse",
  [sym_let] = "let",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_ident] = sym_ident,
  [sym_comment] = sym_comment,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_then] = anon_sym_then,
  [anon_sym_else] = anon_sym_else,
  [anon_sym_let] = anon_sym_let,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [sym_source_file] = sym_source_file,
  [sym__expr] = sym__expr,
  [sym_bool] = sym_bool,
  [sym_def] = sym_def,
  [sym_call] = sym_call,
  [sym_ifElse] = sym_ifElse,
  [sym_let] = sym_let,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_ident] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_if] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_then] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_else] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_let] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__expr] = {
    .visible = false,
    .named = true,
  },
  [sym_bool] = {
    .visible = true,
    .named = true,
  },
  [sym_def] = {
    .visible = true,
    .named = true,
  },
  [sym_call] = {
    .visible = true,
    .named = true,
  },
  [sym_ifElse] = {
    .visible = true,
    .named = true,
  },
  [sym_let] = {
    .visible = true,
    .named = true,
  },
};

enum ts_field_identifiers {
  field_arg = 1,
  field_body = 2,
  field_cond = 3,
  field_else = 4,
  field_func = 5,
  field_in = 6,
  field_key = 7,
  field_then = 8,
  field_value = 9,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_arg] = "arg",
  [field_body] = "body",
  [field_cond] = "cond",
  [field_else] = "else",
  [field_func] = "func",
  [field_in] = "in",
  [field_key] = "key",
  [field_then] = "then",
  [field_value] = "value",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 2},
  [3] = {.index = 4, .length = 3},
  [4] = {.index = 7, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_arg, 1},
    {field_func, 0},
  [2] =
    {field_arg, 0},
    {field_body, 2},
  [4] =
    {field_cond, 1},
    {field_else, 5},
    {field_then, 3},
  [7] =
    {field_in, 5},
    {field_key, 1},
    {field_value, 3},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(1);
      if (lookahead == '#') ADVANCE(2);
      if (lookahead == '(') ADVANCE(3);
      if (lookahead == ')') ADVANCE(4);
      if (lookahead == ':') ADVANCE(6);
      if (lookahead == ';') ADVANCE(8);
      if (lookahead == '=') ADVANCE(7);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(5);
      END_STATE();
    case 1:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 2:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(2);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(sym_ident);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(5);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (lookahead == 'e') ADVANCE(1);
      if (lookahead == 'f') ADVANCE(2);
      if (lookahead == 'i') ADVANCE(3);
      if (lookahead == 'l') ADVANCE(4);
      if (lookahead == 't') ADVANCE(5);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      END_STATE();
    case 1:
      if (lookahead == 'l') ADVANCE(6);
      END_STATE();
    case 2:
      if (lookahead == 'a') ADVANCE(7);
      END_STATE();
    case 3:
      if (lookahead == 'f') ADVANCE(8);
      END_STATE();
    case 4:
      if (lookahead == 'e') ADVANCE(9);
      END_STATE();
    case 5:
      if (lookahead == 'h') ADVANCE(10);
      if (lookahead == 'r') ADVANCE(11);
      END_STATE();
    case 6:
      if (lookahead == 's') ADVANCE(12);
      END_STATE();
    case 7:
      if (lookahead == 'l') ADVANCE(13);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 9:
      if (lookahead == 't') ADVANCE(14);
      END_STATE();
    case 10:
      if (lookahead == 'e') ADVANCE(15);
      END_STATE();
    case 11:
      if (lookahead == 'u') ADVANCE(16);
      END_STATE();
    case 12:
      if (lookahead == 'e') ADVANCE(17);
      END_STATE();
    case 13:
      if (lookahead == 's') ADVANCE(18);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_let);
      END_STATE();
    case 15:
      if (lookahead == 'n') ADVANCE(19);
      END_STATE();
    case 16:
      if (lookahead == 'e') ADVANCE(20);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 18:
      if (lookahead == 'e') ADVANCE(21);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(anon_sym_then);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_ident] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_then] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_let] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(22),
    [sym__expr] = STATE(9),
    [sym_bool] = STATE(9),
    [sym_def] = STATE(9),
    [sym_call] = STATE(9),
    [sym_ifElse] = STATE(9),
    [sym_let] = STATE(9),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_let] = ACTIONS(13),
  },
  [2] = {
    [sym__expr] = STATE(3),
    [sym_bool] = STATE(3),
    [sym_def] = STATE(3),
    [sym_call] = STATE(3),
    [sym_ifElse] = STATE(3),
    [sym_let] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(15),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(15),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_then] = ACTIONS(17),
    [anon_sym_else] = ACTIONS(17),
    [anon_sym_let] = ACTIONS(13),
    [anon_sym_SEMI] = ACTIONS(15),
  },
  [3] = {
    [sym__expr] = STATE(3),
    [sym_bool] = STATE(3),
    [sym_def] = STATE(3),
    [sym_call] = STATE(3),
    [sym_ifElse] = STATE(3),
    [sym_let] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(19),
    [sym_ident] = ACTIONS(21),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(19),
    [anon_sym_RPAREN] = ACTIONS(19),
    [anon_sym_true] = ACTIONS(21),
    [anon_sym_false] = ACTIONS(21),
    [anon_sym_if] = ACTIONS(21),
    [anon_sym_then] = ACTIONS(21),
    [anon_sym_else] = ACTIONS(21),
    [anon_sym_let] = ACTIONS(21),
    [anon_sym_SEMI] = ACTIONS(19),
  },
  [4] = {
    [sym__expr] = STATE(3),
    [sym_bool] = STATE(3),
    [sym_def] = STATE(3),
    [sym_call] = STATE(3),
    [sym_ifElse] = STATE(3),
    [sym_let] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(23),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(23),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_then] = ACTIONS(25),
    [anon_sym_else] = ACTIONS(25),
    [anon_sym_let] = ACTIONS(13),
    [anon_sym_SEMI] = ACTIONS(23),
  },
  [5] = {
    [sym__expr] = STATE(3),
    [sym_bool] = STATE(3),
    [sym_def] = STATE(3),
    [sym_call] = STATE(3),
    [sym_ifElse] = STATE(3),
    [sym_let] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(27),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(27),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_then] = ACTIONS(29),
    [anon_sym_else] = ACTIONS(29),
    [anon_sym_let] = ACTIONS(13),
    [anon_sym_SEMI] = ACTIONS(27),
  },
  [6] = {
    [sym__expr] = STATE(3),
    [sym_bool] = STATE(3),
    [sym_def] = STATE(3),
    [sym_call] = STATE(3),
    [sym_ifElse] = STATE(3),
    [sym_let] = STATE(3),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_then] = ACTIONS(31),
    [anon_sym_let] = ACTIONS(13),
  },
  [7] = {
    [sym__expr] = STATE(3),
    [sym_bool] = STATE(3),
    [sym_def] = STATE(3),
    [sym_call] = STATE(3),
    [sym_ifElse] = STATE(3),
    [sym_let] = STATE(3),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_let] = ACTIONS(13),
    [anon_sym_SEMI] = ACTIONS(33),
  },
  [8] = {
    [sym__expr] = STATE(3),
    [sym_bool] = STATE(3),
    [sym_def] = STATE(3),
    [sym_call] = STATE(3),
    [sym_ifElse] = STATE(3),
    [sym_let] = STATE(3),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_else] = ACTIONS(35),
    [anon_sym_let] = ACTIONS(13),
  },
  [9] = {
    [sym__expr] = STATE(3),
    [sym_bool] = STATE(3),
    [sym_def] = STATE(3),
    [sym_call] = STATE(3),
    [sym_ifElse] = STATE(3),
    [sym_let] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(37),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_let] = ACTIONS(13),
  },
  [10] = {
    [sym__expr] = STATE(3),
    [sym_bool] = STATE(3),
    [sym_def] = STATE(3),
    [sym_call] = STATE(3),
    [sym_ifElse] = STATE(3),
    [sym_let] = STATE(3),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(39),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_let] = ACTIONS(13),
  },
  [11] = {
    [sym__expr] = STATE(6),
    [sym_bool] = STATE(6),
    [sym_def] = STATE(6),
    [sym_call] = STATE(6),
    [sym_ifElse] = STATE(6),
    [sym_let] = STATE(6),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_let] = ACTIONS(13),
  },
  [12] = {
    [sym__expr] = STATE(10),
    [sym_bool] = STATE(10),
    [sym_def] = STATE(10),
    [sym_call] = STATE(10),
    [sym_ifElse] = STATE(10),
    [sym_let] = STATE(10),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_let] = ACTIONS(13),
  },
  [13] = {
    [sym__expr] = STATE(2),
    [sym_bool] = STATE(2),
    [sym_def] = STATE(2),
    [sym_call] = STATE(2),
    [sym_ifElse] = STATE(2),
    [sym_let] = STATE(2),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_let] = ACTIONS(13),
  },
  [14] = {
    [sym__expr] = STATE(4),
    [sym_bool] = STATE(4),
    [sym_def] = STATE(4),
    [sym_call] = STATE(4),
    [sym_ifElse] = STATE(4),
    [sym_let] = STATE(4),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_let] = ACTIONS(13),
  },
  [15] = {
    [sym__expr] = STATE(8),
    [sym_bool] = STATE(8),
    [sym_def] = STATE(8),
    [sym_call] = STATE(8),
    [sym_ifElse] = STATE(8),
    [sym_let] = STATE(8),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_let] = ACTIONS(13),
  },
  [16] = {
    [sym__expr] = STATE(7),
    [sym_bool] = STATE(7),
    [sym_def] = STATE(7),
    [sym_call] = STATE(7),
    [sym_ifElse] = STATE(7),
    [sym_let] = STATE(7),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_let] = ACTIONS(13),
  },
  [17] = {
    [ts_builtin_sym_end] = ACTIONS(41),
    [sym_ident] = ACTIONS(43),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(41),
    [anon_sym_RPAREN] = ACTIONS(41),
    [anon_sym_true] = ACTIONS(43),
    [anon_sym_false] = ACTIONS(43),
    [anon_sym_COLON] = ACTIONS(45),
    [anon_sym_if] = ACTIONS(43),
    [anon_sym_then] = ACTIONS(43),
    [anon_sym_else] = ACTIONS(43),
    [anon_sym_let] = ACTIONS(43),
    [anon_sym_SEMI] = ACTIONS(41),
  },
  [18] = {
    [sym__expr] = STATE(5),
    [sym_bool] = STATE(5),
    [sym_def] = STATE(5),
    [sym_call] = STATE(5),
    [sym_ifElse] = STATE(5),
    [sym_let] = STATE(5),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_if] = ACTIONS(11),
    [anon_sym_let] = ACTIONS(13),
  },
  [19] = {
    [ts_builtin_sym_end] = ACTIONS(47),
    [sym_ident] = ACTIONS(49),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(47),
    [anon_sym_RPAREN] = ACTIONS(47),
    [anon_sym_true] = ACTIONS(49),
    [anon_sym_false] = ACTIONS(49),
    [anon_sym_if] = ACTIONS(49),
    [anon_sym_then] = ACTIONS(49),
    [anon_sym_else] = ACTIONS(49),
    [anon_sym_let] = ACTIONS(49),
    [anon_sym_SEMI] = ACTIONS(47),
  },
  [20] = {
    [ts_builtin_sym_end] = ACTIONS(51),
    [sym_ident] = ACTIONS(53),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(51),
    [anon_sym_RPAREN] = ACTIONS(51),
    [anon_sym_true] = ACTIONS(53),
    [anon_sym_false] = ACTIONS(53),
    [anon_sym_if] = ACTIONS(53),
    [anon_sym_then] = ACTIONS(53),
    [anon_sym_else] = ACTIONS(53),
    [anon_sym_let] = ACTIONS(53),
    [anon_sym_SEMI] = ACTIONS(51),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(55), 1,
      sym_ident,
  [7] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(57), 1,
      ts_builtin_sym_end,
  [14] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(59), 1,
      anon_sym_EQ,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(21)] = 0,
  [SMALL_STATE(22)] = 7,
  [SMALL_STATE(23)] = 14,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_def, 3, 0, 2),
  [17] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_def, 3, 0, 2),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call, 2, 0, 1),
  [21] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call, 2, 0, 1),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_let, 6, 0, 4),
  [25] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_let, 6, 0, 4),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ifElse, 6, 0, 3),
  [29] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ifElse, 6, 0, 3),
  [31] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [35] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 1, 0, 0),
  [43] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr, 1, 0, 0),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 3, 0, 0),
  [49] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr, 3, 0, 0),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bool, 1, 0, 0),
  [53] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_bool, 1, 0, 0),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [57] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_lambda(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_ident,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
