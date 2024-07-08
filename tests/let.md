```
let x = true; false
```

```cst
(source_file
  (let
    key: (ident)
    value: (bool)
    in: (bool)))
```

```ast
Let(
    Var(x),
    Bool(
        true,
    ),
    Bool(
        false,
    ),
)
```

```ir
Let(
    VarDef(x, VarId(0)),
    Bool(
        true,
    ),
    Bool(
        false,
    ),
)
```

```type
Bool
```

```eval
Bool(
    false,
)
```

# With whitespace

```
let x = true;   


 false
```

```cst
(source_file
  (let
    key: (ident)
    value: (bool)
    in: (bool)))
```

```ast
Let(
    Var(x),
    Bool(
        true,
    ),
    Bool(
        false,
    ),
)
```

```ir
Let(
    VarDef(x, VarId(0)),
    Bool(
        true,
    ),
    Bool(
        false,
    ),
)
```

```type
Bool
```

```eval
Bool(
    false,
)
```
