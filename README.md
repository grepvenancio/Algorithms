# Introdução

Esse repositório contem uma série de algoritmos e estrutura de dados implementadas em rust.

## O que é big O?

O big O é uma forma de classificar algoritimos baseados no tempo e memória
necessária durante sua execução, dependendo do input.

↓---- Ordem de complexidade -> Pode ser em relação a tempo ou espaço
O (N)
   ^--------- Quantidade de input

É uma forma generica de classificar a "complexidade" de uma algoritimo.
Ou seja quanto maior a quantidade de imput, qual é a proporção do crescimento
computacional e nesseciadade de memoria?

A função abaixo tem um crescimento linear, que pode ser traduzida para uma
função de complexidade **O (N)**
Isso pode ser facilmente observado pelo loop no corpo da função, pois cada
char a mais no input vai ser mais uma iteração no loop com a complexidade de
tempo crescendo de forma linear

```rust
fn sum_char_code_on(n: &str) -> usize {
    let mut sum = 0;
    for (index, _char) in n.chars().enumerate() {
        sum += index
    }
    return sum;
}
```
## O (N²)

A complexidade do tempo nessa função vai crescer na forma de N², ou seja um
input equivalente a 5 vai ter uma complexidade de 5²

```rust
fn sum_char_code_onsquare2(n: &str) -> usize {
    let mut sum = 0;
    for (_index, _char) in n.chars().enumerate() {
        for (index, _char) in n.chars().enumerate() {
            sum += index
        }
    }
    return sum;
}
```
