

# zen-do: Zenodo tasks from the command-line

<!-- TODO: Include DOI after uploading -->

[![Copier](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/copier-org/copier/master/img/badge/badge-grayscale-inverted-border-teal.json?raw=true.svg)](https://github.com/copier-org/copier)
[![GitHub
License](https://img.shields.io/github/license/seedcase-project/zen-do.svg)](https://github.com/seedcase-project/zen-do/blob/main/LICENSE.md)
[![GitHub
Release](https://img.shields.io/github/v/release/seedcase-project/zen-do.svg)](https://github.com/seedcase-project/zen-do/releases/latest)
<!-- [![Crates.io Version](https://img.shields.io/crates/v/zen-do.svg)](https://crates.io/crates/zen-do) -->
[![Build
documentation](https://github.com/seedcase-project/zen-do/actions/workflows/build-website.yml/badge.svg)](https://github.com/seedcase-project/zen-do/actions/workflows/build-website.yml)
[![Checks](https://github.com/seedcase-project/zen-do/actions/workflows/checks.yml/badge.svg)](https://github.com/seedcase-project/zen-do/actions/workflows/checks.yml)
[![OpenSSF
Scorecard](https://api.scorecard.dev/projects/github.com/seedcase-project/zen-do/badge?raw=true.svg)](https://scorecard.dev/viewer/?uri=github.com/seedcase-project/zen-do)
[![CodeQL](https://github.com/seedcase-project/zen-do/actions/workflows/github-code-scanning/codeql/badge.svg?branch=main)](https://github.com/seedcase-project/zen-do/actions/workflows/github-code-scanning/codeql)
[![pre-commit.ci
status](https://results.pre-commit.ci/badge/github/seedcase-project/zen-do/main.svg)](https://results.pre-commit.ci/latest/github/seedcase-project/zen-do/main)
[![lifecycle](https://lifecycle.r-lib.org/articles/figures/lifecycle-experimental.svg)](https://lifecycle.r-lib.org/articles/stages.html#experimental)
[![Project Status: WIP – Initial development is in progress, but there
has not yet been a stable, usable release suitable for the
public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)
<!-- [![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active) -->

Interact with the Zenodo API from the command line for common,
high-level tasks. The main aim is to make it easy to create a Zenodo
deposit using metadata from a file kept within a Git repository and
upload one or more files from the repository to that deposit. While the
intended use of zen-do is to integrate with a GitHub Action workflow for
continuous deployment, it can also be used locally from the command line
for more interactive use.

Check out our [website](https://zen-do.seedcase-project.org) for more
information, such as the features it provides and a
[guide](https://zen-do.seedcase-project.org/docs/guide) to using the
package. For a list of changes, see our [changelog](CHANGELOG.md).

> [!TIP]
>
> This Rust package was generated from
> [template-rs](https://github.com/seedcase-project/template-rs) :tada:

## Contributing

Check out our [contributing document](CONTRIBUTING.md) for information
on how to contribute to the project, including how to set up your
development environment.

Please note that this project is released with a [Contributor Code of
Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree
to abide by its terms.

### Contributors

The following people have contributed to this project by submitting pull
requests :tada:

[@lwjohnst86](https://github.com/lwjohnst86),
[@martonvago](https://github.com/martonvago)

## Licensing

This project is licensed under the [MIT License](LICENSE.md).

## Citing

If you use this package in your work, please cite it as follows:

Johnston L.W., Vago M. zen-do: Tasks to get Zenodo to do from the
command line URL: https://zen-do.seedcase-project.org

Or as a BibTeX entry:

    @misc{YourReferenceHere,
    author = {Johnston, Luke William and Vago, Marton},
    title = {zen-do: Tasks to get Zenodo to do from the command line},
    url = {https://zen-do.seedcase-project.org}
    }
