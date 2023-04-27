# Keycheck
Checks your repository for Ethereum private keys in the hexadecimal format. Meant to be used in a pre-commit hook.

## Installation

```shell
cargo install keycheck
```

## Usage


### Manual

Check the current folder and all subfolders for private keys. Respects your `.gitignore`
<br/>Upon finding a private key, outputs the file and line number, but not the key itself.
Exit code 0 for no keys, 1 for key(s) found.

```shell
keycheck
```

### In a git pre-commit hook

```shell
brew install lefthook

echo 'pre-commit:
  commands:
    keycheck:
      run: keycheck' > lefthook.yml
      
lefthook install
```

### Ignoring files
keycheck respects a `.keycheckignore` file. Format is same as `.gitignore`, so globs, comments etc. work as expected.

# License

MIT