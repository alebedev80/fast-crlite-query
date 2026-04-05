#!/bin/bash

# Пути
DB_DIR="/root/crlite/rust-query-crlite/crlite_db"
UPDATR_BIN="/root/crlite/rust-query-crlite/target/release/rust-query-crlite"

# Создаем папку, если ее нет
mkdir -p "$DB_DIR"

# 1. Скачиваем обновления
# Мы используем команду 'help', чтобы утилита просто выполнила цикл обновления и вышла
$UPDATR_BIN --db "$DB_DIR" --update prod help > /dev/null 2>&1

# 2. Проверяем результат
if [ $? -eq 0 ]; then
    echo "[$(date)] CRLite DB updated successfully in $DB_DIR"

    # Опционально: удаляем старые файлы фильтров, которые старше 30 дней,
    # чтобы папка не раздувалась бесконечно
    find "$DB_DIR" -name "*.delta" -mtime +30 -delete
else
    echo "[$(date)] CRLite DB update failed!"
    exit 1
fi
