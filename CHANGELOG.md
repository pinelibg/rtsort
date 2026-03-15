# Changelog

## [0.4.0](https://github.com/pinelibg/rtsort/compare/rtsort-v0.3.1...rtsort-v0.4.0) (2026-03-15)


### Features

* add --top/-t option to limit output to first N sorted lines ([fe76089](https://github.com/pinelibg/rtsort/commit/fe760891285b8775521c252ee300426897a40e59))
* add -n/--numeric-sort option ([a2bab73](https://github.com/pinelibg/rtsort/commit/a2bab7378164d42a202128eaa927afdfa9d12328))
* add reverse sorting option ([5db98ef](https://github.com/pinelibg/rtsort/commit/5db98efef5eef18174e668b9bb33372bbe1b8f62))


### Bug Fixes

* add --help long flag (kept -h for human-numeric-sort) ([a5b1082](https://github.com/pinelibg/rtsort/commit/a5b1082d0389775dc0df30b328b556be7d337a54))
* defer alternate screen entry until first line is read ([9612135](https://github.com/pinelibg/rtsort/commit/9612135acf2599681f9de9b90ccf424c6e3ba4c3))
* **deps:** lock file maintenance ([#10](https://github.com/pinelibg/rtsort/issues/10)) ([7cc3154](https://github.com/pinelibg/rtsort/commit/7cc31548baf6d21bac308eef4ec56ee875d9820e))
* test release workflow with draft-publish pattern ([#13](https://github.com/pinelibg/rtsort/issues/13)) ([10cf4ae](https://github.com/pinelibg/rtsort/commit/10cf4ae8c54d12c302d817e75d1e35286ebee61a))
* use alternate screen to prevent upstream stderr from corrupting display ([06c306b](https://github.com/pinelibg/rtsort/commit/06c306bba13385d137095dbac1d0d78178f71303))


### Performance Improvements

* skip terminal redraw when incoming line is discarded by --top ([dc7d6b7](https://github.com/pinelibg/rtsort/commit/dc7d6b7f6a95702dfedd0344ed71dd8fa73ca138))

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
