```
a: a
```

```cst
(source_file
  (def
    arg: (ident)
    body: (ident)))
```

```ast
Some(
    Def(
        Some(
            Var(a),
        ),
        Some(
            a,
        ),
    ),
)
```

```ir
Some(
    Def(
        Some(
            VarDef(a, VarId(0)),
        ),
        Some(
            Var(a, Some(VarId(0))),
        ),
    ),
)
```

```type
Fn(
    T0,
    T0,
)
```

```eval
Fn(
    "a",
    InternId(
        0,
    ),
    $e1,
    RunEnv {
        scope: None,
    },
)
```
