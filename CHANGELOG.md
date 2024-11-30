# Changelog

## \[0.1.7]

### Refactors

- [`46477a6`](https://github.com/swpu-acm/online-judge/commit/46477a63bd9e1319f805114e8499e93699ad0095) ([#28](https://github.com/swpu-acm/online-judge/pull/28) by [@fu050409](https://github.com/swpu-acm/online-judge/../../fu050409)) Full rewrite all api endpoints of assets.
- [`c2d07a2`](https://github.com/swpu-acm/online-judge/commit/c2d07a2c778ac676689c796e450c67ac7d10034f) ([#26](https://github.com/swpu-acm/online-judge/pull/26) by [@fu050409](https://github.com/swpu-acm/online-judge/../../fu050409)) Rewrite the model of submission.

### category

- [`a17fe67`](https://github.com/swpu-acm/online-judge/commit/a17fe67eec9d281ab3037809973326c640763159) ([#29](https://github.com/swpu-acm/online-judge/pull/29) by [@K0nnyaku](https://github.com/swpu-acm/online-judge/../../K0nnyaku)) support category

## \[0.1.6]

### Refactors

- [`235618d`](https://github.com/swpu-acm/online-judge/commit/235618d4c4018c299df1ac4ce5ee2e3c3a4635d4) ([#24](https://github.com/swpu-acm/online-judge/pull/24) by [@K0nnyaku](https://github.com/swpu-acm/online-judge/../../K0nnyaku)) refactor submission

## \[0.1.5]

### Refactors

- [`0797749`](https://github.com/swpu-acm/online-judge/commit/079774945731e11abed54382cbfb0cc54f6863f8) ([#20](https://github.com/swpu-acm/online-judge/pull/20) by [@fu050409](https://github.com/swpu-acm/online-judge/../../fu050409)) Allow get account by its surrealdb id (`Thing`).

## \[0.1.4]

### Refactors

- [`7a4e263`](https://github.com/swpu-acm/online-judge/commit/7a4e263999adfb788636025376296c862db66c44) ([#15](https://github.com/swpu-acm/online-judge/pull/15) by [@fu050409](https://github.com/swpu-acm/online-judge/../../fu050409)) Refactor `surrealdb::sql::Thing` in user interface data structures.

  - Use `UserRecordId` instead `surreal::sql::Thing` in structs.
  - Remove `role` from `Account` definition.
  - Refactor and re-export `ProblemDetail` in user interface.

## \[0.1.3]

### Refactors

- [`e51a965`](https://github.com/swpu-acm/online-judge/commit/e51a96583f773c7a7d606bec2aa77b56a4549322) ([#14](https://github.com/swpu-acm/online-judge/pull/14) by [@fu050409](https://github.com/swpu-acm/online-judge/../../fu050409)) Refactored `list` method for account owned problems.
- [`e28b7a2`](https://github.com/swpu-acm/online-judge/commit/e28b7a2c035e502a4c34dff877b0205d370b74a4) ([#12](https://github.com/swpu-acm/online-judge/pull/12) by [@fu050409](https://github.com/swpu-acm/online-judge/../../fu050409)) Refactor api for getting user profile, allow get user profile anonymously

## \[0.1.2]

### feat

- [`d9fb9f8`](https://github.com/swpu-acm/online-judge/commit/d9fb9f85319d7a1b33b674902be8bd429735b5d5) ([#8](https://github.com/swpu-acm/online-judge/pull/8) by [@fu050409](https://github.com/swpu-acm/online-judge/../../fu050409)) Add method for listing all problems from specific user.
- [`5cfb4c4`](https://github.com/swpu-acm/online-judge/commit/5cfb4c48ded65a10087e0ebbc6d41cd769e3a64b) ([#11](https://github.com/swpu-acm/online-judge/pull/11) by [@fu050409](https://github.com/swpu-acm/online-judge/../../fu050409)) Support for filtering problems by filter options.

## \[0.1.1]

### feat

- [`905ccf8`](https://github.com/swpu-acm/online-judge/commit/905ccf874c7876e7724f6d93ac10ede59fffdfd4) ([#1](https://github.com/swpu-acm/online-judge/pull/1) by [@fu050409](https://github.com/swpu-acm/online-judge/../../fu050409)) Support for problems creation and initial submission.
