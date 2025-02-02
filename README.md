# 🕷️ Tarantula Management Bot

A Telegram bot built in Rust to help tarantula enthusiasts manage their arachnid companions. This is my first Rust project, developed to learn the language while creating something useful for the tarantula keeping hobby.

## Features

- 🕷️ Track multiple tarantulas with individual profiles
- 🍽️ Feeding schedule management and reminders
- 🏥 Health monitoring and alerts
- 🐾 Molt tracking and history
- 🦗 Cricket colony management
- 🧹 Maintenance task tracking
- 📊 Status overview and statistics

## Getting Started

### Prerequisites

- Rust (latest stable version)
- SQLite
- A Telegram Bot Token (obtain from [@BotFather](https://t.me/botfather))

### Environment Variables

Create a `.env` file in the project root with:

```
DATABASE_PATH=tarantulas.sqlite
TELEGRAM_BOT_TOKEN=your_bot_token_here
DEFAULT_CHAT_ID=your_default_chat_id
```

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/tarantula-management-bot
cd tarantula-management-bot
```

2. Build the project:
```bash
cargo build --release
```

3. Run the bot:
```bash
cargo run --release
```

## Usage

Start a chat with your bot on Telegram and use these commands:

- `/start` - Initialize the bot and see the main menu
- `/help` - Show available commands
- `/addtarantula` - Add a new tarantula to your collection
- `/addcolony` - Add a new cricket colony

## Tech Stack

- 🦀 Rust
- 🤖 Teloxide (Telegram Bot Framework)
- 🗄️ SQLite
- 🔄 Tokio (Async Runtime)
