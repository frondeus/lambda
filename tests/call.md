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
Let(
    Var(f),
    Def(
        Var(a),
        a,
    ),
    Call(
        f,
        Bool(
            true,
        ),
    ),
)
```

```ir
Let(
    VarDef(f, VarId(0)),
    Def(
        VarDef(a, VarId(1)),
        Var(a, Some(VarId(1))),
    ),
    Call(
        Var(f, Some(VarId(0))),
        Bool(
            true,
        ),
    ),
)
```

```type
Ok(
    Bool,
)
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
Let(
    Var(f),
    Def(
        Var(a),
        Def(
            Var(b),
            Def(
                Var(c),
                a,
            ),
        ),
    ),
    Call(
        Call(
            f,
            Bool(
                true,
            ),
        ),
        Bool(
            false,
        ),
    ),
)
```

```ir
Let(
    VarDef(f, VarId(0)),
    Def(
        VarDef(a, VarId(1)),
        Def(
            VarDef(b, VarId(2)),
            Def(
                VarDef(c, VarId(3)),
                Var(a, Some(VarId(1))),
            ),
        ),
    ),
    Call(
        Call(
            Var(f, Some(VarId(0))),
            Bool(
                true,
            ),
        ),
        Bool(
            false,
        ),
    ),
)
```

```type
Ok(
    Fn(
        T3,
        Bool,
    ),
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
