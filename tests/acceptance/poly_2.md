```
let f = a: b: b;
f
```


```type
Poly(
    [
        T0,
        T1,
    ],
    Fn(
        T0,
        Fn(
            T1,
            T1,
        ),
    ),
)
```

```diagnostics
```

```eval
Fn(
    "a",
    InternId(
        1,
    ),
    $e4,
    RunEnv {
        scope: Some(
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
)
```