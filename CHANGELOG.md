## Version 0.3.11 (2023/01/02)


### Pull Requests Merged

* [PR 47](https://github.com/andfoy/winpty-rs/pull/47) - Prevent iseof from returning immediately, by [@andfoy](https://github.com/andfoy)

In this release 1 pull request was closed.


## Version 0.3.10 (2023/01/02)


### Pull Requests Merged

* [PR 46](https://github.com/andfoy/winpty-rs/pull/46) - Update windows requirement from 0.40.0 to 0.43.0, by [@dependabot[bot]](https://github.com/apps/dependabot)

In this release 1 pull request was closed.


## Version 0.3.9 (2022/09/20)

### Issues Closed

* [Issue 43](https://github.com/andfoy/winpty-rs/issues/43) - Release v0.3.9

In this release 1 issue was closed.

### Pull Requests Merged

* [PR 42](https://github.com/andfoy/winpty-rs/pull/42) - Update windows requirement from 0.39.0 to 0.40.0, by [@dependabot[bot]](https://github.com/apps/dependabot)

In this release 1 pull request was closed.


## Version 0.3.8 (2022/07/21)


### Pull Requests Merged

* [PR 41](https://github.com/andfoy/winpty-rs/pull/41) - Make sure that the docs can be compiled from a non-Windows host machine, by [@andfoy](https://github.com/andfoy)
* [PR 40](https://github.com/andfoy/winpty-rs/pull/40) - Update windows requirement from 0.38.0 to 0.39.0, by [@dependabot[bot]](https://github.com/apps/dependabot)

In this release 2 pull requests were closed.


## Version 0.3.7 (2022/07/05)


### Pull Requests Merged

* [PR 37](https://github.com/andfoy/winpty-rs/pull/37) - Update windows requirement from 0.29.0 to 0.38.0, by [@dependabot[bot]](https://github.com/apps/dependabot)

In this release 1 pull request was closed.


## Version 0.3.5 (2022/03/04)

### Issues Closed

* [Issue 27](https://github.com/andfoy/winpty-rs/issues/27) - Release v0.3.5

In this release 1 issue was closed.

### Pull Requests Merged

* [PR 26](https://github.com/andfoy/winpty-rs/pull/26) - PR: Use WaitForSingleObject instead of GetExitCodeProcess to determine if the PTY process ended, by [@andfoy](https://github.com/andfoy)

In this release 1 pull request was closed.


## Version 0.3.4 (2022/03/03)

### Issues Closed

* [Issue 25](https://github.com/andfoy/winpty-rs/issues/25) - Release v0.3.4
* [Issue 23](https://github.com/andfoy/winpty-rs/issues/23) - ConPTY tests are failing on CIs

In this release 2 issues were closed.

### Pull Requests Merged

* [PR 24](https://github.com/andfoy/winpty-rs/pull/24) - PR: Remove CI constraints used with windows-2019, by [@andfoy](https://github.com/andfoy)
* [PR 22](https://github.com/andfoy/winpty-rs/pull/22) - Bump actions/checkout from 2 to 3, by [@dependabot[bot]](https://github.com/apps/dependabot)

In this release 2 pull requests were closed.


## Version 0.3.3 (2022/02/02)

### Issues Closed

* [Issue 18](https://github.com/andfoy/winpty-rs/issues/18) - Release v0.3.3

In this release 1 issue was closed.

### Pull Requests Merged

* [PR 17](https://github.com/andfoy/winpty-rs/pull/17) - PR: Emit newline in process when setting CONPTY_CI, by [@andfoy](https://github.com/andfoy)
* [PR 16](https://github.com/andfoy/winpty-rs/pull/16) - Fix write corruption caused by writing an unintended NUL byte, by [@segevfiner](https://github.com/segevfiner) ([210](https://github.com/spyder-ide/pywinpty/issues/210))

In this release 2 pull requests were closed.


## Version 0.3.2 (2022/01/24)

### Issues Closed

* [Issue 15](https://github.com/andfoy/winpty-rs/issues/15) - Release v0.3.2

In this release 1 issue was closed.

### Pull Requests Merged

* [PR 13](https://github.com/andfoy/winpty-rs/pull/13) - PR: Fix some issues related to wstring creation in winpty spawn arguments, by [@andfoy](https://github.com/andfoy)

In this release 1 pull request was closed.


## Version 0.3.1 (2022/01/21)

### Issues Closed

* [Issue 12](https://github.com/andfoy/winpty-rs/issues/12) - Release v0.3.1

In this release 1 issue was closed.

### Pull Requests Merged

* [PR 11](https://github.com/andfoy/winpty-rs/pull/11) - PR: Fix some unstable issues related to reading testing on CI, by [@andfoy](https://github.com/andfoy)

In this release 1 pull request was closed.


## Version 0.3.0 (2022/01/20)

### Issues Closed

* [Issue 10](https://github.com/andfoy/winpty-rs/issues/10) - Release v0.3.0

In this release 1 issue was closed.

### Pull Requests Merged

* [PR 9](https://github.com/andfoy/winpty-rs/pull/9) - PR: Fix concurrency errors related to read and other safety issues, by [@andfoy](https://github.com/andfoy)
* [PR 8](https://github.com/andfoy/winpty-rs/pull/8) - PR: Improve read performance and rewrite iseof, by [@andfoy](https://github.com/andfoy)

In this release 2 pull requests were closed.


## Version 0.2.0 (2022-01-11)


### Pull Requests Merged

* [PR 6](https://github.com/andfoy/winpty-rs/pull/6) - PR: Add get_pid method to PTY struct, by [@andfoy](https://github.com/andfoy)

In this release 1 pull request was closed.


## Version 0.1.2 (2022-01-11)


### Pull Requests Merged

* [PR 5](https://github.com/andfoy/winpty-rs/pull/5) - PR: Require PTYImpl trait to implement Send and Sync, by [@andfoy](https://github.com/andfoy)

In this release 1 pull request was closed.


## Version 0.1.1 (2022-01-11)

### Issues Closed

* [Issue 4](https://github.com/andfoy/winpty-rs/issues/4) - Fix release workflow to use cargo token

In this release 1 issue was closed.

## Version 0.1.0 (2022-01-11)


### Pull Requests Merged

* [PR 3](https://github.com/andfoy/winpty-rs/pull/3) - PR: Add release instructions and update package metadata, by [@andfoy](https://github.com/andfoy)
* [PR 2](https://github.com/andfoy/winpty-rs/pull/2) - PR: Add CIs and dependabot files, by [@andfoy](https://github.com/andfoy)

In this release 2 pull requests were closed.
