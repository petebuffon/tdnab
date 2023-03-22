# 🦀 tdnab 🦀

## Simple HTML table <td> scraper

tdnab is a Rust CLI program that quickly generates a wordlist from a webpage. All cells, HTML tag <td>, are scraped from all HTML tables from a given website URL. Various flags are available to filter and transform the output wordlist.

## Usage
```
$ tdnab --help
🦀 tdnab 🦀, HTML table <td> scraper

Usage: tdnab [OPTIONS] <URL>

Arguments:
  <URL>  Url to scrape

Options:
  -o, --output <FILE>            Outfile for wordlist
  -a, --ascii                    Exclude cells with non ASCII characters
  -i, --integers                 Exclude integer only cells
  -l, --lowercase                Transform characters to lowercase
  -M, --max <MAX>                Maximum length of cells to keep
  -m, --min <MIN>                Minimum length of cells to keep
  -s, --spaces                   Remove spaces
  -u, --user-agent <USER_AGENT>  Custom user-agent header [default: tdnab/0.1.0]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Examples

```
Generate wordlist and send to stdout

$ tdnab "https://en.wikipedia.org/wiki/Rust_(programming_language)"
Origins
Common Voice
OpenFlint
Pale Moon
Firefox Browser
...
```

```
Generate wordlist and save as wordlist.txt

$ tdnab -o wordlist.txt "https://en.wikipedia.org/wiki/Rust_(programming_language)"
$ wc -l wordlist.txt
421 wordlist.txt
```

Exclude cells 2 characters and under

$ tdnab -m 3 -o wordlist.txt "https://en.wikipedia.org/wiki/Rust_(programming_language)"
$ wc -l wordlist.txt
365 wordlist.txt
```
Exclude integer only cells

$ tdnab -i "https://en.wikipedia.org/wiki/Rust_(programming_language)"
Firefox Browser
Tamarin
Vec
2015-04-15
Idris
```

```
Transform characters to lowercase

$ tdnab -l "https://en.wikipedia.org/wiki/Rust_(programming_language)"
firefox send
thunderbird
forks
newsqueak
rust
```