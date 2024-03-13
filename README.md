# bevy_template

Welcome to the bevy_template repository! This project is designed to get you up and running with the [Bevy game engine](https://bevyengine.org/), providing a solid foundation for building your own games or simulations. Included in this template are configurations for VSCode, continuous integration (CI), Lefthook, `rstest` for testing, and coverage reporting, ensuring a robust development environment from the start.

## Getting Started

Follow these steps to set up your development environment:

### Setup Bevy

Ensure you have the latest version of [Rust](https://www.rust-lang.org/) installed. Then, clone this repository and navigate into it:

```sh
git clone https://github.com/your_username/bevy_template.git
cd bevy_template
```

Install the required dependencies and compile the project:

```sh
cargo build
```

### Setup VSCode

This project includes recommended VSCode settings and extensions to enhance your development experience. Open the project in VSCode, and you'll be prompted to install the recommended extensions.

### Continuous Integration (CI)

We use GitHub Actions for CI to ensure code quality and run tests automatically with each push and pull request. Check the .github/workflows directory for the CI configuration.

### Setup Lefthook

[Lefthook](https://github.com/evilmartians/lefthook) is configured for this project to run pre-commit hooks that include formatting and linting checks. Install Lefthook globally and set up the git hooks:

### Setup `rstest`

`rstest` is used for concise and readable `tests`. See the tests directory for examples on how to write your tests using `rstest`.

### Test Coverage

To generate test coverage reports, run:

```sh
cargo llvm-cov --open
```

Open in your browser to view the coverage report.

## Documentation

- **Setup GitHub Repository**: Guidelines for initializing and configuring your GitHub repository can be found in `docs/setup_github_repository.md.`
- **Setup Branch Protection**: To ensure code quality and stability, follow the branch protection setup in `docs/setup_branch_protection.md.`

## Contributing

We welcome contributions! Please read `CONTRIBUTING.md` for more information on how to get involved.

## License

This project is licensed under MIT License. See the LICENSE file for more details.
