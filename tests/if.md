```
if true then false else true
```

```cst
(source_file
  (ifElse
    cond: (bool)
    then: (bool)
    else: (bool)))
```

```ast
Some(
    IfElse(
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
        Some(
            Bool(
                true,
            ),
        ),
    ),
)
```

```ir
Some(
    IfElse(
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
        Some(
            Bool(
                true,
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

# When the condition is not boolean:

```
if f: f then false else true
```

```diagnostics
[31mError:[0m Could not unify Fn(T0, T0) != Bool
   [38;5;246m╭[0m[38;5;246m─[0m[38;5;246m[[0mtest:1:4[38;5;246m][0m
   [38;5;246m│[0m
 [38;5;246m1 │[0m [38;5;249mi[0m[38;5;249mf[0m[38;5;249m [0m[31mf[0m[31m:[0m[31m [0m[31mf[0m[38;5;249m [0m[38;5;249mt[0m[38;5;249mh[0m[38;5;249me[0m[38;5;249mn[0m[38;5;249m [0m[38;5;249mf[0m[38;5;249ma[0m[38;5;249ml[0m[38;5;249ms[0m[38;5;249me[0m[38;5;249m [0m[38;5;249me[0m[38;5;249ml[0m[38;5;249ms[0m[38;5;249me[0m[38;5;249m [0m[38;5;249mt[0m[38;5;249mr[0m[38;5;249mu[0m[38;5;249me[0m
[38;5;246m───╯[0m

```

```type
Bool
```

```eval
<No eval, errors found>
```

# When branches are not the same type

```
if true then f: f else false
```

```diagnostics
[31mError:[0m Could not unify Bool != Fn(T0, T0)
   [38;5;246m╭[0m[38;5;246m─[0m[38;5;246m[[0mtest:1:24[38;5;246m][0m
   [38;5;246m│[0m
 [38;5;246m1 │[0m [38;5;249mi[0m[38;5;249mf[0m[38;5;249m [0m[38;5;249mt[0m[38;5;249mr[0m[38;5;249mu[0m[38;5;249me[0m[38;5;249m [0m[38;5;249mt[0m[38;5;249mh[0m[38;5;249me[0m[38;5;249mn[0m[38;5;249m [0m[38;5;249mf[0m[38;5;249m:[0m[38;5;249m [0m[38;5;249mf[0m[38;5;249m [0m[38;5;249me[0m[38;5;249ml[0m[38;5;249ms[0m[38;5;249me[0m[38;5;249m [0m[31mf[0m[31ma[0m[31ml[0m[31ms[0m[31me[0m
[38;5;246m───╯[0m

```

```type
Fn(
    T0,
    T0,
)
```

```eval
<No eval, errors found>
```