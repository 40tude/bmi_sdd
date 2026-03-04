# bmi_sdd

> **Warning:** The `.cargo/` folder contains Windows-specific configuration (custom `target-dir` for OneDrive, CPU flags). Delete or rename before building:
> ```bash
> mv .cargo .cargo.bak
> ```
> More information on this [page](https://www.40tude.fr/docs/06_programmation/rust/005_my_rust_setup_win11/my_rust_setup_win11.html#onedrive).


## Description

* This is one of the projects I plan to use to illustrate a blog post about "SDD in Rust using Spec Kit".
* SDD = Spec Driven Development
* The aim is to apply the Spec Kit workflow to create
    * A simple BMI Calculator web application (using SI units)
    * In rust
    * Bootstrap should be used for the UI
    * Preferred crates (if applicable)
        * thiserror & anyhow
        * tracing & tracing-subscriber
        * clap
        * serde
        * tokio, axum, reqwest
    * TDD
    * Deployed on Heroku
    * Must run and be tested locally before to be deployed on Heroku



## License

MIT License - see [LICENSE](LICENSE) for details


## Contributing
This project is developed for personal and educational purposes. Feel free to explore and use it to enhance your own learning.

Given the nature of the project, external contributions are not actively sought nor encouraged. However, constructive feedback aimed at improving the project (in terms of speed, accuracy, comprehensiveness, etc.) is welcome. Please note that this project is being created as a hobby and is unlikely to be maintained once my initial goal has been achieved.
