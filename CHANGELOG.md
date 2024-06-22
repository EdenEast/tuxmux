# Changelog

## [0.3.0](https://github.com/EdenEast/tuxmux/compare/tuxmux-v0.2.2...tuxmux-v0.3.0) (2024-06-22)


### ⚠ BREAKING CHANGES

* **ui:** Removed hight from config file

### Features

* **ui:** add custom tui and fuzzy search ([#56](https://github.com/EdenEast/tuxmux/issues/56)) ([3eb933c](https://github.com/EdenEast/tuxmux/commit/3eb933cf9cc2dc1b3eee1457df49b98e4a387eb3))


### Bug Fixes

* **attach:** existing session list passed to picker ([03b4f05](https://github.com/EdenEast/tuxmux/commit/03b4f0550b8bcc436a48102326b848c31ecf2990))

## [0.2.2](https://github.com/EdenEast/tuxmux/compare/tuxmux-v0.2.1...tuxmux-v0.2.2) (2024-06-01)


### Security

* **gix:** resolve CVE-2024-35186 and CVE-2024-35197 ([04231a5](https://github.com/EdenEast/tuxmux/commit/04231a54b867a738f1f24e870e516d27d5161f3b))
* patch gix to resolve CVE-2024-32884 ([33bc4bd](https://github.com/EdenEast/tuxmux/commit/33bc4bdee28446e7c6c7ac009510f8ad5f1b0e7b))


### Bug Fixes

* **nix:** change zsh to correct completion ([d977c9f](https://github.com/EdenEast/tuxmux/commit/d977c9fce3c211089b0ea1e9d8cceb3301f46ea9))

## [0.2.1](https://github.com/EdenEast/tuxmux/compare/v0.2.0...v0.2.1) (2023-12-31)


### Bug Fixes

* **docs:** use non-private image links in docs ([#42](https://github.com/EdenEast/tuxmux/issues/42)) ([057ee4e](https://github.com/EdenEast/tuxmux/commit/057ee4e6e6d3065d7af9cad2633681485af42b98))

## [0.2.0](https://github.com/EdenEast/tuxmux/compare/v0.1.1...v0.2.0) (2023-12-30)


### ⚠ BREAKING CHANGES

* rename tm bin name to tux ([#38](https://github.com/EdenEast/tuxmux/issues/38))

### Features

* **attach:** support worktrees with non-bare repos ([#33](https://github.com/EdenEast/tuxmux/issues/33)) ([a133682](https://github.com/EdenEast/tuxmux/commit/a133682482e43ae9c1310be1349f8f09439edebb))
* **cli:** add completion subcommand ([#40](https://github.com/EdenEast/tuxmux/issues/40)) ([716843f](https://github.com/EdenEast/tuxmux/commit/716843f4b8952f723c1f4951b8aea34b2714b10c))


### Bug Fixes

* **attach:** Window name now set to head branch name ([ae9b95f](https://github.com/EdenEast/tuxmux/commit/ae9b95fffafc07d7715c2e6aa51931dbc88644fa))
* **config:** error on unknown configuration ([1bbe8db](https://github.com/EdenEast/tuxmux/commit/1bbe8db845d916555c3e43a201b252550411f3f3))


### Code Refactoring

* rename tm bin name to tux ([#38](https://github.com/EdenEast/tuxmux/issues/38)) ([e75cd39](https://github.com/EdenEast/tuxmux/commit/e75cd398283282d9687b34560c6126029119bc60))

## [0.1.1](https://github.com/EdenEast/tuxmux/compare/v0.1.0...v0.1.1) (2023-11-01)


### Features

* **cli:** add open in editor option to base command ([#19](https://github.com/EdenEast/tuxmux/issues/19)) ([e4cba78](https://github.com/EdenEast/tuxmux/commit/e4cba78f9fffa2d9095e81642d49a45ccc05b5fd))


### Bug Fixes

* **attach:** early exit if worktree selection is cancelled ([#21](https://github.com/EdenEast/tuxmux/issues/21)) ([96b583a](https://github.com/EdenEast/tuxmux/commit/96b583a73d0b8a9c7f76d73a4c1a9e1e9a6b6b73))


### Performance Improvements

* strip binary and optimize for size ([1586562](https://github.com/EdenEast/tuxmux/commit/158656260f965847464aca33dcc0807c21daed83))
