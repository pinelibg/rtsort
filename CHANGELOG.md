# Changelog

## [0.6.1](https://github.com/pinelibg/rtsort/compare/v0.6.0...v0.6.1) (2026-04-28)


### Bug Fixes

* **deps:** lock file maintenance ([#56](https://github.com/pinelibg/rtsort/issues/56)) ([3d2cd1d](https://github.com/pinelibg/rtsort/commit/3d2cd1dbe0c33af322ac593426431fb6ede29097))
* **deps:** lock file maintenance ([#69](https://github.com/pinelibg/rtsort/issues/69)) ([b83a776](https://github.com/pinelibg/rtsort/commit/b83a776a02d0934deea2e859fac392d1c86bcd40))
* **deps:** lock file maintenance ([#71](https://github.com/pinelibg/rtsort/issues/71)) ([edae220](https://github.com/pinelibg/rtsort/commit/edae2204435868d38670ec828e73fa9252053f17))

## [0.6.0](https://github.com/pinelibg/rtsort/compare/v0.5.0...v0.6.0) (2026-03-30)


### Features

* add --no-preview to suppress live terminal display ([#42](https://github.com/pinelibg/rtsort/issues/42)) ([49c6ca6](https://github.com/pinelibg/rtsort/commit/49c6ca6f5c4f794306eac43c4e0b9e681b169276))
* add -k/--key and -t/--field-separator; remove -t short alias from --top ([#44](https://github.com/pinelibg/rtsort/issues/44)) ([ffc73c7](https://github.com/pinelibg/rtsort/commit/ffc73c74b4af2faa3cf778ee6e549f1956b3aa56))
* add -u/--unique flag to remove duplicate lines ([#41](https://github.com/pinelibg/rtsort/issues/41)) ([b54a866](https://github.com/pinelibg/rtsort/commit/b54a8667d536e39acd9e2bab90ac8510b5eebe8a))
* add -V/--version-sort ([#43](https://github.com/pinelibg/rtsort/issues/43)) ([ae6ba90](https://github.com/pinelibg/rtsort/commit/ae6ba903a36aacf3b6697184b206ed0fb974f4b2))


### Bug Fixes

* **deps:** lock file maintenance ([#54](https://github.com/pinelibg/rtsort/issues/54)) ([bfd7abf](https://github.com/pinelibg/rtsort/commit/bfd7abf897adfecfcdac4a5149c205e9b725e79e))
* **main:** restore terminal on Ctrl-C via SIGINT handler ([#51](https://github.com/pinelibg/rtsort/issues/51)) ([9ff421c](https://github.com/pinelibg/rtsort/commit/9ff421c93afc27706ea2fbdebb523d74185e2d2f))


### Performance Improvements

* **main:** add --fps flag to control preview update rate ([#52](https://github.com/pinelibg/rtsort/issues/52)) ([3f5e353](https://github.com/pinelibg/rtsort/commit/3f5e35308566a1a169956515630b0fa42ff0a7e3))
* **main:** cache extracted sort keys to avoid redundant computation ([#48](https://github.com/pinelibg/rtsort/issues/48)) ([30a34b5](https://github.com/pinelibg/rtsort/commit/30a34b5139695bb96d9d28b0590dd60bc70bc329))

## [0.5.0](https://github.com/pinelibg/rtsort/compare/v0.4.3...v0.5.0) (2026-03-22)


### Features

* add --bottom option to output the last N sorted lines ([#36](https://github.com/pinelibg/rtsort/issues/36)) ([f35c790](https://github.com/pinelibg/rtsort/commit/f35c790566e2a8918c8213f0acc2ce2a30191603))

## [0.4.3](https://github.com/pinelibg/rtsort/compare/v0.4.2...v0.4.3) (2026-03-22)


### Bug Fixes

* **ci:** trigger publish workflow from release publish ([#37](https://github.com/pinelibg/rtsort/issues/37)) ([6de3a93](https://github.com/pinelibg/rtsort/commit/6de3a93247d5bad952e73ddb2780a3a527382486))

## [0.4.2](https://github.com/pinelibg/rtsort/compare/v0.4.1...v0.4.2) (2026-03-22)


### Bug Fixes

* **ci:** prevent stale release PR caused by draft release visibility ([#34](https://github.com/pinelibg/rtsort/issues/34)) ([c0e5015](https://github.com/pinelibg/rtsort/commit/c0e501554f6510b78015e4e9012b6eb62b115dd9))

## [0.4.1](https://github.com/pinelibg/rtsort/compare/v0.4.0...v0.4.1) (2026-03-22)


### Bug Fixes

* **ci:** force tag creation for draft releases ([#31](https://github.com/pinelibg/rtsort/issues/31)) ([a70c756](https://github.com/pinelibg/rtsort/commit/a70c7568084b07a4c1a3848711f46554b1837c26))

## [0.4.0](https://github.com/pinelibg/rtsort/compare/v0.3.4...v0.4.0) (2026-03-22)


### Features

* add case-insensitive sort option (-f/--ignore-case) ([#29](https://github.com/pinelibg/rtsort/issues/29)) ([ee52388](https://github.com/pinelibg/rtsort/commit/ee52388840e999566dea8a7eb0c824763296bea8))


### Bug Fixes

* **ci:** correct crates-io-auth-action tag comment for Renovate ([#25](https://github.com/pinelibg/rtsort/issues/25)) ([8b55b53](https://github.com/pinelibg/rtsort/commit/8b55b5320a2d2f10f0657be00813b6a5ff18ef6b))

## [0.3.4](https://github.com/pinelibg/rtsort/compare/v0.3.3...v0.3.4) (2026-03-15)


### Bug Fixes

* **ci:** remove ref from checkout in build job ([#23](https://github.com/pinelibg/rtsort/issues/23)) ([28ce34d](https://github.com/pinelibg/rtsort/commit/28ce34d6f0f2d55c42d74c74ca9af8bafc16cdb2))

## [0.3.3](https://github.com/pinelibg/rtsort/compare/v0.3.2...v0.3.3) (2026-03-15)


### Bug Fixes

* **ci:** use refs/tags/ prefix for checkout ref in build job ([#20](https://github.com/pinelibg/rtsort/issues/20)) ([dd92d81](https://github.com/pinelibg/rtsort/commit/dd92d813a81d6076e406a8faea03667cc8468308))

## [0.3.2](https://github.com/pinelibg/rtsort/compare/v0.3.1...v0.3.2) (2026-03-15)


### Bug Fixes

* **ci:** set include-component-in-tag=false to match existing tag format ([#17](https://github.com/pinelibg/rtsort/issues/17)) ([dccc4ff](https://github.com/pinelibg/rtsort/commit/dccc4ff8581e8fa325e7846f39f0f1f1d600bdb9))

## [0.3.1](https://github.com/pinelibg/rtsort/compare/v0.3.0...v0.3.1) (2026-03-15)


### Bug Fixes

* test release workflow with draft-publish pattern ([#13](https://github.com/pinelibg/rtsort/issues/13)) ([10cf4ae](https://github.com/pinelibg/rtsort/commit/10cf4ae8c54d12c302d817e75d1e35286ebee61a))

## [0.3.0](https://github.com/pinelibg/rtsort/compare/v0.2.0...v0.3.0) (2026-03-15)


### Features

* add --top/-t option to limit output to first N sorted lines ([fe76089](https://github.com/pinelibg/rtsort/commit/fe760891285b8775521c252ee300426897a40e59))


### Bug Fixes

* **deps:** lock file maintenance ([#10](https://github.com/pinelibg/rtsort/issues/10)) ([7cc3154](https://github.com/pinelibg/rtsort/commit/7cc31548baf6d21bac308eef4ec56ee875d9820e))


### Performance Improvements

* skip terminal redraw when incoming line is discarded by --top ([dc7d6b7](https://github.com/pinelibg/rtsort/commit/dc7d6b7f6a95702dfedd0344ed71dd8fa73ca138))

## [0.2.0](https://github.com/pinelibg/rtsort/compare/v0.1.0...v0.2.0) (2026-03-14)


### Features

* add -n/--numeric-sort option ([a2bab73](https://github.com/pinelibg/rtsort/commit/a2bab7378164d42a202128eaa927afdfa9d12328))


### Bug Fixes

* add --help long flag (kept -h for human-numeric-sort) ([a5b1082](https://github.com/pinelibg/rtsort/commit/a5b1082d0389775dc0df30b328b556be7d337a54))
* defer alternate screen entry until first line is read ([9612135](https://github.com/pinelibg/rtsort/commit/9612135acf2599681f9de9b90ccf424c6e3ba4c3))

## 0.1.0 (2026-03-14)


### Features

* add reverse sorting option ([5db98ef](https://github.com/pinelibg/rtsort/commit/5db98efef5eef18174e668b9bb33372bbe1b8f62))


### Bug Fixes

* use alternate screen to prevent upstream stderr from corrupting display ([06c306b](https://github.com/pinelibg/rtsort/commit/06c306bba13385d137095dbac1d0d78178f71303))
