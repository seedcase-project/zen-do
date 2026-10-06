# Changelog

Since we follow [Conventional
Commits](https://decisions.seedcase-project.org/why-conventional-commits/),
we're able to automatically create formal releases based on the commit messages.
The releases are also published to `crate.io` for package distribution and
Zenodo for easier discovery, archival, and citation purposes. We use
[Cocogitto](https://decisions.seedcase-project.org/why-semantic-release-with-cocogitto/)
to be able to automatically create these releases, which uses
[SemVar](https://semverdoc.org) as the version numbering scheme, and [Git
Cliff](https://decisions.seedcase-project.org/why-changelog-with-git-cliff/) to
generate the changelog based on the commit messages.

Because releases are created based on commit messages, a new release is created
quite often---sometimes several times in a day. This also means that any
individual release will not have many changes within it. Below is a list of the
releases we've made so far, along with what was changed within each release.

Commits from bots, like `dependabot` or `pre-commit-ci`, are not included in the
changelog.

## [0.12.1](https://github.com/seedcase-project/zen-do/compare/0.12.0..0.12.1) - 2026-10-06

### ♻️ Refactor

- Set up CLI command structs
  [#168](https://github.com/seedcase-project/zen-do/pull/168) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([97f105c](https://github.com/seedcase-project/zen-do/commit/97f105c46745db076f35830a82692f7434a92980))

### 📝 Documentation

- Add overview page [#194](https://github.com/seedcase-project/zen-do/pull/194)
  by [`@DanMazJen`](https://github.com/DanMazJen)
  ([02ee016](https://github.com/seedcase-project/zen-do/commit/02ee016f79d347d4d335c11f55445e1b8792e437))

### 👩‍💻 Miscellaneous

- Ignore typos in CHANGELOG, lots of false positives
  [#215](https://github.com/seedcase-project/zen-do/pull/215) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([01f3557](https://github.com/seedcase-project/zen-do/commit/01f355721b864c768d9893b944e63004c561de7c))

## [0.12.0](https://github.com/seedcase-project/zen-do/compare/0.11.0..0.12.0) - 2026-10-06

### ✨ Features

- Include error reason in API error
  [#76](https://github.com/seedcase-project/zen-do/pull/76) by
  [`@fruvago`](https://github.com/fruvago)
  ([5eb63fe](https://github.com/seedcase-project/zen-do/commit/5eb63fe62f45ab0a0b1c32ecf0ac4a417fec114c))
- Implement CLI list [#85](https://github.com/seedcase-project/zen-do/pull/85)
  by [`@fruvago`](https://github.com/fruvago)
  ([c8fa83b](https://github.com/seedcase-project/zen-do/commit/c8fa83b15f30eb539c97a65d19de911898e4011c))
- Add `init` CLI command
  [#91](https://github.com/seedcase-project/zen-do/pull/91) by
  [`@fruvago`](https://github.com/fruvago)
  ([80b0591](https://github.com/seedcase-project/zen-do/commit/80b0591731ad711d0757f8e0d20923131bc0f46f))
- Add `get` CLI command
  [#88](https://github.com/seedcase-project/zen-do/pull/88) by
  [`@fruvago`](https://github.com/fruvago)
  ([0445e5a](https://github.com/seedcase-project/zen-do/commit/0445e5a171e6af4b18f22c66987a329b5e609c38))

### ♻️ Refactor

- Move metadata checks to metadata class
  [#82](https://github.com/seedcase-project/zen-do/pull/82) by
  [`@fruvago`](https://github.com/fruvago)
  ([9114109](https://github.com/seedcase-project/zen-do/commit/9114109f96d6744879dfb4ead78863857dbbc96a))
- Use toml metadata file
  [#84](https://github.com/seedcase-project/zen-do/pull/84) by
  [`@fruvago`](https://github.com/fruvago)
  ([991cbc9](https://github.com/seedcase-project/zen-do/commit/991cbc985d04cc34e1d44ae9fa03de45f79aa068))
- Expose individual errors in error message from API
  [#132](https://github.com/seedcase-project/zen-do/pull/132) by
  [`@fruvago`](https://github.com/fruvago)
  ([67b500f](https://github.com/seedcase-project/zen-do/commit/67b500fb8e1650232145b2cca427fb09612f5c51))
- Move metadata into Rust
  [#147](https://github.com/seedcase-project/zen-do/pull/147) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([d89f093](https://github.com/seedcase-project/zen-do/commit/d89f093cda3ef7273d8ca2a51d460905f4c70193))
- Convert read and write metadata into Rust
  [#166](https://github.com/seedcase-project/zen-do/pull/166) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([a49b328](https://github.com/seedcase-project/zen-do/commit/a49b32866b071004d7cca0f4b7291962b0833bd6))

### 📝 Documentation

- Remove broken badge
  [#129](https://github.com/seedcase-project/zen-do/pull/129) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([640f14b](https://github.com/seedcase-project/zen-do/commit/640f14b4d5a0b0409d2646d3ed0014d9f8181312))
- Add `discard` interface design
  [#136](https://github.com/seedcase-project/zen-do/pull/136) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([bae3dde](https://github.com/seedcase-project/zen-do/commit/bae3dde33770a95ae09109bd5f8781a26ff942cc))
- Add interface design for `update`
  [#134](https://github.com/seedcase-project/zen-do/pull/134) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([839fe58](https://github.com/seedcase-project/zen-do/commit/839fe589b367cd21b30a50562bf695a2b6c78cf2))
- Add design interface for `publish`
  [#135](https://github.com/seedcase-project/zen-do/pull/135) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([d85a144](https://github.com/seedcase-project/zen-do/commit/d85a1446ad3d73ecaa259b48911427ac62d0f30f))
- Correct small grammar and spelling issues
  [#130](https://github.com/seedcase-project/zen-do/pull/130) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([a33f922](https://github.com/seedcase-project/zen-do/commit/a33f9220c64972fce5d9061399decd51abc09b9c))
- Update community health docs from template
  [#124](https://github.com/seedcase-project/zen-do/pull/124) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([65df0c4](https://github.com/seedcase-project/zen-do/commit/65df0c41810ba9d9e4b7c2905720632205a70bae))
- Edit minor grammar and spelling issues
  [#144](https://github.com/seedcase-project/zen-do/pull/144) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([8cc4523](https://github.com/seedcase-project/zen-do/commit/8cc45237698ffd0f96a7f9c45edfe2629894f1e1))
- Add requirement to install from crates.io or uv
  [#175](https://github.com/seedcase-project/zen-do/pull/175) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([8f8581c](https://github.com/seedcase-project/zen-do/commit/8f8581cecc24c76541b8492ce5b581e4564981f2))
- Match guide page with other packages
  [#181](https://github.com/seedcase-project/zen-do/pull/181) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([cb6f0bb](https://github.com/seedcase-project/zen-do/commit/cb6f0bbc9f01b1593141e1b43af4cd5f8fb9a1e7))
- Switch to Fru's name
  [#187](https://github.com/seedcase-project/zen-do/pull/187) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([9708599](https://github.com/seedcase-project/zen-do/commit/97085993f7eb05351b94df30d60a8f2759bb562b))
- Add installation guide
  [#145](https://github.com/seedcase-project/zen-do/pull/145) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([4feedc7](https://github.com/seedcase-project/zen-do/commit/4feedc737f373e5fc8756d39a3e4743736a10fde))
- Change rumdl URL, Lychee started flagging it
  [#195](https://github.com/seedcase-project/zen-do/pull/195) by
  [`@DanMazJen`](https://github.com/DanMazJen)
  ([433c902](https://github.com/seedcase-project/zen-do/commit/433c9021c137cc12d16c6cabaa6c8346322f3995))
- Update contributor list with Daniel and Signe
  [#207](https://github.com/seedcase-project/zen-do/pull/207) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([fe077e7](https://github.com/seedcase-project/zen-do/commit/fe077e743303e6e5c47156642f216f924c2d394e))
- Replace "Python" with "Rust" in doc files
  [#206](https://github.com/seedcase-project/zen-do/pull/206) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([a73fa8d](https://github.com/seedcase-project/zen-do/commit/a73fa8de101a84fc650c5ee81d21e01e44458bd6))
- Move TOML motivation to Inputs page
  [#208](https://github.com/seedcase-project/zen-do/pull/208) by
  [`@signekb`](https://github.com/signekb)
  ([0d3c40a](https://github.com/seedcase-project/zen-do/commit/0d3c40ad52973273d79f6486a474d713a015fb84))
- Add navbar title [#209](https://github.com/seedcase-project/zen-do/pull/209)
  by [`@signekb`](https://github.com/signekb)
  ([1da2bf6](https://github.com/seedcase-project/zen-do/commit/1da2bf60be032418e836cf6aa4067469a422d589))

### 💄 Styling

- Update Quarto theme
  [#128](https://github.com/seedcase-project/zen-do/pull/128) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([571f326](https://github.com/seedcase-project/zen-do/commit/571f3261f57dc9753254b5f4689b3bedb6717468))
- Update Quarto theme
  [#188](https://github.com/seedcase-project/zen-do/pull/188) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([431386f](https://github.com/seedcase-project/zen-do/commit/431386fee1c90a7d642279294f873fba4f24d180))
- Reformat Markdown [#196](https://github.com/seedcase-project/zen-do/pull/196)
  by [`@DanMazJen`](https://github.com/DanMazJen)
  ([ba9694a](https://github.com/seedcase-project/zen-do/commit/ba9694ae98656ba8f708edc489198b438785523b))

### 🧪 Tests

- Add publish integration test
  [#143](https://github.com/seedcase-project/zen-do/pull/143) by
  [`@fruvago`](https://github.com/fruvago)
  ([1e41410](https://github.com/seedcase-project/zen-do/commit/1e4141052fccf4ab0034a25017400500c6aa2c42))

### 👷 CI/CD

- Update to Rust workflows from template
  [#126](https://github.com/seedcase-project/zen-do/pull/126) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([d64da20](https://github.com/seedcase-project/zen-do/commit/d64da203febd77c799e2f21466539582f02c63aa))
- Switch to using product project board
  [#189](https://github.com/seedcase-project/zen-do/pull/189) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([0dc2f22](https://github.com/seedcase-project/zen-do/commit/0dc2f223892725b23a3b189e118eac57d4e9bf47))
- Add workflow to release to crates.io
  [#204](https://github.com/seedcase-project/zen-do/pull/204) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([6dde569](https://github.com/seedcase-project/zen-do/commit/6dde569857e94c50e0cb178959a69862ee1d6a4d))
- Install `cargo-edit` in release workflow
  [#214](https://github.com/seedcase-project/zen-do/pull/214) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([6cdd127](https://github.com/seedcase-project/zen-do/commit/6cdd127d9cc2d979ef80863f2aac16768cd68b9e))

### 👩‍💻 Miscellaneous

- Update Python package dependencies
  [#122](https://github.com/seedcase-project/zen-do/pull/122) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([8574015](https://github.com/seedcase-project/zen-do/commit/8574015ba09e9beb82a019bcd3f6cfe540fc9f2a))
- Switch to building a Rust package
  [#123](https://github.com/seedcase-project/zen-do/pull/123) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([ebe7b4a](https://github.com/seedcase-project/zen-do/commit/ebe7b4a97466ea310c169a182407f4e3df6aa338))
- Update DevEx files from template for Rust packages
  [#125](https://github.com/seedcase-project/zen-do/pull/125) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([e920922](https://github.com/seedcase-project/zen-do/commit/e920922d2b132267f561aa5f17d7e0b933d90e10))
- Remove Python config files
  [#127](https://github.com/seedcase-project/zen-do/pull/127) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([62c6eb3](https://github.com/seedcase-project/zen-do/commit/62c6eb33a89ecc5e85c8a3217cc5f2c4a01c414c))
- Add `publish` Python CLI command
  [#133](https://github.com/seedcase-project/zen-do/pull/133) by
  [`@fruvago`](https://github.com/fruvago)
  ([b0301ee](https://github.com/seedcase-project/zen-do/commit/b0301ee37dea5e36ca4544069f1eb46414f7eb6c))
- Include all files to be rendered, but ignore `targets/`
  [#146](https://github.com/seedcase-project/zen-do/pull/146) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([9efc68a](https://github.com/seedcase-project/zen-do/commit/9efc68a584f21161ad966cd92cf6e208795a9c68))
- Ignore XML files when checking URLs
  [#174](https://github.com/seedcase-project/zen-do/pull/174) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([4a0cc8c](https://github.com/seedcase-project/zen-do/commit/4a0cc8cda178a0ad2b6a80b8e9c004275063eb46))
- Fix CODEOWNERS team
  [#173](https://github.com/seedcase-project/zen-do/pull/173) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([d88d492](https://github.com/seedcase-project/zen-do/commit/d88d4926f949a34dc59ab2b221de76895fb78889))
- Remove `license-file` from `Cargo.toml`, was Clippy warning
  [#148](https://github.com/seedcase-project/zen-do/pull/148) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([a9f6a0b](https://github.com/seedcase-project/zen-do/commit/a9f6a0b5c77cfca34891af946891e4c27ea1fd6a))
- Warn on `unwrap()` use, not good practice to use
  [#170](https://github.com/seedcase-project/zen-do/pull/170) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([287eb07](https://github.com/seedcase-project/zen-do/commit/287eb072d50fa1d20ac5f69ac000e00d520c0c1d))
- Set pedantic as `Cargo.toml` option, not via CLI
  [#171](https://github.com/seedcase-project/zen-do/pull/171) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([3416fc2](https://github.com/seedcase-project/zen-do/commit/3416fc245a85a51ec271d6198c69753676fe24a2))
- Switch `CODEOWNER` to new team
  [#197](https://github.com/seedcase-project/zen-do/pull/197) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([8e12431](https://github.com/seedcase-project/zen-do/commit/8e124314b5397f91d288b60d4da8f0148d8fee6c))
- Fix so `qmd` and `md` *not* in `target/` are rendered
  [#203](https://github.com/seedcase-project/zen-do/pull/203) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([9b42260](https://github.com/seedcase-project/zen-do/commit/9b422609a7e66fbdd6ce9088fb02cec9cf08f471))
- Remove Python specific setting files
  [#205](https://github.com/seedcase-project/zen-do/pull/205) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([004014d](https://github.com/seedcase-project/zen-do/commit/004014da2ecac6f134e9844141d6168a3ca70a22))
- Commit `Cargo.toml` during release process
  [#212](https://github.com/seedcase-project/zen-do/pull/212) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([7bb30cc](https://github.com/seedcase-project/zen-do/commit/7bb30ccbc701d51e9c743a56f06bf842f4466a49))
- Use `_` and not `-` for Clippy lint setting
  [#211](https://github.com/seedcase-project/zen-do/pull/211) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([aa7be32](https://github.com/seedcase-project/zen-do/commit/aa7be32a6c75f076bb97403293aadb3ab673f935))
- Add `format-justfile` recipe to justfile
  [#213](https://github.com/seedcase-project/zen-do/pull/213) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([13e09fc](https://github.com/seedcase-project/zen-do/commit/13e09fc5d4d0f113ce7ecf9d621e10976bb7b1e6))

### ❤️ New contributors

- [`@signekb`](https://github.com/signekb) made their first contribution in
  [#209](https://github.com/seedcase-project/zen-do/pull/209)
- [`@DanMazJen`](https://github.com/DanMazJen) made their first contribution in
  [#195](https://github.com/seedcase-project/zen-do/pull/195)

## [0.9.6](https://github.com/seedcase-project/zen-do/compare/0.9.5..0.9.6) - 2026-04-30

### ♻️ Refactor

- Reorganise files [#87](https://github.com/seedcase-project/zen-do/pull/87) by
  [`@fruvago`](https://github.com/fruvago)
  ([09b1270](https://github.com/seedcase-project/zen-do/commit/09b12701660fbdeee86ae0af98eccf69b1d5d05c))

## [0.9.5](https://github.com/seedcase-project/zen-do/compare/0.9.4..0.9.5) - 2026-04-28

### ♻️ Refactor

- Return JSON from Zenodo client
  [#83](https://github.com/seedcase-project/zen-do/pull/83) by
  [`@fruvago`](https://github.com/fruvago)
  ([901d0c1](https://github.com/seedcase-project/zen-do/commit/901d0c1755aea7825a571a89c39dfcf4a80a22ad))

### 🧪 Tests

- Always mock keyring in tests
  [#81](https://github.com/seedcase-project/zen-do/pull/81) by
  [`@fruvago`](https://github.com/fruvago)
  ([f90a702](https://github.com/seedcase-project/zen-do/commit/f90a702dc9a8ba7e86f54dc7f830c202357fb79d))

### 👩‍💻 Miscellaneous

- Setup so website gets built
  [#67](https://github.com/seedcase-project/zen-do/pull/67) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([57c0e34](https://github.com/seedcase-project/zen-do/commit/57c0e348a22efba15a84f8222c80bc1ef69a15da))

## [0.9.4](https://github.com/seedcase-project/zen-do/compare/0.9.3..0.9.4) - 2026-04-24

### ♻️ Refactor

- `ZenodoDepositState` to enum, not literal
  [#65](https://github.com/seedcase-project/zen-do/pull/65) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([dba2021](https://github.com/seedcase-project/zen-do/commit/dba2021ba801131f51cac5c5b82cbbad9ea0e347))

## [0.9.3](https://github.com/seedcase-project/zen-do/compare/0.9.2..0.9.3) - 2026-04-24

### ♻️ Refactor

- Switch to use Soil functionals
  [#80](https://github.com/seedcase-project/zen-do/pull/80) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([94065b2](https://github.com/seedcase-project/zen-do/commit/94065b28941140fcb5048b338edb89ecabde4531))

### 📝 Documentation

- Interface design for `convert`
  [#63](https://github.com/seedcase-project/zen-do/pull/63) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([6dcd47c](https://github.com/seedcase-project/zen-do/commit/6dcd47c16900d94b57e7c13feea07e4e372fca28))

## [0.9.2](https://github.com/seedcase-project/zen-do/compare/0.9.1..0.9.2) - 2026-04-24

### ♻️ Refactor

- Improve look of CLI by using our Soil theme
  [#68](https://github.com/seedcase-project/zen-do/pull/68) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([28ecab1](https://github.com/seedcase-project/zen-do/commit/28ecab14002c522c2de3e6b1360be62aa5ac8c16))

### 👩‍💻 Miscellaneous

- Update Quarto theme [#79](https://github.com/seedcase-project/zen-do/pull/79)
  by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([3ec8bb7](https://github.com/seedcase-project/zen-do/commit/3ec8bb7ac282b0eef45cfa5c72bb9e1f95c2aba7))

## [0.9.1](https://github.com/seedcase-project/zen-do/compare/0.9.0..0.9.1) - 2026-04-20

### ♻️ Refactor

- Rename from `Record` to `Deposit`
  [#64](https://github.com/seedcase-project/zen-do/pull/64) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([ab97f30](https://github.com/seedcase-project/zen-do/commit/ab97f307f08f8bf5eb1d64394fe8169810000e9c))

## [0.9.0](https://github.com/seedcase-project/zen-do/compare/0.8.0..0.9.0) - 2026-04-20

### ✨ Features

- Add `new_version()` [#35](https://github.com/seedcase-project/zen-do/pull/35)
  by [`@fruvago`](https://github.com/fruvago)
  ([f94f8ac](https://github.com/seedcase-project/zen-do/commit/f94f8ac2b7af3b4e85bd3eae71898c2435a1155f))

## [0.8.0](https://github.com/seedcase-project/zen-do/compare/0.6.1..0.8.0) - 2026-04-17

### ✨ Features

- Add `get_token()` [#59](https://github.com/seedcase-project/zen-do/pull/59) by
  [`@fruvago`](https://github.com/fruvago)
  ([94d86a9](https://github.com/seedcase-project/zen-do/commit/94d86a97cd66a2d2fb1c4db6e3c7f06f9312d870))
- Add `update_metadata()`
  [#34](https://github.com/seedcase-project/zen-do/pull/34) by
  [`@fruvago`](https://github.com/fruvago)
  ([da634df](https://github.com/seedcase-project/zen-do/commit/da634dfb10c74dc46b05cbd6b9d7842bacddaed0))

### ♻️ Refactor

- Remove unused/out-dated functions
  [#66](https://github.com/seedcase-project/zen-do/pull/66) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([4dd0976](https://github.com/seedcase-project/zen-do/commit/4dd097663761ad6a7fefc77740c93a3e2c89fa1e))

### 📝 Documentation

- Clarify multi-deposit repos and the input metadata files
  [#56](https://github.com/seedcase-project/zen-do/pull/56) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([e29c190](https://github.com/seedcase-project/zen-do/commit/e29c1907a191bd36bdcc34ad46b891a4a91ddf9f))
- Interface design of `get()`
  [#55](https://github.com/seedcase-project/zen-do/pull/55) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([2df8c07](https://github.com/seedcase-project/zen-do/commit/2df8c070c350867d249ab80cfc3a86dc0ca850d7))
- Interface design for `list()`
  [#53](https://github.com/seedcase-project/zen-do/pull/53) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([8702a42](https://github.com/seedcase-project/zen-do/commit/8702a4292cd29e1dbb3ae6a63390a3cdde672a5b))

## [0.6.1](https://github.com/seedcase-project/zen-do/compare/0.6.0..0.6.1) - 2026-04-15

### ♻️ Refactor

- Use new URN format [#58](https://github.com/seedcase-project/zen-do/pull/58)
  by [`@fruvago`](https://github.com/fruvago)
  ([db5a4a7](https://github.com/seedcase-project/zen-do/commit/db5a4a778e5b1f9b4ef62dcf136f77a99db28d4b))

### 📝 Documentation

- Describe interface's input and output
  [#46](https://github.com/seedcase-project/zen-do/pull/46) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([e806d91](https://github.com/seedcase-project/zen-do/commit/e806d91cb27fa1872bb2a87da3173f4ca9a946fb))
- Fix link to interface landing page
  [#51](https://github.com/seedcase-project/zen-do/pull/51) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([d2e2c2e](https://github.com/seedcase-project/zen-do/commit/d2e2c2e55090c1b0a2a974ed28547531e615a671))
- Interface design for `init()`
  [#52](https://github.com/seedcase-project/zen-do/pull/52) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([ee632ea](https://github.com/seedcase-project/zen-do/commit/ee632eae33250ee78a007ebf208cbd679bacfe4d))
- Explain the URN ID and why we need it
  [#50](https://github.com/seedcase-project/zen-do/pull/50) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([719c673](https://github.com/seedcase-project/zen-do/commit/719c673cefa893f73d9740c72c4941e148285423))

### 💄 Styling

- Ran Markdown formatter
  [#54](https://github.com/seedcase-project/zen-do/pull/54) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([035e3c4](https://github.com/seedcase-project/zen-do/commit/035e3c44000e17c707d259bf1f8001fed068478c))

### 👷 CI/CD

- Comment out publish to pypi for now
  [#48](https://github.com/seedcase-project/zen-do/pull/48) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([c2345c2](https://github.com/seedcase-project/zen-do/commit/c2345c24aae505c33e2e25a55d8990a80ff9c033))

### 👩‍💻 Miscellaneous

- Split out quartodoc step with render step
  [#49](https://github.com/seedcase-project/zen-do/pull/49) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([556fa73](https://github.com/seedcase-project/zen-do/commit/556fa73a57ef9fd2fd9d7ebec5e717f83c71c0bd))
- Don't check URLs in some Python files
  [#47](https://github.com/seedcase-project/zen-do/pull/47) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([d0a2b1b](https://github.com/seedcase-project/zen-do/commit/d0a2b1bd66118b8fbbb57962bad0a059712695ff))

### ❤️ New contributors

- `@pre-commit-ci[bot]` started making automated contributions

## [0.6.0](https://github.com/seedcase-project/zen-do/compare/0.5.0..0.6.0) - 2026-04-08

### ✨ Features

- Handle draft records [#29](https://github.com/seedcase-project/zen-do/pull/29)
  by [`@fruvago`](https://github.com/fruvago)
  ([815b527](https://github.com/seedcase-project/zen-do/commit/815b52727c485b77ed96d34ba6c2b1f8641aade3))

## [0.5.0](https://github.com/seedcase-project/zen-do/compare/0.4.1..0.5.0) - 2026-04-08

### ✨ Features

- Add `upload_file()` [#31](https://github.com/seedcase-project/zen-do/pull/31)
  by [`@fruvago`](https://github.com/fruvago)
  ([3ca1884](https://github.com/seedcase-project/zen-do/commit/3ca1884c0f258efc167faa61aa53606c110dea55))

### 📝 Documentation

- Architecture docs for zen-do
  [#7](https://github.com/seedcase-project/zen-do/pull/7) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([96e18d0](https://github.com/seedcase-project/zen-do/commit/96e18d07d06575dc1033b27cb266cd4bd9ecb41d))

## [0.4.1](https://github.com/seedcase-project/zen-do/compare/0.4.0..0.4.1) - 2026-04-07

### ♻️ Refactor

- Use URN as ID [#33](https://github.com/seedcase-project/zen-do/pull/33) by
  [`@fruvago`](https://github.com/fruvago)
  ([29bcfb4](https://github.com/seedcase-project/zen-do/commit/29bcfb4f077b36bcac580305e1958a912c9f1780))

## [0.4.0](https://github.com/seedcase-project/zen-do/compare/0.3.0..0.4.0) - 2026-04-07

### ✨ Features

- Add `create_record` [#30](https://github.com/seedcase-project/zen-do/pull/30)
  by [`@fruvago`](https://github.com/fruvago)
  ([cc6800b](https://github.com/seedcase-project/zen-do/commit/cc6800be85ff2817f128c85529d2c4cfcb14d478))

## [0.3.0](https://github.com/seedcase-project/zen-do/compare/0.2.0..0.3.0) - 2026-03-25

### ✨ Features

- Add `publish` [#32](https://github.com/seedcase-project/zen-do/pull/32) by
  [`@fruvago`](https://github.com/fruvago)
  ([517d806](https://github.com/seedcase-project/zen-do/commit/517d806104c16fa047b66b52c27817f79edeac03))

## [0.2.0] - 2026-03-23

### ✨ Features

- Move existing code from old repo
  [#28](https://github.com/seedcase-project/zen-do/pull/28) by
  [`@fruvago`](https://github.com/fruvago)
  ([a459eae](https://github.com/seedcase-project/zen-do/commit/a459eae01e3ad6b5059770afa198eba7078c25ba))

### 📝 Documentation

- Purpose and requirements for zen-do
  [#6](https://github.com/seedcase-project/zen-do/pull/6) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([8f4b629](https://github.com/seedcase-project/zen-do/commit/8f4b629c4d3908760c1e90acedcfef95d4970f89))

### 💄 Styling

- Ran Markdown formatter [#5](https://github.com/seedcase-project/zen-do/pull/5)
  by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([d6491ce](https://github.com/seedcase-project/zen-do/commit/d6491ce6c49dfe84a3bfee0bb7d26cdbfbb680f4))

### 👩‍💻 Miscellaneous

- Start of project by [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([678055b](https://github.com/seedcase-project/zen-do/commit/678055ba87907a7e385cf74c3834fe4107f9fe2d))
- Add Netlify ID [#4](https://github.com/seedcase-project/zen-do/pull/4) by
  [`@lwjohnst86`](https://github.com/lwjohnst86)
  ([27e903d](https://github.com/seedcase-project/zen-do/commit/27e903d7e306ec4aac1b804f38463352aa6d47d2))

### ❤️ New contributors

- `@github-actions[bot]` started making automated contributions

- [`@fruvago`](https://github.com/fruvago) made their first contribution in
  [#28](https://github.com/seedcase-project/zen-do/pull/28)

- [`@lwjohnst86`](https://github.com/lwjohnst86) made their first contribution
  in [#6](https://github.com/seedcase-project/zen-do/pull/6)

- `@dependabot[bot]` started making automated contributions
