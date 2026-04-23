# Estudo de Caso A1 — FastBite

Resolução da atividade de Estruturas de Dados e Análise de Algoritmos.

---

## Questão 1

### a)
O problema da FastBite pode ser considerado NP-Completo (ou NP-difícil), pois é parecido com o TSP e o VRP, que são problemas conhecidos por serem difíceis.

Ele não está em P porque não existe solução eficiente conhecida. Porém, se alguém der uma solução pronta, dá pra verificar em tempo razoável, então está em NP.

Como ele é uma variação do TSP, acaba sendo NP-Completo.

---

### b)
Se simplificar o problema para apenas um entregador e ignorar algumas restrições, dá pra transformar em um TSP.

Cada pedido vira dois pontos (restaurante e cliente), e o objetivo passa a ser encontrar a menor rota que passa por todos.

---

### c)
Força bruta não funciona porque o número de combinações cresce muito rápido.

Exemplo:
3 pedidos = 6 pontos  
6! = 720 possibilidades

Com vários entregadores e mais pedidos, isso explode. A complexidade fica próxima de O(n!), então não dá pra usar em tempo real.

---

## Questão 2

### a)
O algoritmo funciona assim:
- pega um pedido
- escolhe o entregador mais próximo
- repete até acabar os pedidos

Depois monta a rota sempre indo para o ponto mais próximo.

---

### b)
Ele é guloso porque sempre faz a melhor escolha naquele momento (menor distância), sem pensar no resultado final.

---

### c)
Pode dar erro em casos onde um entregador mais longe seria melhor no conjunto geral.

O algoritmo pode escolher o mais próximo agora, mas gerar uma rota pior no final.

---

### d)
Para cada pedido (n), ele verifica todos os entregadores (m):

O(n × m)

A rota também tem custo, mas no geral ainda é rápido.

---

## Questão 3

### a)
Programação Dinâmica pode ser usada, mas fica muito pesada.

Complexidade:
O(2^k × k²)

Funciona para poucos pedidos, mas não em tempo real.

---

### b)
Dividir a cidade em regiões ajuda, mas não é perfeito.

Problema: pedidos perto da fronteira podem ser melhor atendidos por outra região.

---

## Questão 4

| Critério | Greedy | PD | Divisão |
|----------|--------|----|---------|
| Qualidade | Média | Ótima | Boa |
| Tempo | Baixo | Alto | Médio |
| Tempo real | Sim | Não | Depende |

No geral, greedy é o mais viável.

---

## Questão 5

### a)
Heurística é uma forma de encontrar uma solução boa sem precisar ser perfeita.

---

### b)
Uma solução real poderia ser:
- dividir por região  
- usar greedy  
- melhorar um pouco depois  
- parar quando bater o tempo limite  

---

### c)
Vale buscar solução ótima quando o problema é pequeno, tipo poucos pedidos.

---

## Questão 6

Nem sempre vale a pena buscar a solução perfeita. Em sistemas grandes, o mais importante é ser rápido e eficiente.

Uma solução boa e rápida muitas vezes é melhor do que uma perfeita que demora demais.
