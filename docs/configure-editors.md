# Configure Code Editors

## [Zed](https://zed.dev)


### Global

Use the application menu:
`Application Menu` > `Open Settings File` or edit the global settings file directly `~./config/zed/settings.json`  

### Local

Edit the local settings file `.zed/settings.json`

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "check": {        
          "command": "clippy", // default: "check"
        },
        "runnables": {
          "extraArgs": ["--release"],
          // extraEnv does not work properly
          // "extraEnv": {
          //  "ZENCH": "panic",
          // },
        },
      },
    },
  },

  // temporarily use in the terminal
  "terminal": {
    "env": {
      "ZENCH": "warn", // or panic
    },
  },
}
```

## [VSCode](https://code.visualstudio.com/)

### Local

Edit the local settings file `.vscode/settings.json`

```json
{
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.runnables.extraArgs": [
        "--release"
    ],
    "rust-analyzer.runnables.extraEnv": {
        "ZENCH": "warn" // or panic
    },
}
```
