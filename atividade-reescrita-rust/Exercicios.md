# Atividade - Análise de Algoritmos (Rust)

---

## Exercício 1 — Verificar Primeiro

**Complexidade:** O(1)

**Lógica do algoritmo:**
Só retorna o primeiro valor da lista, se ela tiver algum elemento.

**Justificativa da complexidade:**
Ele não percorre a lista, só olha direto o primeiro item, então tanto faz o tamanho.

```rust
pub fn verificar_primeiro(lista: &[i32]) -> Option<i32> {
    if lista.is_empty() {
        None
    } else {
        Some(lista[0])
    }
}
```

---

## Exercício 2 — Somar Lista

**Complexidade:** O(n)

**Lógica do algoritmo:**
Vai passando pela lista e somando tudo.

**Justificativa da complexidade:**
Precisa olhar elemento por elemento, então quanto maior a lista, mais tempo leva.

```rust
pub fn somar_lista(lista: &[i32]) -> i32 {
    let mut total = 0;

    for &num in lista {
        total += num;
    }

    total
}
```

---

## Exercício 3 — Busca Binária

**Complexidade:** O(log n)

**Lógica do algoritmo:**
Sempre pega o meio da lista e decide se vai pra esquerda ou direita.

**Justificativa da complexidade:**
Cada vez corta metade da lista, então não cresce muito rápido.

```rust
pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    let mut esquerda: isize = 0;
    let mut direita: isize = lista.len() as isize - 1;

    while esquerda <= direita {
        let meio = (esquerda + direita) / 2;
        let idx = meio as usize;

        if lista[idx] == alvo {
            return Some(idx);
        } else if lista[idx] < alvo {
            esquerda = meio + 1;
        } else {
            direita = meio - 1;
        }
    }

    None
}
```

---

## Exercício 4 — Pares com Soma

**Complexidade:** O(n²)

**Lógica do algoritmo:**
Testa todas as combinações de dois números da lista.

**Justificativa da complexidade:**
Tem um loop dentro do outro, então acaba comparando muita coisa.

```rust
pub fn pares_com_soma(lista: &[i32], alvo: i32) {
    let n = lista.len();

    for i in 0..n {
        for j in (i + 1)..n {
            if lista[i] + lista[j] == alvo {
                println!("{} + {} = {}", lista[i], lista[j], alvo);
            }
        }
    }
}
```

---

## Exercício 5 — Imprimir Pares e Pares

**Complexidade:** O(n²)

**Lógica do algoritmo:**
Primeiro só mostra os valores, depois mostra todos os pares possíveis.

**Justificativa da complexidade:**
Mesmo tendo uma parte simples, o segundo bloco domina porque tem dois loops.

```rust
pub fn imprimir_pares_e_pares(lista: &[i32]) {
    for &x in lista {
        println!("{}", x);
    }

    for &x in lista {
        for &y in lista {
            println!("({}, {})", x, y);
        }
    }
}
```

---

## Exercício 6 — Potências de Dois

**Complexidade:** O(log n)

**Lógica do algoritmo:**
Começa em 1 e vai dobrando o valor.

**Justificativa da complexidade:**
Como cresce rápido (dobrando), não precisa de muitas repetições.

```rust
pub fn potencias_de_dois(n: u64) {
    let mut i = 1;

    while i < n {
        println!("{}", i);
        i *= 2;
    }
}
```

---

## Exercício 7 — Fibonacci Recursivo

**Complexidade:** O(2^n)

**Lógica do algoritmo:**
Chama ele mesmo várias vezes pra calcular os valores.

**Justificativa da complexidade:**
Repete muito cálculo, então cresce bem rápido o tempo.

```rust
pub fn fibonacci_recursivo(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
}
```

---

## Exercício 8 — Ordenação Bolha

**Complexidade:** O(n²)

**Lógica do algoritmo:**
Vai comparando os vizinhos e trocando de lugar.

**Justificativa da complexidade:**
Tem dois loops, então o tempo aumenta bastante conforme a lista cresce.

```rust
pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();

    for i in 0..n {
        for j in 0..(n - i - 1) {
            if lista[j] > lista[j + 1] {
                lista.swap(j, j + 1);
            }
        }
    }
}
```

---

## Exercício 9 — Produto de Matrizes

**Complexidade:** O(n³)

**Lógica do algoritmo:**
Multiplica duas matrizes usando três loops.

**Justificativa da complexidade:**
Cada posição depende de várias multiplicações, então fica bem pesado.

```rust
pub fn produto_de_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut c = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    c
}
```

---

## Exercício 10 — Merge Sort

**Complexidade:** O(n log n)

**Lógica do algoritmo:**
Divide a lista em partes menores e depois junta tudo ordenado.

**Justificativa da complexidade:**
Divide várias vezes e depois percorre pra juntar, por isso n log n.

```rust
pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
    if lista.len() <= 1 {
        return lista;
    }

    let meio = lista.len() / 2;

    let esquerda = merge_sort(lista[..meio].to_vec());
    let direita = merge_sort(lista[meio..].to_vec());

    merge(esquerda, direita)
}

fn merge(esq: Vec<i32>, dir: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::with_capacity(esq.len() + dir.len());

    let mut i = 0;
    let mut j = 0;

    while i < esq.len() && j < dir.len() {
        if esq[i] <= dir[j] {
            resultado.push(esq[i]);
            i += 1;
        } else {
            resultado.push(dir[j]);
            j += 1;
        }
    }

    resultado.extend_from_slice(&esq[i..]);
    resultado.extend_from_slice(&dir[j..]);

    resultado
}
```
