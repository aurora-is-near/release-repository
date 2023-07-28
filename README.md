[![Project license](https://img.shields.io/badge/License-Public%20Domain-blue.svg)](https://creativecommons.org/publicdomain/zero/1.0/)
[![Lints](https://github.com/aurora-is-near/release-repository/actions/workflows/lints.yml/badge.svg)](https://github.com/aurora-is-near/release-repository/actions/workflows/lints.yml)
[![Tests](https://github.com/aurora-is-near/release-repository/actions/workflows/tests.yml/badge.svg)](https://github.com/aurora-is-near/release-repository/actions/workflows/tests.yml)

# NEAR Release Repository

A NEAR contract which stores a history of contract releases. Useful for cases
when a DAO needs to deploy new releases from a single source of truth.

## Specification

All functions get and return with JSON format.

- `new`
    
    Initialize contract with owner account id
    ```
    INPUT: { "owner_id": string }
    ```
- `push`

    Push new release
    ```
    INPUT: { "owner_id": string }
    ```
    ```
    OUTPUT: { "owner_id": string }
    ```
- `pull`

  Pull (yank) release
    ```
    INPUT: { "owner_id": string }
    ```
    ```
    OUTPUT: { "owner_id": string }
    ```
- `is_owner` - view function

    Check is current user owner 
    ```
    OUTPUT: { "owner_id": string }
    ```
- `get_status` - view function
- `get_blob` - view function
- `list` - view function
- `list_yang` - view function
- `latest` - view function

### License: [CC0 1.0 Universal](LICENSE)
