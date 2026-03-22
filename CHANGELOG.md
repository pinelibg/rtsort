# Changelog

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
