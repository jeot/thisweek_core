# thisweek-core

`thisweek-core` is a library written in Rust that powers the functionality of the `ThisWeek` task management desktop application. It provides core logic and utilities for managing weekly goals and notes in different calendar system efficiently.

Currently, its primary role is to be used inside the [ThisWeek](https://github.com/jeot/thisweek) (a Tauri desktop application).

## Features

- **Weekly Item Management**
  - Create, update, and delete items for each week: goals (tasks), notes
  - Organize items by priority
  - completion status

- **Objectives Management**
  - Manage long-term goals/notes for year/season/month periods
  - Organize items by priority
  - completion status

- **Multiple Calendar Support**
  - Support for users who want to use two different calendar systems simultaneously
  - Compatible with Gregorian, Chinese, Persian, and Arabic calendars
  - Support for specific calendar language

- **Performance**
  - Built with Rust for optimal performance and reliability
  - Lightweight and fast execution
  - Use SQLite for simple local storage
  - Minimal resource usage

## Installation

To use `thisweek_core` as a standalone library or integrate it into your project:

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/thisweek_core.git
   ```

2. Build the project using Cargo:

   ```bash
   cargo build
   ```

3. Run tests to ensure everything works correctly:

   ```bash
   cargo test
   ```

## Related Projects

- [ThisWeek App](https://github.com/jeot/thisweek) - The main desktop application

## License

This project is licensed under the [MIT License](LICENSE).
