# grawler

[![Build](https://github.com/mrtnvgr/grawler/actions/workflows/rust.yml/badge.svg)](https://github.com/mrtnvgr/grawler/actions/workflows/rust.yml)

Check PR and issue URLs state

## Usage

```bash
grawler check
```

You can specify tokens for increased rate limits:

```bash
GITHUB_TOKEN=`gh auth token` grawler check
```

## Installation

```
cargo install grawler
```

## TODO

- [ ] `grawler setup-hooks`
- [ ] Template for Github Workflows
