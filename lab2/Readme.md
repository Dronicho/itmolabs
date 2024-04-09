# Лабораторная работа №2

## Задание
Простой http-сервер. Имеется форма регистрации и логина. Также вохможна отправка сообщений от одного пользователя другому. Доступен список всех сообщений, а также их просмотр. Сообщения хранятся в базе (например sqlite)

## Решение
Проект разбит на 2 папки:

- `client` - Фронтенд для работы с сервером
- `server` - http-server

### Stack

Для фронта использовался [React](https://react.dev/)

Для сервера [rust](https://www.rust-lang.org/) + [actix_web](https://actix.rs/) в качестве веб фреймворка

База данных - sqlite

### Getting Started
Для локального запуска клиента необходимо установить [rust](https://www.rust-lang.org/) и [node](https://nodejs.org/en)

> Я использую pnpm для управлением зависимостями

#### Frontend
```bash
cd client
npm install
npm run dev
```
сайт запущен на http://locahost:5137/

#### Server
```bash
cd server
cargo run
```

### Auth
В проекте используется session-based авторизация

### Testing
В качестве чекера написал юнит тесты для сервера

Для проверки

```bash
cd server
cargo test
// Все тесты пройдены :)
```

### Docker
В корне лежит `docker-compose.yml` для запуска клиента и сервера