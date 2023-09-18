[![Project license](https://img.shields.io/badge/License-Public%20Domain-blue.svg)](https://creativecommons.org/publicdomain/zero/1.0/)
[![Tests](https://github.com/aurora-is-near/release-repository/actions/workflows/ci.yml/badge.svg)](https://github.com/aurora-is-near/release-repository/actions/workflows/ci.yml)

# NEAR Release Repository

A NEAR contract which stores a history of contract releases. Useful for cases
when a DAO needs to deploy new releases from a single source of truth.

## Specification

All functions get arguments and return with JSON format.

- `new`
    
    Initialize contract with owner account id
    ```
    INPUT: { "owner_id": "some-account.near" }
    ```

- `is_owner` - call function (you can't use it as view function)

  Check is current user owner. Return: `boolean`.
    ```
    OUTPUT: { true }
    ```

- `get_owner` - view function

  Get owner. Return: `Account ID`.
    ```
    OUTPUT: { "some-account.near" }
    ```

- `push`

    Push new release
    ```
    INPUT: {...}
    ```
    ```
    OUTPUT: {...}
    ```
- `pull`

  Pull (yank) release
    ```
    INPUT: {...}
    ```
    ```
    OUTPUT: {...}
    ```

- `get_status` - view function

  Get status for specific release ID.
    ```
    INPUT: { "id": "v0.5.3-04ca27a6d025a811b213f0870957e76da4bac3257ff751413974f8e05a86ecac" }
    ```
    ```
    OUTPUT: {...}
    ```

- `get_blob` - view function
- `list` - view function
- `list_yang` - view function
- `latest` - view function

## How to
- Build: `cargo build --release --target wasm32-unknown-unknown`
- Clippy: 
  - `cargo clippy -- -D warnings`
  - `cargo clippy --tests -- -D warnings`

- Test: `cargo test --all`


### License: [CC0 1.0 Universal](LICENSE)
