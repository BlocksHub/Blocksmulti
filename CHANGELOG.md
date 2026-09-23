# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-09-22

### Changed
- **refactor(http)**: Migrated `HttpError` to `thiserror` and added `PoisonedSession` error variant.
- **refactor(http)**: Handle `RwLock` session lock poisoning and simplified JSON request building with `reqwest::RequestBuilder::json`.
- **refactor(client)**: Extracted `auth_payload()` helper to eliminate duplicate request body construction.
- **refactor(client)**: Added `parse_one_or_many<T>()` generic helper for fallback single/array JSON response parsing.
- **refactor(client)**: Restricted `auth_token` and `server_url` field visibility to `pub(crate)`.
- **perf(http)**: Optimized `Endpoint::path` to return `&'static str` instead of allocating heap strings on every request.
- **chore**: Dynamically resolve `USER_AGENT` via `CARGO_PKG_VERSION`.

### Documentation
- Expanded `README.md` with detailed feature descriptions, CI and release badges, and list of supported universities.
- Added documentation usage notes to `README.md`.

### CI & Build
- Cleaned up iOS build workflow by removing Android AAR dependencies and release artifacts from the iOS job.

## [0.1.0] - 2026-09-15

### Added
- Initial release of Blocksmulti.
- Asynchronous Rust client for the Esup Multi API.
- Support for CAS ticket authentication, session management, schedules, cards, university news, interactive map, restaurants, notifications, and static pages.
- UniFFI bindings and automated packaging for Android (Kotlin AAR) and iOS (Swift Package Manager / XCFramework).
- CI/CD workflow for automated multi-platform builds and GitHub releases.
