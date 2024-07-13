```
let f = a: a;
f true
```

```cst
(source_file
  (let
    key: (ident)
    value: (def
      arg: (ident)
      body: (ident))
    in: (call
      func: (ident)
      arg: (bool))))
```

```ast
Some(
    Let(
        Some(
            Var(f),
        ),
        Some(
            Def(
                Some(
                    Var(a),
                ),
                Some(
                    a,
                ),
            ),
        ),
        Some(
            Call(
                Some(
                    f,
                ),
                Some(
                    Bool(
                        true,
                    ),
                ),
            ),
        ),
    ),
)
```

```ir
Some(
    Let(
        Some(
            VarDef(f, VarId(0)),
        ),
        Some(
            Def(
                Some(
                    VarDef(a, VarId(1)),
                ),
                Some(
                    Var(a, Some(VarId(1))),
                ),
            ),
        ),
        Some(
            Call(
                Some(
                    Var(f, Some(VarId(0))),
                ),
                Some(
                    Bool(
                        true,
                    ),
                ),
            ),
        ),
    ),
)
```

```type
Bool
```

```eval
Bool(
    true,
)
```

# Currying

```
let f = a: b: c: a;
f true false
```

```cst
(source_file
  (let
    key: (ident)
    value: (def
      arg: (ident)
      body: (def
        arg: (ident)
        body: (def
          arg: (ident)
          body: (ident))))
    in: (call
      func: (call
        func: (ident)
        arg: (bool))
      arg: (bool))))
```

```ast
Some(
    Let(
        Some(
            Var(f),
        ),
        Some(
            Def(
                Some(
                    Var(a),
                ),
                Some(
                    Def(
                        Some(
                            Var(b),
                        ),
                        Some(
                            Def(
                                Some(
                                    Var(c),
                                ),
                                Some(
                                    a,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
        Some(
            Call(
                Some(
                    Call(
                        Some(
                            f,
                        ),
                        Some(
                            Bool(
                                true,
                            ),
                        ),
                    ),
                ),
                Some(
                    Bool(
                        false,
                    ),
                ),
            ),
        ),
    ),
)
```

```ir
Some(
    Let(
        Some(
            VarDef(f, VarId(0)),
        ),
        Some(
            Def(
                Some(
                    VarDef(a, VarId(1)),
                ),
                Some(
                    Def(
                        Some(
                            VarDef(b, VarId(2)),
                        ),
                        Some(
                            Def(
                                Some(
                                    VarDef(c, VarId(3)),
                                ),
                                Some(
                                    Var(a, Some(VarId(1))),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
        Some(
            Call(
                Some(
                    Call(
                        Some(
                            Var(f, Some(VarId(0))),
                        ),
                        Some(
                            Bool(
                                true,
                            ),
                        ),
                    ),
                ),
                Some(
                    Bool(
                        false,
                    ),
                ),
            ),
        ),
    ),
)
```

```type
Fn(
    T3,
    Bool,
)
```

```eval
Fn(
    "c",
    InternId(
        3,
    ),
    $e4,
    RunEnv {
        scope: Some(
            Scope {
                name: InternId(
                    2,
                ),
                value: RefCell {
                    value: core::mem::maybe_uninit::MaybeUninit<lambda::runtime::Value>,
                },
                parent: Some(
                    Scope {
                        name: InternId(
                            1,
                        ),
                        value: RefCell {
                            value: core::mem::maybe_uninit::MaybeUninit<lambda::runtime::Value>,
                        },
                        parent: Some(
                            Scope {
                                name: InternId(
                                    0,
                                ),
                                value: RefCell {
                                    value: core::mem::maybe_uninit::MaybeUninit<lambda::runtime::Value>,
                                },
                                parent: None,
                            },
                        ),
                    },
                ),
            },
        ),
    },
)
```
