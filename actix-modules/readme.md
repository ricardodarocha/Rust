# Actix Modules

## Intro

🐺 Este é um exemplo básico sobre **criação de APIS com o RUST**
Neste exemplo eu mostro as seguintes features  
- Criação de módulos
- Uuid
- Chrono

Partindo do exemplo actix 2 (ver diretório rust/actix) nós fazemos algumas refatorações para quebrar o projeto em módulos  
Instale a linha de comando cargo mod para lhe auxiliar  
```
cargo install cargo-mod
cargo mod api
cargo mod models
cargo mod repo
```

### Hierarquia de diretórios

Neste exemplo nós dividimos a aplicação em três camadas começando pela camada de aplicação **api**. Esta camada também é comumente chamada **app** a depender do propósito da aplicação. Por exemplo, 
sendo um servidor de páginas html com várias rotas para renderizar uma aplicação vue-js, react ou svelte, ou uma aplicação com Interface do usuário, poderia ser chamado app.
Ainda em se tratando de uma aplicação de linha de comando, cli, poderia ser chamado **cli** etc. O importante é que esta seja a _camada da aplicação_, ou seja, o nível mais próximo do usuário, a interface da aplicação.

A pasta **models** é o core, ou domínio da aplicação, onde ficarão as regras de negócios junto com os modelos (aggregates, entidades, value objects)
Esta pasta pode ou não ser divididas em subpastas por exemplo
```
models
  types
  interfaces
  entity
  valueobjecs
  dto
```

A pasta **repo** o **repository** é onde ficam as implementações do repositório, seja uma layer mock, uma camada de persistência para múltiplos bancos de dados ou um sistema de arquivos

### 
