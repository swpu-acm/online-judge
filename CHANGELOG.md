# Changelog

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
