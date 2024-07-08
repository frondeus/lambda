```
let f = y: x: y;
f true
```

```type
Fn(
    T2,
    Bool,
)
```

```diagnostics
```

```eval
Fn(
    "x",
    InternId(
        2,
    ),
    $e3,
    RunEnv {
        scope: Some(
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
)
```