# Introdução à Arquiteutra de Projetos com Rust

Neste repositório eu demonstro como planejar a sua aplicação em diretórios, baseado na funcionalidade e na modularização

Alguns princípios que você deve estar atento ao elaborar uma Arquitetura de Projeto

- [x] Modularizção
- [x] Organização e Estrutura de Pastas 
- [x] Nomenclaturas claras e Identificação correta
- [x] Aplicação de Design Patterns

Se você organizar bem o seu código, você terá menos trabalho para encontrar os arquivos, para dar manutenção e reaproveitar as funcionalidades que você criou.

- [x] Reaproveitamento de código
- [x] Facilidade para encontrar o que deseja
- [x] Alta produtividade

## Criando uma estrutura de Projetos

Embora não se possa dizer que esta estrutura de pastas seja realmente eficiente em 100% dos casos, eu acho que é um excelente ponto de partida para você começar a estudar Rust e organizar sua aplicação em módulos
Se deseja um estudo mais profundo de como organizar sua própria estrutura, consulte o tópico Analisando minha estrutura de projetos

###  Analisando minha estrutura de projetos

 O primeiro passo para analisar a sua estrutura de projetos atual é instalando o pacote de análise cargo-modules
 
 ```Rust
cargo install cargo-modules
cargo modules generate tree
```

O comando `generate tree` irá produzir uma estrutura de pastas do seu projeto, como no exemplo
```
crate apipg
├── mod api: pub(crate)
│   ├── mod customer: pub
│   ├── mod routes: pub
│   └── mod user: pub
└── mod model: pub(crate)
    ├── mod models: pub
    ├── mod usermodel: pub
    └── mod customermodel: pub
```

o comando `cargo modules generate tree —with-types` **--with-types** irá gerar uma árvore mais detalhada, incluindo as Struct públicas e privadas da sua aplicação

```Rust
cargo modules generate tree —with-types
//output>
```

``` 
crate sqlxpg
├── mod api: pub(crate)
│   ├── mod book: pub
│   │   └── fn sample: pub(self)
│   ├── mod routes: pub
│   │   ├── fn ping: pub
│   │   ├── fn pong: pub
│   │   └── fn user: pub
│   └── mod user: pub
│       ├── fn create_user: pub
│       ├── fn delete_user: pub
│       └── fn update_user: pub
├── fn main: pub(crate)
└── mod model: pub(crate)
    ├── mod models: pub
    │   └── struct Bookstore: pub
    └── mod usermodel: pub
        └── struct User: pub
```

## Gerando Grafos 

Esta funcionalidade vai revolucionar a sua visão de arquitetura de projetos com Rust (E também com outras linguagens)  
Instale a aplicação para visualizar grafos [**Graphviz**](https://graphviz.org/download/) (Open Source, Free to download)  
![Grapho](grapho.png)

Após instalar você poderá executar o verificador utilizando o parametro

```Rust
cargo-modules generate graph --with-types --with-tests --with-orphans | "d:/programas/graphviz/dot.exe" -Tsvg > diagrama.svg
```
Observe que você precisa informar o local onde foi instalado o executável "dot.exe" e passar o parametro -Tsvg  
o parâmetro `> diagrama.svg` exporta o resultado para um arquivo .svg, que voce pode visualizar utilizando o Browser

Dica instale a extensão [**svg Viewer**](https://marketplace.visualstudio.com/items?itemName=cssho.vscode-svgviewer) para visualizar o grafo no VSCode.



