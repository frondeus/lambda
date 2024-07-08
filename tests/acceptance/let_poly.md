```
let id = x: x;
let h = a: b: a;
h (id id) (id true)
```


```type
Poly(
    [
        T0,
    ],
    Fn(
        T0,
        T0,
    ),
)
```

```diagnostics
```

```eval
Fn(
    "x",
    InternId(
        1,
    ),
    $e2,
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