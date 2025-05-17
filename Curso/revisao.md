---
marp: true
math: katex
theme: default
class:
  - invert

---

## 🔘 Lógica binária (0️⃣ 1️⃣)

- Boolean values: `true` or `false`  
- 1 = Verdadeiro, 0 = Falso  

### 🔍 Tabela verdade E AND (`&&`)

| A   | B   | A AND B |
| --- | --- | ------- |
| 0   | 0   | 0       |
| 0   | 1   | 0       |
| 1   | 0   | 0       |
| 1   | 1   | 1       |

---

## Tabela verdade OU OR (`||`)

| A   | B   | A OR B |
| --- | --- | ------ |
| 0   | 0   | 0      |
| 0   | 1   | 1      |
| 1   | 0   | 1      |
| 1   | 1   | 1      |


---

## Potência de 2

Binary grows in powers of 2:

| n   | 2ⁿ  |
| --- | --- |
| 0   | 1   |
| 1   | 2   |
| 2   | 4   |
| 3   | 8   |
| 4   | 16  |
| 5   | 32  |
| 6   | 64  |
| 7   | 128 |

💡 Computadores usam potência de 2, porque é possível converter qualquer número natual em um número binário facilmente usando esta técnica

---

## 🔄 Decimal to Binary

Converta `13` em binário:

$$ 13\ /\ 2\ =\ 6\ sobra\ 1\ $$  
$$ 6\ /\ 2\ =\ 3\ sobra\ 0\ $$  
$$ 3\ /\ 2\ =\ 1\ sobra\ 1\ $$  
$$ 1\ /\ 2\ =\ 0\ sobra\ 1\ $$  

--- 

## 🧮 Hexadecimal (Base 16)

Digits: `0 1 2 3 4 5 6 7 8 9 A B C D E F`  
Each hex digit = 4 bits (binary)

### Example:

- Binary: `1111 0001`  
- Hex: `F1`

---

## Fatoração, Resto da divisão

Problema do troco para notas de 50, 20, 10, 5, 2

$$r = a - \lfloor \frac{a}{b} \rfloor* b $$  

| divisor(a) | dividendo(b) | quociente(q) | resto(r) |
| ---------- | ------------ | ------------ | -------- |
| 187        | 50           | 3            | 37       |
| 37         | 20           | 1            | 17       |
| 17         | 10           | 1            | 7        |
| 7          | 5            | 1            | 2        |
| 2          | 2            | 1            | 0        |

---

# Matrizes

$
 \left[\begin{matrix}
  1 & 2 & 3\\
  4 & 5 & 6\\
  7 & 8 & 9
\end{matrix}\right]$

$$A_{i⨉j}$$
$$ i = j = 3$$
---

# Funções

$f(x) = 3x-1$

|x|fx|
|-|-|
|2|5|
|3|8|
|...|
|6|17|


--- 
