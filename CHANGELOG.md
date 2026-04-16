# Changelog

Since we follow
[Conventional Commits](https://decisions.seedcase-project.org/why-conventional-commits/)
when writing commit messages, we're able to automatically create formal
releases of the Python package based on the commit messages. The
releases are also published to Zenodo for easier discovery, archival,
and citation purposes. We use
[Commitizen](https://decisions.seedcase-project.org/why-semantic-release-with-commitizen/)
to be able to automatically create these releases, which uses
[SemVar](https://semverdoc.org) as the version numbering scheme.

Because releases are created based on commit messages, we release quite
often, sometimes several times in a day. This also means that any
individual release will not have many changes within it. Below is a list
of the releases we've made so far, along with what was changed within
each release.

## 0.7.0 (2026-04-16)

### Feat

- ✨ add `get_token()` (#59)

## 0.6.1 (2026-04-15)

### Refactor

- ♻️ use new URN format (#58)

## 0.6.0 (2026-04-08)

### Feat

- ✨ handle draft records (#29)

## 0.5.0 (2026-04-08)

### Feat

- :sparkles: add `upload_file()` (#31)

## 0.4.1 (2026-04-07)

### Refactor

- ♻️ use URN as ID (#33)

## 0.4.0 (2026-04-07)

### Feat

- :sparkles: add `create_record` (#30)

## 0.3.0 (2026-03-25)

### Feat

- :sparkles: add `publish` (#32)

## 0.2.0 (2026-03-23)

### Feat

- :sparkles: move existing code from old repo (#28)
