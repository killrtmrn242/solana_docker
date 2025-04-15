# Solana Hello World Program (Rust)

## 📚 Описание

Это пример смарт-контракта (программы) на Rust, развернутого в блокчейне Solana в сети `devnet`. Контракт написан с использованием фреймворка `solana-program`, собран с помощью `cargo build-bpf` и задеплоен через Solana CLI.

---

## 📦 Стек технологий

- 🦀 Rust (solana-program)
- ⚙️ Solana CLI
- 🐳 Docker
- 🐧 Ubuntu (в Docker контейнере)
- 🔁 Devnet (тестовая сеть Solana)

---

## 🚀 Быстрый старт

### 1. Клонирование репозитория

```bash
git clone https://github.com/solana-labs/example-helloworld
cd example-helloworld
```

---

### 2. Запуск среды (Docker)

```bash
docker run -it --name solana-dev -v $PWD:/app -w /app ubuntu:20.04 bash
```

> Или используйте `docker-compose` при наличии конфигурации.

---

### 3. Установка зависимостей внутри контейнера

```bash
apt update && apt install -y curl build-essential git pkg-config libssl-dev libudev-dev
```

---

### 4. Установка Rust и BPF Toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

rustup update
rustup install stable
rustup component add rust-src
rustup target add bpfel-unknown-unknown

cargo install --git https://github.com/solana-labs/solana --tag v1.17.28 solana-cli
```

---

### 5. Настройка Solana CLI

```bash
solana config set --url https://api.devnet.solana.com
solana-keygen new --outfile /root/.config/solana/devnet.json
solana config set --keypair /root/.config/solana/devnet.json
```

---

### 6. Получение SOL токенов

```bash
solana airdrop 2
```

---

### 7. Сборка смарт-контракта

```bash
cd helloworld
cargo build-bpf
```

После сборки появится файл:
```bash
target/deploy/helloworld.so
```

---

### 8. Деплой контракта

#### Создание нового Program ID

```bash
solana-keygen new --outfile target/deploy/helloworld-keypair.json
```

#### Деплой в devnet

```bash
solana program deploy target/deploy/helloworld.so --keypair target/deploy/helloworld-keypair.json
```

Вывод покажет:
```bash
Program Id: 6Y9...xyz
```

---

## 📌 Проверка статуса программы

```bash
solana program show <PROGRAM_ID>
```

Пример:
```bash
solana program show 6Y9...xyz
```

---

## 🔁 Как восстановить среду после перезапуска

1. Запустить Docker:
```bash
docker start -ai solana-dev
```

2. Перейти в директорию с проектом:
```bash
cd /app/helloworld
```

3. Проверить статус:
```bash
solana program show <PROGRAM_ID>
```

4. При необходимости повторно собрать:
```bash
cargo build-bpf
```

---

## 📌 Полезные команды

| Команда | Описание |
|--------|----------|
| `solana --version` | Проверить версию |
| `solana balance` | Проверить баланс |
| `solana config get` | Проверить текущую конфигурацию |
| `solana program show <PROGRAM_ID>` | Информация о загруженной программе |
| `cargo build-bpf` | Сборка программы |
| `solana program deploy ...` | Деплой программы |

