# Implementando Models com Rust

Nesta sessão eu mostro as inúmeras formas de implementar Models para os dados com **Structs**  
Apresento alguns conceitos como **composição** e explico porque Rust não é uma POO - Programação Orientada a Objetos, e as implicações disso.

Em seguida eu introduzo conceitos mais avançados, como Traits, Enuns e Macros.

---

Para começar, o que é um Model? (Modelo)

Um Model é uma representação para uma estrutura de dados, também conhecido como uma Entidade ou uma Classe em POO.
Outras variações para o termo Model que você irá encontrar

Model
Classe
Entidade
Record
Documento
Row

---

Porque criamos Models?

Models representam os dados do mundo real, por exemplo podemos modelar um servidor rest com os seguintes atributos

```Rust
struct Req {
  host: String,
  porta: i32,
  parametros: vec!<String>
}
```

Em Rust Models são fortemente tipados, e possuem apenas os dados de uma regra de negócios. (Não possui lógica, comportamento, etc)

No entanto podemos extender nosssos modelos para admitir comportamento, por meio da **implementação de traits**

---
# Implementação de Models

# Implementação de traits

```Rust
exemplo trait1
```

Um trait pode ser inicialmente comparado com uma interface. Isso porque os traits contém contratos que uma determinada struct poderá implementar.
No entanto há uma série de diferenças que tornam traits um tanto mais interessantes.

## Default

Traits admitem implementações padrão  


