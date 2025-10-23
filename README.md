# Dependency Analyzer

A tool for analyzing TypeScript/JavaScript import dependencies.

## Features

- Analyzes TypeScript/JavaScript files for different types of imports:
  - **Package imports**: External npm packages
  - **Cross-feature imports**: Aliased imports (configurable, defaults to `~`)
  - **Local imports**: Relative imports within the same feature

- Multiple output formats:
  - **Summary** (default): Overview with occurrence counts
  - **Table**: Clean table format showing import counts per file
  - **JSON**: Machine-readable format for further processing

- **Reverse dependency lookup** (`--find-usage`):
  - Find which files depend on legacy code before removing it
  - Uses glob patterns to match import paths
  - Excludes internal imports within the legacy code itself
  - Shows only external dependencies that need updating

- Configurable options:
  - Custom alias symbol for cross-feature imports
  - Filter to show only files with external dependencies
  - Option to include/exclude local imports
  - Write output to file

## Usage

### Basic Analysis

```bash
# Analyze current directory
dep-analyzer

# Analyze specific directory with table format
dep-analyzer -d /path/to/project -f table

# Show only files with external dependencies
dep-analyzer --external-only

# Use custom alias symbol (e.g., @ instead of ~)
dep-analyzer --alias-symbol @

# Write output to file
dep-analyzer -o report.txt

# Show local imports in table view
dep-analyzer -f table --show-local

# JSON output for scripting
dep-analyzer -f json -o dependencies.json
```

### Finding Legacy Code Dependencies

Before removing legacy code, find what depends on it:

```bash
# Find what imports from a specific legacy path
dep-analyzer --find-usage "~/Meetings/components/details/*"

# Check multiple legacy paths with wildcards
dep-analyzer --find-usage "~/legacy/**/*"

# Save findings to file for review
dep-analyzer --find-usage "~/OldFeature/*" -o cleanup-report.txt

# Works with any alias symbol
dep-analyzer --alias-symbol @ --find-usage "@/old-utils/*"
```

The tool will show:
- Which files import from the pattern (excluding files within the pattern itself)
- The specific import paths being used
- Total count of external dependencies
- Whether it's safe to delete (if no dependencies found)

## Output Formats

### Summary (default)
Shows overview statistics and lists all unique imports with occurrence counts:
```
=== DEPENDENCY ANALYSIS SUMMARY ===
Total files analyzed: 42
Files with package imports: 35
Files with cross-feature imports: 12

Package Dependencies (15):
  • react (23 occurrences)
  • lodash (8 occurrences)
  ...

Cross-Feature Dependencies (5):
  • ~/shared/utils (4 occurrences)
  ...
```

### Table
Clean table format showing import counts per file:
```
┌──────────────────────────────────┬────────┬─────────────┐
│ File                             │  Pkgs  │ Cross-Feat  │
├──────────────────────────────────┼────────┼─────────────┤
│ src/components/App.tsx           │   5    │      2      │
│ src/utils/helper.ts              │   3    │      1      │
└──────────────────────────────────┴────────┴─────────────┘
```

### Usage Finder
Shows external dependencies on legacy code:
```
=== EXTERNAL DEPENDENCIES ON: ~/Meetings/components/details/* ===

Found 3 files importing from this path:

📄 src/Dashboard/MeetingsList.tsx
   • ~/Meetings/components/details/MeetingCard.tsx
   • ~/Meetings/components/details/helper.ts

📄 src/Calendar/EventView.tsx
   • ~/Meetings/components/details/MeetingCard.tsx

📄 src/Reports/index.ts
   • ~/Meetings/components/details/utils.ts

Total: 3 external dependencies

⚠️  These files need to be updated before removing the legacy code.
```

Or if no dependencies found:
```
No external dependencies found for pattern: ~/OldFeature/*

This means no files outside the pattern are importing from it.
Safe to delete! ✓
```

### JSON
Machine-readable format for scripting and further processing.

## Project Structure

```
src/
├── lib.rs           # Library exports
├── main.rs          # CLI entry point
├── analyzer.rs      # Core analysis logic
├── models.rs        # Data structures
├── output.rs        # Output formatting
├── pager.rs         # Automatic paging support
├── summary.rs       # Summary generation
└── usage_finder.rs  # Reverse dependency lookup
```

## Automatic Paging

When output exceeds 30 lines and stdout is a terminal, the tool automatically pipes through a pager (like `less`), similar to how ripgrep works. This makes it easy to browse through large results.

The pager is automatically disabled when:
- Output is redirected to a file using `-o/--output`
- Output is piped to another command
- Output is less than 30 lines
