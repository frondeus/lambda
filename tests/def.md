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
Def(
    Var(a),
    a,
)
```

```ir
Def(
    VarDef(a, VarId(0)),
    Var(a, Some(VarId(0))),
)
```

```type
Ok(
    Fn(
        T0,
        T0,
    ),
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
