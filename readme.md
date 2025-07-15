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
```
git clone https://github.com/yourusername/Nota.git
cd Nota
```

---

### 2. Build the project

```
cargo build --release
```

---

### 3. Run the tool

```
./target/release/nota [COMMAND] [OPTIONS]
```

---

## Usage Overview

Nota is invoked from the command line. The basic syntax is:

```
nota <SUBCOMMAND> [PATH] [FILTER] [FLAGS]
```

- `<SUBCOMMAND>`: The operation you want to perform (`same`, `filter`, `restore`)
- `[PATH]`: The directory to scan (optional; defaults to current directory)
- `[FILTER]`: Optional filter keyword (e.g., `large`) for advanced filtering
- `[FLAGS]`: Optional flags for extra operations (e.g., quarantine, filter)

---

## Subcommands & Flags

### 1. `same`: Find and handle duplicate files

Detects duplicates in the given directory and offers options to quarantine or filter them.

#### Syntax
```
nota same [PATH] [FILTER] [FLAGS]
```

#### Options

| Argument/Flag         | Description                                                                                      |
|-----------------------|--------------------------------------------------------------------------------------------------|
| `[PATH]`              | Directory to scan. Defaults to the current directory if omitted.                                 |
| `[FILTER]`            | Optional filter keyword. E.g., `large` to process only large files.                              |
| `-f`, `--filter`      | After quarantine, permanently deletes files in quarantine.                                       |
| `-q`, `--quarantine`  | Move duplicate files to a `quarantine` subdirectory for safe review/recovery.                    |

#### Examples

- **Scan current directory for all duplicates:**

```
nota same
```

- **Scan `E:/test` for duplicates, only large files (>10MB):**

```
nota same E:/test large
```

- **Scan and quarantine duplicates:**

```
nota same E:/test -q
```

- **Scan, quarantine, and then filter (delete) duplicates:**
```
nota same E:/test -f
```

**Note:** Using both `-q` and `-f` together is not allowed.

---

### 2. `filter`: Filter (delete) files in quarantine

Permanently deletes all files in the quarantine directory for the given path.

#### Syntax
```
nota filter [PATH] [FILTER] [FLAGS]
```

#### Options

| Argument/Flag         | Description                                                                                      |
|-----------------------|--------------------------------------------------------------------------------------------------|
| `[PATH]`              | Directory whose quarantine folder should be filtered. Defaults to current directory.             |
| `[FILTER]`            | Optional filter keyword.                                                                         |
| `-q`, `--quarantine`  | Quarantines duplicates before filtering.                                                         |

#### Example

- **Delete all files in quarantine for `E:/test`:**
```
nota filter E:/test
```

- **Quarantine and then delete:**
```
nota filter E:/test -q
```

---

### 3. `restore`: Restore quarantined files

Interactively restores any quarantined file to its original location, based on the JSON report.

#### Syntax
```
nota restore [PATH]
```

#### Options

| Argument     | Description                                                                                              |
|--------------|----------------------------------------------------------------------------------------------------------|
| `[PATH]`     | Directory whose `quarantine` subfolder should be used for restore. Defaults to current directory.        |

#### Example

- **Restore a file from quarantine in `E:/test`:**

```
nota restore E:/test
```

You will be shown a numbered list of quarantined files and prompted to select one for restoration.

---

## Filtering

You can filter files before deduplication using keywords:

| Keyword   | Effect                                        |
|-----------|-----------------------------------------------|
| `large`   | Only process files larger than 10MB           |
| *(future)*| Other filters (e.g., by extension, date, etc.)|

---

## JSON Report

After each deduplication run, Nota creates `report.json` in the current directory, containing:

- `file_paths`: List of all quarantined (duplicate) file paths
- `total_space_saved_bytes`: Total disk space saved
- `duplicate_file_count`: Number of duplicate files found

This report is used for selective recovery.

---

## Recovery System

- **Quarantine**: Duplicates are moved to `quarantine/` under the scanned directory.
- **Restore**: Use `nota restore [PATH]` to see and restore any quarantined file.
- **Filter**: Use `nota filter [PATH]` to permanently delete all quarantined files.

---

## Example Workflows

**1. Safe Deduplication:**

```
nota same E:/photos -q
```

- Finds duplicates in `E:/photos`, moves them to `E:/photos/quarantine`, and generates a report.

**2. Review and Selective Restore:**

```
nota restore E:/photos
```

- Lists all quarantined files for `E:/photos` and lets you restore any by index.

**3. Permanent Deletion:**

```
nota filter E:/photos
```

- Permanently deletes all files in `E:/photos/quarantine`.

---

## Advanced Notes

- **Parallel Processing:** Uses all available CPU cores for fast scanning.
- **Extensible Filtering:** The filtering system is modular and can be extended for more complex criteria.
- **Safe by Default:** No files are deleted unless you explicitly use the filter command.
- **Cross-Platform:** Works on Windows, Linux, and macOS.

---

## Troubleshooting

- **Unused imports/warnings:** Clean up unused imports and variables as suggested by the compiler.
- **Crate name:** Ensure your crate name is lowercase (`nota`) in `Cargo.toml`.
- **Integration tests:** Place tests in the `tests/` directory and import modules via `nota::module`.

---

## Contributing

- Fork the repo and open a pull request.
- Add new filters, reporting formats, or recovery features.
- Write tests in the `tests/` directory.

---

## License

MIT License

**Nota** is designed to make deduplication safe, fast, and transparent.  
For questions, suggestions, or bug reports, please open an issue on the repository.



