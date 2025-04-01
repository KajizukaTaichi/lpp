# Λ++
（読み方：ラムダプラスプラス）

高水準言語を純粋なラムダ計算にするやつ 

```
$ lpp "(1 + 3) * 5"
Lambda Calculus Formula: (((λm. (λn. (λf. (m (n f))))) (((λm. (λn. (λf. (λx. ((m f) ((n f) x)))))) (λf. (λx. (f x)))) (λf. (λx. (f (f (f x))))))) (λf. (λx. (f (f (f (f (f x))))))))
Evaluated Result: 20
```
