# Changelog

## Version 0.1.0 - Initial Release

### New Features

- **Snippet Cleaning:** ShopifyThemeCleaner now supports the removal of unused snippet files from Shopify themes. This feature helps to declutter your theme by eliminating unnecessary files that are not being referenced or used in any section or template.
- **Section Cleaning:** Alongside snippets, this version introduces the capability to clean unused section files from your Shopify theme. Similar to snippet cleaning, this feature aids in optimizing your theme's structure and performance by removing redundant section files.
- **Complete Clean Option:** For users looking to perform a comprehensive cleanup, the `--clean-all` option has been introduced. This command combines the functionality of both snippet and section cleaning, offering a more streamlined approach to theme optimization.

### Improvements

- **Performance Enhancements:** Significant improvements have been made to the overall performance of the ShopifyThemeCleaner. These enhancements ensure faster processing times, especially for themes with a large number of files.

- **User Experience:** The CLI interface has been refined for ease of use, with clear instructions and feedback provided to the user throughout the cleaning process.

- **Regex Optimization:** The regular expressions used for identifying snippet, section, and asset references within theme files have been optimized for accuracy and efficiency.

### Known Issues

- **Asset Directory Handling:** As of this release, the cleaning process does not extend to the `assets` directory. This is a known limitation and is scheduled for development in the next version.

### Next Steps

- **Assets Directory Support:** Future development will focus on extending the cleaning capabilities to the `assets` directory, allowing for a more thorough theme optimization process.
