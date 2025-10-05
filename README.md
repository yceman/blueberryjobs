# blueberryjobs
Applicant Tracking System

https://github.com/yceman/blueberryjobs.git

```
my_ats/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── models/         # Definições de structs para seus dados
│   ├── controllers/    # Lógica para lidar com as requisições (handlers)
│   ├── services/       # Lógica de negócios
│   ├── repositories/   # Lógica de acesso ao banco de dados
│   ├── routes/         # Definição das rotas da API
│   ├── database.rs     # Configuração e conexão com o banco de dados
│   └── utils/          # Módulos utilitários
└── tests/            # Arquivos de teste
```
Create new project

```
cargo new my_project
cargo new blueberryjobs
```
or

```
cargo new blueberryjobs
```