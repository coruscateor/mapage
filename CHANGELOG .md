# Changelog

## Version 0.1.0 (02/08/2023)

- Initial release

## Version 0.2.0 (25/01/2024)

- Updated dependencies
- Added queries for getting feature configurations.
- Different key types for all the keys of selected types can now be specified as features (only for String and Arc<String> currently).
- Removed a lot of commented code.
- SelectedTypeIOQuery functionality disabled.
- Added ops handlers for the i128 and u128 types.
- Updated project description in cargo.toml.
- Every key and value stored in namespaces must now implement Clone.
- Added Vec fields to Whatever and related enums.
- Added Vec namespaces
- Changed the T type of the returned Hashset object of the *_get_all_keys resolver methods from String to KeyType.
- Changed InputOneofWhatever to InputOneOfWhatever
- Added Vec types to Whatever, SelectedType, InputOneOfSelectedType and AvalibleSelectedType with conversions.
- Updated project description in cargo.toml.
- Every key and value stored in namespaces must now implement Clone.
- Added fns and macros for Vec namespaces.
- Added feature flags for Vec types.
- Rearranged the contents of the resolver_objects/schema module to be a bit more readable.
- Made a bunch of Graphql input and output types implement PartialEq and Eq where possible.
- Stopped the .vscode directory from being tracked in the repository.
- Cleaned up main.rs.
- Added CORS middleware to the Poem web server.







