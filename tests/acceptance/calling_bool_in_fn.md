```
let call = x: (x true);
call false
```

```type
T2
```

```diagnostics
[31mError:[0m Could not unify Fn(Bool, T1) != Bool
   [38;5;246m╭[0m[38;5;246m─[0m[38;5;246m[[0mtest:2:1[38;5;246m][0m
   [38;5;246m│[0m
 [38;5;246m2 │[0m [31mc[0m[31ma[0m[31ml[0m[31ml[0m[38;5;249m [0m[38;5;249mf[0m[38;5;249ma[0m[38;5;249ml[0m[38;5;249ms[0m[38;5;249me[0m
[38;5;246m───╯[0m

```

```eval
<No eval, errors found>
```