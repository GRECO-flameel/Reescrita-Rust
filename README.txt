# Atividade Rust — Estruturas de Dados

## Exercício 1 — Verificar Primeiro
**Complexidade:** O(1)

**Lógica do algoritmo:**
Retorna o primeiro elemento da lista ou None caso esteja vazia.

**Justificativa:**
Não há loops. O acesso ao índice 0 é direto e independe do tamanho da entrada.

---

## Exercício 2 — Somar Lista
**Complexidade:** O(n)

**Lógica do algoritmo:**
Percorre todos os elementos somando seus valores.

**Justificativa:**
Um único loop percorre n elementos.

---

## Exercício 3 — Busca Binária
**Complexidade:** O(log n)

**Lógica do algoritmo:**
Divide o intervalo de busca ao meio a cada iteração.

**Justificativa:**
O espaço de busca é reduzido pela metade a cada passo.

---

## Exercício 4 — Pares com Soma
**Complexidade:** O(n²)

**Lógica do algoritmo:**
Verifica todos os pares possíveis da lista.

**Justificativa:**
Dois loops aninhados.

---

## Exercício 5 — Imprimir Pares e Pares
**Complexidade:** O(n²)

**Lógica do algoritmo:**
Primeiro imprime elementos, depois todos os pares possíveis.

**Justificativa:**
O(n) + O(n²) → O(n²)

---

## Exercício 6 — Potências de Dois
**Complexidade:** O(log n)

**Lógica do algoritmo:**
Multiplica por 2 até atingir n.

**Justificativa:**
O valor cresce exponencialmente.

---

## Exercício 7 — Fibonacci Recursivo
**Complexidade:** O(2ⁿ)

**Lógica do algoritmo:**
Calcula Fibonacci recursivamente.

**Justificativa:**
Recalcula subproblemas repetidamente.

---

## Exercício 8 — Bubble Sort
**Complexidade:** O(n²)

**Lógica do algoritmo:**
Compara e troca elementos adjacentes.

**Justificativa:**
Dois loops aninhados.

---

## Exercício 9 — Produto de Matrizes
**Complexidade:** O(n³)

**Lógica do algoritmo:**
Multiplica duas matrizes.

**Justificativa:**
Três loops aninhados.

---

## Exercício 10 — Merge Sort
**Complexidade:** O(n log n)

**Lógica do algoritmo:**
Divide a lista e mescla ordenando.

**Justificativa:**
Divide em log n níveis e processa n elementos por nível.