# Rapid File Comparison Toolkit (RFCTk)
A library of functionality used to compare files written in Rust.

## Build from source
```sh
git clone git@github.com:Num0Programmer/rfctk.git
cd rfctk
make install
```


## Test installation

```sh
make test
```


## Use RFCTk

### Compare two files
```sh
rfc <file> <other_file>
```

### Compare a file to a directory
```sh
rfc <file> <directory>
```

### Compare two directories
```sh
rfc <directory> <other_directory>
```
