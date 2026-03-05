# Exercícios – Análise de Complexidade Assintótica (Big-O)

## Exercício 1

```python
def verificar_primeiro(lista):
    if len(lista) == 0:
        return None
    return lista[0]
```

Complexidade: O(1)

Justificativa:  
O algoritmo só verifica se a lista está vazia e retorna o primeiro elemento. Não importa se a lista tem 1 ou mil elementos, ele sempre faz praticamente a mesma coisa.

---

## Exercício 2

```python
def somar_lista(lista):
    total = 0
    for elemento in lista:
        total += elemento
    return total
```

Complexidade: O(n)

Justificativa:  
Aqui o algoritmo precisa percorrer todos os elementos da lista para fazer a soma. Então quanto maior a lista, mais vezes o loop vai rodar.

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

Complexidade: O(log n)

Justificativa:  
A busca binária sempre divide a lista pela metade a cada passo. Então o número de verificações cresce bem devagar em relação ao tamanho da lista.

---

## Exercício 4

```python
def pares_com_soma(lista, alvo):
    for i in range(len(lista)):
        for j in range(i + 1, len(lista)):
            if lista[i] + lista[j] == alvo:
                print(lista[i], lista[j])
```

Complexidade: O(n²)

Justificativa:  
Tem dois loops um dentro do outro verificando combinações de elementos. Por causa disso o número de comparações cresce bastante quando a lista aumenta.

---

## Exercício 5

```python
def imprimir_pares_e_soma(lista):

    # Bloco 1
    for i in range(len(lista)):
        print(lista[i])

    # Bloco 2
    for i in range(len(lista)):
        for j in range(len(lista)):
            print(lista[i], lista[j])
```

Complexidade: O(n²)

Justificativa:  
O primeiro loop percorre a lista uma vez. Já o segundo tem dois loops juntos, então ele cresce bem mais rápido. No final o que mais pesa é o O(n²).

---

## Exercício 6

```python
def potencias_de_dois(n):
    i = 1
    while i < n:
        print(i)
        i *= 2
```

Complexidade: O(log n)

Justificativa:  
A cada repetição o valor de i dobra. Então não cresce de forma linear, e sim mais devagar, até chegar perto de n.

---

## Exercício 7

```python
def fibonacci_recursivo(n):
    if n <= 1:
        return n
    return fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2)
```

Complexidade: O(2ⁿ)

Justificativa:  
Cada chamada da função acaba gerando duas outras chamadas. Por isso o número de chamadas cresce muito rápido conforme o valor de n aumenta.

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

Complexidade: O(n²)

Justificativa:  
O Bubble Sort compara os elementos várias vezes usando dois loops. No pior caso ele acaba passando praticamente por todos os pares da lista.

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

Complexidade: O(n³)

Justificativa:  
Tem três loops um dentro do outro para fazer o cálculo da multiplicação das matrizes. Então o número de operações cresce em n × n × n.

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

Complexidade: O(n log n)

Justificativa:  
O Merge Sort divide a lista várias vezes até ficar bem pequena e depois junta tudo de novo ordenando. Por isso aparece a combinação de n com log n.
