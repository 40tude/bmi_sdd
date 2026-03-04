# bmi_sdd

> **Warning:** The `.cargo/` folder contains Windows-specific configuration (custom `target-dir` for OneDrive, CPU flags). Delete or rename before building:
> ```bash
> mv .cargo .cargo.bak
> ```
> More information on this [page](https://www.40tude.fr/docs/06_programmation/rust/005_my_rust_setup_win11/my_rust_setup_win11.html#onedrive).


## Description

* This is one of the projects I plan to use to illustrate a blog post about "SDD in Rust using Spec Kit".
* SDD = Spec Driven Development
* The aim is to apply the Spec Kit workflow to create a simple BMI Calculator web application in Rust.


## Core Functionality

* Calculate Body Mass Index (BMI) using SI units (kg for weight, meters for height)
* Classify BMI into standard WHO categories:
    * Underweight: < 18.5
    * Normal: 18.5 – 24.9
    * Overweight: 25.0 – 29.9
    * Obese: ≥ 30.0
* Stateless application — no database, no persistence


## API

* Single endpoint: `POST /api/bmi`
    * Request body (JSON): `{ "weight_kg": f64, "height_m": f64 }`
    * Success response (200): `{ "bmi": f64, "category": "string" }`
    * Error response (422): `{ "error": "string" }` with meaningful messages (e.g., `"weight_kg must be positive"`)
* Health check: `GET /health` returning 200 OK


## Tech Stack & Crates

* **Web framework:** Axum + Tokio (async runtime)
* **Serialization:** Serde (JSON request/response)
* **Error handling:** thiserror (domain/library errors) + anyhow (application-level errors)
* **Logging:** tracing + tracing-subscriber — all errors logged server-side
* **CLI config:** Clap (port, log level)
* **HTTP client:** Reqwest (for integration tests)
* **UI:** Bootstrap (CDN), served as embedded HTML via Axum


## Architecture

* Clean separation: domain logic, API layer, UI serving
* Domain module: pure functions for BMI calculation and classification (no I/O, no framework dependencies)
* API module: Axum handlers, JSON types, input validation, error mapping
* UI module: single HTML page with Bootstrap form, fetch-based submission to `/api/bmi`, result display


## Quality & Testing (TDD)

* Unit tests for domain logic (calculation accuracy, category boundaries, edge cases like zero/negative inputs)
* Integration tests for API endpoints using Reqwest (valid requests, invalid inputs, missing fields)
* All tests runnable via `cargo test`


## Deployment

* Run and test locally first — port configurable via `--port` CLI flag or `PORT` env var (Heroku convention)
* Deploy on Heroku using Rust buildpack
* `PORT` env var takes precedence over CLI flag when set
* Procfile included


## Non-goals

* No input range constraints beyond positivity
* No persistence or database
* No API versioning
* No authentication


## License

MIT License - see [LICENSE](LICENSE) for details


## Contributing

This project is developed for personal and educational purposes. Feel free to explore and use it to enhance your own learning.

Given the nature of the project, external contributions are not actively sought nor encouraged. However, constructive feedback aimed at improving the project (in terms of speed, accuracy, comprehensiveness, etc.) is welcome. Please note that this project is being created as a hobby and is unlikely to be maintained once my initial goal has been achieved.