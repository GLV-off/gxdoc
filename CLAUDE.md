# GXDoc — GLV Office XML Document framework

Кроссплатформенный Rust framework для работы с Open XML форматами (OOXML).
Текущий статус: начальная стадия разработки, реализуется поддержка `.xlsx`.

## Сборка и тестирование

```bash
cargo build
cargo test
cargo clippy
```

## Код-стайл

- Rust edition 2024
- Стандартные Rust naming conventions (snake_case, CamelCase)
- `Self` в impl-блоках
- `into()` для конверсий типов
- `Vec<T>` без избыточного аннотирования где тип очевиден
- Публичные типы документировать через rustdoc

## Архитектура

```
Cargo.toml                  # workspace root
crates/
  gx_xlsx/                  # поддержка .xlsx
    src/
      lib.rs                # корень крейта
      cell.rs               # Cell
      cells.rs              # Cells(Vec<Cell>)
      cell_value.rs         # CellValue (enum)
      matrix.rs             # Matrix<T>
      sheet.rs              # Sheet, Sheets
      document.rs           # Document (точка входа)
```

## Роли агентов

- `@architect` — архитектор ПО и координатор команды
- `@dev` — Rust-разработчик
- `@qa_tester` — тестировщик и QA-инженер
- `@tech_writer` — технический писатель
