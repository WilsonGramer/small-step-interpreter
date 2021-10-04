# Small-step interpreter

A little interpreter that uses [small-step evaluation](https://en.wikipedia.org/wiki/Operational_semantics#Reduction_semantics). The provided example adds two numbers using a function:

```
(let f = (a -> (b -> (a + b))) in ((f 1) 2))
(((a -> (b -> (a + b))) 1) 2)
((<function> 1) 2)
((b -> (1 + b)) 2)
(<function> 2)
(1 + 2)
3
```
