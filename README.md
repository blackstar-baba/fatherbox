<div align="center"> <a href="https://github.com/blackstar-baba/fatherbox"> <img alt="fatherbox Logo" width="107" src="public/logo.png"> </a> <br> <br>

[![license](https://img.shields.io/github/license/blackstar-baba/fatherbox.svg)](LICENSE)

<h1>FatherBox</h1>
</div>

**English** | [中文](./README.zh-CN.md)

## Introduction

FatherBox is an open-source, free utility app built using Vben & Tauri technology. Designed to enhance user productivity, it offers a range of tools and features to streamline your workflow efficiently.

## Feature

- Utils, Time Parser JsonFormatter UuidGenerator Encryptor RegExp
- AI, Local LLM & Remote LLM Links
- Editors, Markdown & Flow

## Preview

### Use Gitpod

Open the project in Gitpod (free online dev environment for GitHub) and start coding immediately.

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/blackstar-baba/fatherbox)

## Documentation

## Dev & Build

- Get the project code

```bash
git clone https://github.com/blackstar-baba/fatherbox.git
```

- Installation dependencies

  - Rust 1.78.0+
  - Node.js v16.13+
  - Tauri v1

- run

```bash
cargo tauri dev
```

- build

```bash
cargo tauri build
```

## Install

1. Download app in `releases` page, https://github.com/blackstar-baba/fatherbox/releases.
2. Double click app binary file for install.

## Use

Double click fatherbox app. use username `default` & password `123456` to login <img width="1478" alt="login pag" src="https://github.com/user-attachments/assets/3312c8b9-3a29-48fa-afbf-e24cd03c4b58">

**macos need exec this command before click**:

```
sudo xattr -rd com.apple.quarantine /Applications/fatherbox.app
```

## Change Log

## How to contribute

You are very welcome to join！[Raise an issue](https://github.com/blackstar-baba/fatherbox/issues/new/choose) Or submit a Pull Request。

**Pull Request:**

1. Fork code!
2. Create your own branch: `git checkout -b feat/xxxx`
3. Submit your changes: `git commit -am 'feat(function): add xxxxx'`
4. Push your branch: `git push origin feat/xxxx`
5. submit`pull request`

## Git Contribution submission specification

- reference [vue](https://github.com/vuejs/vue/blob/dev/.github/COMMIT_CONVENTION.md) specification ([Angular](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-changelog-angular))

  - `feat` Add new features
  - `fix` Fix the problem/BUG
  - `style` The code style is related and does not affect the running result
  - `perf` Optimization/performance improvement
  - `refactor` Refactor
  - `revert` Undo edit
  - `test` Test related
  - `docs` Documentation/notes
  - `chore` Dependency update/scaffolding configuration modification etc.
  - `ci` Continuous integration
  - `types` Type definition file changes
  - `wip` In development

## Browser support

The `Chrome 80+` browser is recommended for local development

Support modern browsers, not IE

| [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/edge/edge_48x48.png" alt=" Edge" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>IE | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/edge/edge_48x48.png" alt=" Edge" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Edge | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/firefox/firefox_48x48.png" alt="Firefox" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Firefox | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/chrome/chrome_48x48.png" alt="Chrome" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Chrome | [<img src="https://raw.githubusercontent.com/alrra/browser-logos/master/src/safari/safari_48x48.png" alt="Safari" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Safari |
| :-: | :-: | :-: | :-: | :-: |
| not support | last 2 versions | last 2 versions | last 2 versions | last 2 versions |

## Maintainer

[@blackstar-baba](https://github.com/blackstar-baba)

## Donate

If you think this project is helpful to you, you can help the author buy a cup of coffee to show your support!

## Contributor

## Discord

- [Github Discussions](https://github.com/blackstar-baba/fatherbox/discussions)

## License

- [MIT © blackstar-baba-2024](./LICENSE)
- This project includes code from the [vue-vben-admin](https://github.com/vbenjs/vue-vben-admin), licensed under the MIT License.
