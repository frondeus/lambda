#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 18
#define LARGE_STATE_COUNT 15
#define SYMBOL_COUNT 17
#define ALIAS_COUNT 0
#define TOKEN_COUNT 11
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 6
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 4

enum ts_symbol_identifiers {
  sym_ident = 1,
  sym_comment = 2,
  anon_sym_LPAREN = 3,
  anon_sym_RPAREN = 4,
  anon_sym_true = 5,
  anon_sym_false = 6,
  anon_sym_COLON = 7,
  anon_sym_let = 8,
  anon_sym_EQ = 9,
  anon_sym_SEMI = 10,
  sym_source_file = 11,
  sym__expr = 12,
  sym_bool = 13,
  sym_def = 14,
  sym_call = 15,
  sym_let = 16,
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
  [anon_sym_let] = "let",
  [anon_sym_EQ] = "=",
  [anon_sym_SEMI] = ";",
  [sym_source_file] = "source_file",
  [sym__expr] = "_expr",
  [sym_bool] = "bool",
  [sym_def] = "def",
  [sym_call] = "call",
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
  [anon_sym_let] = anon_sym_let,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [sym_source_file] = sym_source_file,
  [sym__expr] = sym__expr,
  [sym_bool] = sym_bool,
  [sym_def] = sym_def,
  [sym_call] = sym_call,
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
  [sym_let] = {
    .visible = true,
    .named = true,
  },
};

enum ts_field_identifiers {
  field_arg = 1,
  field_body = 2,
  field_func = 3,
  field_in = 4,
  field_key = 5,
  field_value = 6,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_arg] = "arg",
  [field_body] = "body",
  [field_func] = "func",
  [field_in] = "in",
  [field_key] = "key",
  [field_value] = "value",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 2},
  [2] = {.index = 2, .length = 2},
  [3] = {.index = 4, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_arg, 1},
    {field_func, 0},
  [2] =
    {field_arg, 0},
    {field_body, 2},
  [4] =
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
      if (lookahead == 'f') ADVANCE(1);
      if (lookahead == 'l') ADVANCE(2);
      if (lookahead == 't') ADVANCE(3);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      END_STATE();
    case 1:
      if (lookahead == 'a') ADVANCE(4);
      END_STATE();
    case 2:
      if (lookahead == 'e') ADVANCE(5);
      END_STATE();
    case 3:
      if (lookahead == 'r') ADVANCE(6);
      END_STATE();
    case 4:
      if (lookahead == 'l') ADVANCE(7);
      END_STATE();
    case 5:
      if (lookahead == 't') ADVANCE(8);
      END_STATE();
    case 6:
      if (lookahead == 'u') ADVANCE(9);
      END_STATE();
    case 7:
      if (lookahead == 's') ADVANCE(10);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_let);
      END_STATE();
    case 9:
      if (lookahead == 'e') ADVANCE(11);
      END_STATE();
    case 10:
      if (lookahead == 'e') ADVANCE(12);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 12:
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
    [anon_sym_let] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(16),
    [sym__expr] = STATE(5),
    [sym_bool] = STATE(5),
    [sym_def] = STATE(5),
    [sym_call] = STATE(5),
    [sym_let] = STATE(5),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_let] = ACTIONS(11),
  },
  [2] = {
    [sym__expr] = STATE(2),
    [sym_bool] = STATE(2),
    [sym_def] = STATE(2),
    [sym_call] = STATE(2),
    [sym_let] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(13),
    [sym_ident] = ACTIONS(15),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(13),
    [anon_sym_RPAREN] = ACTIONS(13),
    [anon_sym_true] = ACTIONS(15),
    [anon_sym_false] = ACTIONS(15),
    [anon_sym_let] = ACTIONS(15),
    [anon_sym_SEMI] = ACTIONS(13),
  },
  [3] = {
    [sym__expr] = STATE(2),
    [sym_bool] = STATE(2),
    [sym_def] = STATE(2),
    [sym_call] = STATE(2),
    [sym_let] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(17),
    [sym_ident] = ACTIONS(19),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(22),
    [anon_sym_RPAREN] = ACTIONS(17),
    [anon_sym_true] = ACTIONS(25),
    [anon_sym_false] = ACTIONS(25),
    [anon_sym_let] = ACTIONS(28),
    [anon_sym_SEMI] = ACTIONS(17),
  },
  [4] = {
    [sym__expr] = STATE(2),
    [sym_bool] = STATE(2),
    [sym_def] = STATE(2),
    [sym_call] = STATE(2),
    [sym_let] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(31),
    [sym_ident] = ACTIONS(33),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(36),
    [anon_sym_RPAREN] = ACTIONS(31),
    [anon_sym_true] = ACTIONS(39),
    [anon_sym_false] = ACTIONS(39),
    [anon_sym_let] = ACTIONS(42),
    [anon_sym_SEMI] = ACTIONS(31),
  },
  [5] = {
    [sym__expr] = STATE(2),
    [sym_bool] = STATE(2),
    [sym_def] = STATE(2),
    [sym_call] = STATE(2),
    [sym_let] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(45),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_let] = ACTIONS(11),
  },
  [6] = {
    [sym__expr] = STATE(2),
    [sym_bool] = STATE(2),
    [sym_def] = STATE(2),
    [sym_call] = STATE(2),
    [sym_let] = STATE(2),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_RPAREN] = ACTIONS(47),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_let] = ACTIONS(11),
  },
  [7] = {
    [sym__expr] = STATE(2),
    [sym_bool] = STATE(2),
    [sym_def] = STATE(2),
    [sym_call] = STATE(2),
    [sym_let] = STATE(2),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_let] = ACTIONS(11),
    [anon_sym_SEMI] = ACTIONS(49),
  },
  [8] = {
    [sym__expr] = STATE(6),
    [sym_bool] = STATE(6),
    [sym_def] = STATE(6),
    [sym_call] = STATE(6),
    [sym_let] = STATE(6),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_let] = ACTIONS(11),
  },
  [9] = {
    [sym__expr] = STATE(3),
    [sym_bool] = STATE(3),
    [sym_def] = STATE(3),
    [sym_call] = STATE(3),
    [sym_let] = STATE(3),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_let] = ACTIONS(11),
  },
  [10] = {
    [sym__expr] = STATE(7),
    [sym_bool] = STATE(7),
    [sym_def] = STATE(7),
    [sym_call] = STATE(7),
    [sym_let] = STATE(7),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_let] = ACTIONS(11),
  },
  [11] = {
    [sym__expr] = STATE(4),
    [sym_bool] = STATE(4),
    [sym_def] = STATE(4),
    [sym_call] = STATE(4),
    [sym_let] = STATE(4),
    [sym_ident] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(7),
    [anon_sym_true] = ACTIONS(9),
    [anon_sym_false] = ACTIONS(9),
    [anon_sym_let] = ACTIONS(11),
  },
  [12] = {
    [ts_builtin_sym_end] = ACTIONS(51),
    [sym_ident] = ACTIONS(53),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(51),
    [anon_sym_RPAREN] = ACTIONS(51),
    [anon_sym_true] = ACTIONS(53),
    [anon_sym_false] = ACTIONS(53),
    [anon_sym_COLON] = ACTIONS(55),
    [anon_sym_let] = ACTIONS(53),
    [anon_sym_SEMI] = ACTIONS(51),
  },
  [13] = {
    [ts_builtin_sym_end] = ACTIONS(57),
    [sym_ident] = ACTIONS(59),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(57),
    [anon_sym_RPAREN] = ACTIONS(57),
    [anon_sym_true] = ACTIONS(59),
    [anon_sym_false] = ACTIONS(59),
    [anon_sym_let] = ACTIONS(59),
    [anon_sym_SEMI] = ACTIONS(57),
  },
  [14] = {
    [ts_builtin_sym_end] = ACTIONS(61),
    [sym_ident] = ACTIONS(63),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LPAREN] = ACTIONS(61),
    [anon_sym_RPAREN] = ACTIONS(61),
    [anon_sym_true] = ACTIONS(63),
    [anon_sym_false] = ACTIONS(63),
    [anon_sym_let] = ACTIONS(63),
    [anon_sym_SEMI] = ACTIONS(61),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(65), 1,
      sym_ident,
  [7] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(67), 1,
      ts_builtin_sym_end,
  [14] = 2,
    ACTIONS(3), 1,
      sym_comment,
    ACTIONS(69), 1,
      anon_sym_EQ,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(15)] = 0,
  [SMALL_STATE(16)] = 7,
  [SMALL_STATE(17)] = 14,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(12),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call, 2, 0, 1),
  [15] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call, 2, 0, 1),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_def, 3, 0, 2),
  [19] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym_def, 3, 0, 2), SHIFT(12),
  [22] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym_def, 3, 0, 2), SHIFT(8),
  [25] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym_def, 3, 0, 2), SHIFT(13),
  [28] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym_def, 3, 0, 2), SHIFT(15),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_let, 6, 0, 3),
  [33] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym_let, 6, 0, 3), SHIFT(12),
  [36] = {.entry = {.count = 2, .reusable = true}}, REDUCE(sym_let, 6, 0, 3), SHIFT(8),
  [39] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym_let, 6, 0, 3), SHIFT(13),
  [42] = {.entry = {.count = 2, .reusable = false}}, REDUCE(sym_let, 6, 0, 3), SHIFT(15),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 1, 0, 0),
  [53] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr, 1, 0, 0),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [57] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bool, 1, 0, 0),
  [59] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_bool, 1, 0, 0),
  [61] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 3, 0, 0),
  [63] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__expr, 3, 0, 0),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [67] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
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
