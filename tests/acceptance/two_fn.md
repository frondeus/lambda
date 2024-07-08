```
let f = a: a;
let g = a true;
let h = x: y: x;
h (f true) (g true)
```


```type
Bool
```

`````diagnostics
[31mError:[0m Variable `a` is not defined anywhere
   [38;5;246m╭[0m[38;5;246m─[0m[38;5;246m[[0mtest:2:9[38;5;246m][0m
   [38;5;246m│[0m
 [38;5;246m2 │[0m [38;5;249ml[0m[38;5;249me[0m[38;5;249mt[0m[38;5;249m [0m[38;5;249mg[0m[38;5;249m [0m[38;5;249m=[0m[38;5;249m [0m[31ma[0m[38;5;249m [0m[38;5;249mt[0m[38;5;249mr[0m[38;5;249mu[0m[38;5;249me[0m[38;5;249m;[0m
[38;5;246m───╯[0m

`````

```eval
<No eval, errors found>
```
