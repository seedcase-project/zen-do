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

## 0.9.6 (2026-04-30)

### Refactor

- 🚚 reorganise files (#87)

## 0.9.5 (2026-04-28)

### Refactor

- ♻️ return JSON from Zenodo client (#83)

## 0.9.4 (2026-04-24)

### Refactor

- ♻️ `ZenodoDepositState` to enum, not literal (#65)

## 0.9.3 (2026-04-24)

### Refactor

- ♻️ switch to use Soil functionals (#80)

## 0.9.2 (2026-04-24)

### Refactor

- :lipstick: improve look of CLI by using our Soil theme (#68)

## 0.9.1 (2026-04-20)

### Refactor

- 🚚 rename from `Record` to `Deposit` (#64)

## 0.9.0 (2026-04-20)

### Feat

- ✨ add `new_version()` (#35)

## 0.8.0 (2026-04-17)

### Feat

- :sparkles: add `update_metadata()` (#34)

### Refactor

- 🔥 remove unused/out-dated functions (#66)

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
