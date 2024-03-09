# ShopifyThemeCleaner

## About

ShopifyThemeCleaner is a tool designed to clean up Shopify themes by removing unused snippet and section files. This tool helps to optimize Shopify themes by reducing clutter and potentially improving theme performance and readability.

## Features

- **Snippet Cleaning**: Removes unused snippet files from your Shopify theme.
- **Section Cleaning**: Removes unused section files from your Shopify theme.
- **Complete Clean Option**: Allows for cleaning both snippets and sections with a single command.

## Prerequisites

To use ShopifyThemeCleaner, you must have [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your machine.

## Installation

1. Clone this repository to your local machine using:
   ```
   git clone <repository-link>
   ```
2. Navigate to the cloned project folder.
3. Compile the project with Cargo using:
   ```
   cargo build --release
   ```
4. The compiled binary will be located in `./target/release/`.

## Usage

To use ShopifyThemeCleaner, navigate to your Shopify theme folder and execute:

```
./shopify_theme_cleaner <path-to-your-theme-folder> [options]
```

### Available Options

- `--clean-all`: Cleans both unused snippets and sections.
- `--clean-snippets`: Cleans only unused snippet files.
- `--clean-sections`: Cleans only unused section files.

## Next Steps

The next development phase for ShopifyThemeCleaner will involve addressing the `assets` directory, which has not yet been implemented. This addition will further enhance the theme cleaning process by removing or optimizing unused or redundant asset files.

## Contributing

Contributions are welcome! If you would like to contribute to the project, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature (`git checkout -b feature/amazing-feature`).
3. Commit your changes (`git commit -am 'Add some amazing feature'`).
4. Push to the branch (`git push origin feature/amazing-feature`).
5. Open a Pull Request.
