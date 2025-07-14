# Nota: Intelligent File Deduplicator

Nota is a fast, safe, and user-friendly command-line tool for finding and removing duplicate files in any directory. It uses parallel processing, advanced filtering, and a robust quarantine/recovery system to ensure you never lose data by accident. Detailed JSON reports and selective restore make it ideal for both power users and cautious beginners.

## Features

- **Multi-threaded, fast scanning** (using Rayon and jwalk)
- **Flexible filtering**: by size, type, and more
- **Safe deduplication**: duplicates are quarantined, not deleted
- **Detailed JSON reports**: paths, space saved, duplicate count
- **Selective recovery**: restore any quarantined file by index
- **Simple CLI interface**: clear commands and flags

## Installation

1. **Clone the repository**:
