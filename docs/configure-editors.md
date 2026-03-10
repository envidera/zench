# Configure Code Editors

## [Zed](https://zed.dev)


`Application Menu` > `Open Settings File`

or

~./config/zed/settings.json file

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "check": {
          "command": "clippy", 
        },
        "runnables": {
          "extraArgs": ["--release"],
          // extraEnv does not work properly
          // "extraEnv": {
          //   "ZENCH": "warn",
          // },
        },
      },
    },
  },
  "terminal": {
    "env": {
      "ZENCH": "warn",
    },
  },
}
```

## [VSCode](https://code.visualstudio.com/)

Put in in your .vscode/settings.json file

```json
{
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.runnables.extraArgs": [
        "--release"
    ],
    "rust-analyzer.runnables.extraEnv": {
        "ZENCH": "warn"
    },
}
```
