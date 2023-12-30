# Changelog

## [0.3.0](https://github.com/EdenEast/tuxmux/compare/v0.2.0...v0.3.0) (2023-12-30)


### ⚠ BREAKING CHANGES

* rename tm bin name to tux ([#38](https://github.com/EdenEast/tuxmux/issues/38))

### Features

* Add completion and manfile to nix build ([1a49f5e](https://github.com/EdenEast/tuxmux/commit/1a49f5e19c74671b569f2de79a04f7804924a126))
* Add completion option ([102d328](https://github.com/EdenEast/tuxmux/commit/102d328a605408eac28986e6b4fdbc7c7a112855))
* Add completions and man file generation ([2fbc2a4](https://github.com/EdenEast/tuxmux/commit/2fbc2a43f543c9861ece70a7d7974b700eddcfd0))
* Add config command ([0e98e47](https://github.com/EdenEast/tuxmux/commit/0e98e4795576b247cbf354d9a995a2b63b75cb14))
* add config option for default worktree selection ([1213804](https://github.com/EdenEast/tuxmux/commit/12138046ba44d6db0135187f0cf3c7a813e76b56))
* Add depth to settings ([b3444d1](https://github.com/EdenEast/tuxmux/commit/b3444d19f252b95576fe6610314a8cb8d5ae8896))
* Add exact flag for searching ([f005207](https://github.com/EdenEast/tuxmux/commit/f0052079426428d35d10702d7ef1ca18dbd68e9f))
* Add height of fuzzy search as config option ([bec497b](https://github.com/EdenEast/tuxmux/commit/bec497bb2904ec1de15f895aa6bbfddbdba27724))
* Add just recipes ([d519f9f](https://github.com/EdenEast/tuxmux/commit/d519f9f2aaff428496d919380bcf6c14fd6bd8c4))
* **attach:** select if only one exact match ([cc75ecb](https://github.com/EdenEast/tuxmux/commit/cc75ecb968925a6957839a54c22c66b0fd7921e2))
* **attach:** shortcut attach to current path `.` ([3b0c922](https://github.com/EdenEast/tuxmux/commit/3b0c92219ca341a8a0b6164fcdb27ba8223f6907))
* **attach:** support worktrees with non-bare repos ([#33](https://github.com/EdenEast/tuxmux/issues/33)) ([a133682](https://github.com/EdenEast/tuxmux/commit/a133682482e43ae9c1310be1349f8f09439edebb))
* **build:** generate manpages foreach subcommand ([fb6c9b3](https://github.com/EdenEast/tuxmux/commit/fb6c9b393fda17ec5fc7f5aa5f996223a19b8c4e))
* **build:** generate shell completions and manpages at build time ([33b7f15](https://github.com/EdenEast/tuxmux/commit/33b7f1538870918b9fd09da0edefbda9334da62a))
* Canonicalize single paths ([c2bab6d](https://github.com/EdenEast/tuxmux/commit/c2bab6d8c73a70bd50f220278aa1421a8fa1cf3b))
* **cli:** add completion subcommand ([#40](https://github.com/EdenEast/tuxmux/issues/40)) ([716843f](https://github.com/EdenEast/tuxmux/commit/716843f4b8952f723c1f4951b8aea34b2714b10c))
* **cli:** add default-config cli option ([4efaf8f](https://github.com/EdenEast/tuxmux/commit/4efaf8f6ea3e412615044db0feb46551be7d3c57))
* **cli:** add open in editor option to base command ([#19](https://github.com/EdenEast/tuxmux/issues/19)) ([e4cba78](https://github.com/EdenEast/tuxmux/commit/e4cba78f9fffa2d9095e81642d49a45ccc05b5fd))
* Config command now has options ([65a089e](https://github.com/EdenEast/tuxmux/commit/65a089e74628258f5f14c1bb89119a36189bc8eb))
* **config:** add exclude_path option ([4152b1c](https://github.com/EdenEast/tuxmux/commit/4152b1c810764fd224ed40873aa184e1f0c8faa3))
* **config:** add parser with proper error messages ([63cab85](https://github.com/EdenEast/tuxmux/commit/63cab85a9aec80a1fbee5800d72cd2834151f15c))
* **config:** Load config from kdl ([0bf9495](https://github.com/EdenEast/tuxmux/commit/0bf9495c3aa33477e0e62d1ff7b3c48b6871c0cb))
* **config:** migrade commands to use config ([9ad2809](https://github.com/EdenEast/tuxmux/commit/9ad28097558b1209336100196ba9c45215023b6a))
* **config:** output diagnostic errors ([8ce66dd](https://github.com/EdenEast/tuxmux/commit/8ce66dde61870c53de0c2606b605e5517744e875))
* **config:** walk workspace paths for workspace definitions ([e8c66a2](https://github.com/EdenEast/tuxmux/commit/e8c66a26166923e1c42335050354a13793e65138))
* **finder:** remove skim as internal fuzzy finder ([#7](https://github.com/EdenEast/tuxmux/issues/7)) ([4c8ec5d](https://github.com/EdenEast/tuxmux/commit/4c8ec5d8697c9055109bdca7c120c2feaf26a7e6))
* list path command ([9d7444c](https://github.com/EdenEast/tuxmux/commit/9d7444cde1adf367a4f4f4b2e4a2d14da580e959))
* **list:** add all paths option ([512037c](https://github.com/EdenEast/tuxmux/commit/512037cd4414e741231022dad108602423d947e4))
* migrade from eyre to miette Result ([67532f5](https://github.com/EdenEast/tuxmux/commit/67532f59d0529306d7654692d396e0388e0b5af3))
* **nix:** switch to use crane for nix rust build ([69da779](https://github.com/EdenEast/tuxmux/commit/69da7796db029ea7b44b75d6a773fbed0a96cded))
* Prioritize exact matches compared to fuzzy ([a803960](https://github.com/EdenEast/tuxmux/commit/a803960a49548c50c99287583db42ab022299786))
* select worktree if more then one exists ([9e29af3](https://github.com/EdenEast/tuxmux/commit/9e29af3f7c796c0c5f71b8be88380b02800e8cf1))
* Update to clap 4.0 ([3927f4d](https://github.com/EdenEast/tuxmux/commit/3927f4dbe8b36aae762cfbcffb64dda701b1a23d))


### Bug Fixes

* add default config file to cargo include list ([02e82ce](https://github.com/EdenEast/tuxmux/commit/02e82ce9732452b791e45f9eb9465757470f1ad9))
* Attach command uses --path command ([294cd10](https://github.com/EdenEast/tuxmux/commit/294cd1038ac954716ae1968d04172f68a77eb0c9))
* **attach:** early exit if worktree selection is cancelled ([#21](https://github.com/EdenEast/tuxmux/issues/21)) ([96b583a](https://github.com/EdenEast/tuxmux/commit/96b583a73d0b8a9c7f76d73a4c1a9e1e9a6b6b73))
* **attach:** ignore non bare repos for worktrees ([29a231e](https://github.com/EdenEast/tuxmux/commit/29a231ed57dd8d33ddbd77f3e9e18934b2798ad3))
* **attach:** Window name now set to head branch name ([ae9b95f](https://github.com/EdenEast/tuxmux/commit/ae9b95fffafc07d7715c2e6aa51931dbc88644fa))
* **cli:** disable colored help ([725e9ed](https://github.com/EdenEast/tuxmux/commit/725e9ed9a7305e1626e7e5c706535162a4d02dc7))
* **cli:** make aliases visible in help ([e3bcfbf](https://github.com/EdenEast/tuxmux/commit/e3bcfbff78d6aee53d7cb3a6d5368cd7ae713ce7))
* **cmd:** handle aliases for commands ([d2f726f](https://github.com/EdenEast/tuxmux/commit/d2f726f109025aee9991eaf7e5bf6cd2d7b9110b))
* **config:** default depth setting to 10 ([b80edb4](https://github.com/EdenEast/tuxmux/commit/b80edb4421dd7e240953bfd174ce9591425da946))
* **config:** error on unknown configuration ([1bbe8db](https://github.com/EdenEast/tuxmux/commit/1bbe8db845d916555c3e43a201b252550411f3f3))
* deny copyleft dependencies ([ca4e93e](https://github.com/EdenEast/tuxmux/commit/ca4e93e388baeba2880f6f62716ddbef444b06c5))
* **deny:** remove ignore advisory as skim is removed ([bd43f38](https://github.com/EdenEast/tuxmux/commit/bd43f38a74c62a86f9d9e808d997cbd87b894fca))
* **finder:** remove printing query from result ([a167b5e](https://github.com/EdenEast/tuxmux/commit/a167b5ea0a34f815fe311061243aa75472846484))
* Format session name to a valid tmux name ([4246ce0](https://github.com/EdenEast/tuxmux/commit/4246ce0f29f023ea08435548c352b5678430e9f2))
* Height to 40 for search ([07ef9c2](https://github.com/EdenEast/tuxmux/commit/07ef9c2aadd138fd3adf3f9be858854191b05fcf))
* **jump:** create session if does not exist ([2e588aa](https://github.com/EdenEast/tuxmux/commit/2e588aa4ac797ddd9859fe55165be8875112fdb3))
* **main:** fallback to `attach` if subcommand fail ([2ccf0a9](https://github.com/EdenEast/tuxmux/commit/2ccf0a94a4d2e1f87ee1042b29675f608789d9ab))
* make fuzzy search of worktree optional ([485a1a1](https://github.com/EdenEast/tuxmux/commit/485a1a1db185013d9dd158c9b6cd074b36cf0b75))
* **nix:** add missing security native lib on mac ([38c50ed](https://github.com/EdenEast/tuxmux/commit/38c50ed7ca9881507a601d9d6d66ca940c945d69))
* **nix:** install manpage and shell completion ([94d7f73](https://github.com/EdenEast/tuxmux/commit/94d7f7374f3896316eaa23d4d31c73225ec45eb7))
* **nix:** move from nixos-unstable to nixpkgs-unstable ([114bafd](https://github.com/EdenEast/tuxmux/commit/114bafddc8d0f6c1b28f4a0a434baa880a2e4577))
* **nix:** pkg-config added to full package drv ([a57edd4](https://github.com/EdenEast/tuxmux/commit/a57edd4a355a64902b172fdfda513c558f918d3e))
* **nix:** set proper name for nix app ([d81ecef](https://github.com/EdenEast/tuxmux/commit/d81ecef9d77f4b76d06054648e13793545cf82e7))
* Path option can on attach command can now take `.` ([bfa4d34](https://github.com/EdenEast/tuxmux/commit/bfa4d340a958d074139cef9cef3c56cea002f8b6))
* **path:** expand `~` to home dir ([d867f88](https://github.com/EdenEast/tuxmux/commit/d867f884aa06466591d80c3204ce71f8aa4d22fb))
* remove config command line option ([a7a5b85](https://github.com/EdenEast/tuxmux/commit/a7a5b85e7abfc5a8daebbc2e96c46f1e6fa0cf1a))
* remove unused argument for path.add ([d58647a](https://github.com/EdenEast/tuxmux/commit/d58647ae82e679cc44841dd445fb8c76793cc912))
* **walker:** add missing singles path back to result ([dd247a1](https://github.com/EdenEast/tuxmux/commit/dd247a1256ea95a3bcfdf2e1a1f121a4d062d23a))


### Performance Improvements

* strip binary and optimize for size ([1586562](https://github.com/EdenEast/tuxmux/commit/158656260f965847464aca33dcc0807c21daed83))


### Code Refactoring

* rename tm bin name to tux ([#38](https://github.com/EdenEast/tuxmux/issues/38)) ([e75cd39](https://github.com/EdenEast/tuxmux/commit/e75cd398283282d9687b34560c6126029119bc60))

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
