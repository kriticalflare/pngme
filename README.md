# PNGme

PNGme is my Rust implementation of the PNGme Project, which you can find more about at [this link](https://picklenerd.github.io/pngme_book/). It's a command line program that lets you hide secret messages in PNG files.

You can find the latest release [here](https://github.com/kriticalflare/pngme/releases)

## Encode Message

### Usage

```
pngme encode <FILE_PATH> <CHUNK_TYPE> <MESSAGE> [OUTPUT_FILE]
```

### Arguments

- `<FILE_PATH>`: The path to the input PNG file.
- `<CHUNK_TYPE>`: The type of chunk to be encoded.
- `<MESSAGE>`: The message to be encoded.
- `[OUTPUT_FILE]` (optional): The path where the encoded PNG will be written to.

## Decode Message

### Usage

```
pngme decode <FILE_PATH> <CHUNK_TYPE>
```

### Arguments

- `<FILE_PATH>`: The path to the input PNG file.
- `<CHUNK_TYPE>`: The type of chunk to be decoded.

## Remove a Chunk

### Usage

```
pngme remove <FILE_PATH> <CHUNK_TYPE>
```

### Arguments

- `<FILE_PATH>`: The path to the input PNG file.
- `<CHUNK_TYPE>`: The type of chunk to be removed.

## Print

### Usage

```
pngme print <FILE_PATH>
```

### Arguments

- `<FILE_PATH>`: The path to the input PNG file.
