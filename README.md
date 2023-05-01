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
keycheck respects a `.keycheckignore` file. Format is the same as `.gitignore`, so globs, comments etc. work as expected.

## Performance

on `OpenZeppelin/openzeppelin-contracts` repo:

```shell
$ hyperfine keycheck -i --warmup 5
Benchmark 1: keycheck
  Time (mean ± σ):      13.1 ms ±   0.5 ms    [User: 3.6 ms, System: 15.9 ms]
  Range (min … max):    12.4 ms …  15.2 ms    183 runs
```
[priv-key-precommit](https://github.com/Dhaiwat10/priv-key-precommit)

```shell
hyperfine priv-key-precommit --warmup 5
Benchmark 1: priv-key-precommit
Time (mean ± σ):      1.651 s ±  0.047 s    [User: 1.625 s, System: 0.217 s]
Range (min … max):    1.599 s …  1.746 s    10 runs
``````

`keycheck` is 126x times faster, or a 99.2% performance improvement.

# License

MIT