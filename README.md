# grawler

[![Build](https://github.com/mrtnvgr/grawler/actions/workflows/rust.yml/badge.svg)](https://github.com/mrtnvgr/grawler/actions/workflows/rust.yml)

Check PR and issue URLs state

> For a List of supported websites, check out [this](https://github.com/mrtnvgr/gitfrog) repo

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
- run: wget https://github.com/mrtnvgr/grawler/releases/latest/download/grawler
- run: chmod +x grawler; ./grawler check
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

## Installation

```
cargo install grawler
```
