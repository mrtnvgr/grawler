# grawler

[![Build](https://github.com/mrtnvgr/grawler/actions/workflows/rust.yml/badge.svg)](https://github.com/mrtnvgr/grawler/actions/workflows/rust.yml)

Check PR and issue URLs state

## Usage

Check all files:

```bash
grawler check
```

Setup a pre-commit hook:

```bash
grawler setup-hook
```

You can specify tokens for increased rate limits:

```bash
GITHUB_TOKEN=`gh auth token` grawler check
```

Integrate with Github Workflows:

```yaml
- uses: cargo-bins/cargo-binstall@main
- run: cargo binstall grawler -y --force
- run: grawler check
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Installation

```
cargo install grawler
```

## TODO

- [ ] Template for Github Workflows
