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
Some(
    Let(
        Some(
            Var(x),
        ),
        Some(
            Bool(
                true,
            ),
        ),
        Some(
            Bool(
                false,
            ),
        ),
    ),
)
```

```ir
Some(
    Let(
        Some(
            VarDef(x, VarId(0)),
        ),
        Some(
            Bool(
                true,
            ),
        ),
        Some(
            Bool(
                false,
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
Some(
    Let(
        Some(
            Var(x),
        ),
        Some(
            Bool(
                true,
            ),
        ),
        Some(
            Bool(
                false,
            ),
        ),
    ),
)
```

```ir
Some(
    Let(
        Some(
            VarDef(x, VarId(0)),
        ),
        Some(
            Bool(
                true,
            ),
        ),
        Some(
            Bool(
                false,
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
    false,
)
```
