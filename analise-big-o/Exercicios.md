# Exercícios – Análise de Complexidade Assintótica (Big-O)

## Exercício 1

```python
def verificar_primeiro(lista):
    if len(lista) == 0:
        return None
    return lista[0]
```

**Complexidade:** O(1)

**Justificativa:**  
O algoritmo apenas verifica se a lista está vazia e retorna o primeiro elemento. Não importa o tamanho da lista, o número de operações é sempre o mesmo.

---

## Exercício 2

```python
def somar_lista(lista):
    total = 0
    for elemento in lista:
        total += elemento
    return total
```

**Complexidade:** O(n)

**Justificativa:**  
O algoritmo percorre todos os elementos da lista para somá-los. Assim, o número de operações cresce proporcionalmente ao tamanho da lista.

---

## Exercício 3

```python
def busca_binaria(lista, alvo):
    esquerda, direita = 0, len(lista) - 1

    while esquerda <= direita:
        meio = (esquerda + direita) // 2

        if lista[meio] == alvo:
            return meio
        elif lista[meio] < alvo:
            esquerda = meio + 1
        else:
            direita = meio - 1

    return -1
```

**Complexidade:** O(log n)

**Justificativa:**  
A busca binária divide o intervalo de busca pela metade a cada iteração. Isso reduz rapidamente o espaço de busca, resultando em crescimento logarítmico.

---

## Exercício 4

```python
def pares_com_soma(lista, alvo):
    for i in range(len(lista)):
        for j in range(i + 1, len(lista)):
            if lista[i] + lista[j] == alvo:
                print(lista[i], lista[j])
```

**Complexidade:** O(n²)

**Justificativa:**  
O algoritmo utiliza dois loops aninhados para verificar todos os pares possíveis da lista. Assim, o número de comparações cresce proporcionalmente a n × n.

---

## Exercício 5

```python
def imprimir_pares_e_soma(lista):

    # Bloco 1: imprime cada elemento
    for i in range(len(lista)):
        print(lista[i])

    # Bloco 2: imprime todos os pares
    for i in range(len(lista)):
        for j in range(len(lista)):
            print(lista[i], lista[j])
```

**Complexidade:** O(n²)

**Justificativa:**  
O primeiro loop percorre a lista uma vez (O(n)), mas o segundo possui dois loops aninhados (O(n²)). Como O(n²) cresce mais rápido, ele domina a complexidade total.

---

## Exercício 6

```python
def potencias_de_dois(n):
    i = 1
    while i < n:
        print(i)
        i *= 2
```

**Complexidade:** O(log n)

**Justificativa:**  
A cada iteração o valor de i é multiplicado por 2. Isso faz com que o número de repetições cresça de forma logarítmica em relação ao valor de n.

---

## Exercício 7

```python
def fibonacci_recursivo(n):
    if n <= 1:
        return n
    return fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
```

**Complexidade:** O(2ⁿ)

**Justificativa:**  
Cada chamada da função gera duas novas chamadas recursivas. Isso faz com que o número de chamadas cresça exponencialmente conforme n aumenta.

---

## Exercício 8

```python
def ordenacao_bolha(lista):
    n = len(lista)

    for i in range(n):
        for j in range(0, n - i - 1):
            if lista[j] > lista[j + 1]:
                lista[j], lista[j + 1] = lista[j + 1], lista[j]

    return lista
```

**Complexidade:** O(n²)

**Justificativa:**  
O Bubble Sort utiliza dois loops aninhados para comparar e trocar elementos da lista. No pior caso, ele precisa comparar praticamente todos os pares.

---

## Exercício 9

```python
def produto_de_matrizes(A, B, n):
    C = [[0] * n for _ in range(n)]

    for i in range(n):
        for j in range(n):
            for k in range(n):
                C[i][j] += A[i][k] * B[k][j]

    return C
```

**Complexidade:** O(n³)

**Justificativa:**  
O algoritmo utiliza três loops aninhados para calcular o produto das matrizes. Cada loop percorre n posições, resultando em n × n × n operações.

---

## Exercício 10

```python
def merge_sort(lista):
    if len(lista) <= 1:
        return lista

    meio = len(lista) // 2

    esquerda = merge_sort(lista[:meio])
    direita = merge_sort(lista[meio:])

    resultado = []
    i = j = 0

    while i < len(esquerda) and j < len(direita):
        if esquerda[i] <= direita[j]:
            resultado.append(esquerda[i])
            i += 1
        else:
            resultado.append(direita[j])
            j += 1

    resultado.extend(esquerda[i:])
    resultado.extend(direita[j:])

    return resultado
```

**Complexidade:** O(n log n)

**Justificativa:**  
O Merge Sort divide a lista em duas partes repetidamente (log n divisões) e depois percorre os elementos para fazer a junção (n operações). Por isso a complexidade final é n log n.
