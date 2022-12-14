# Homoglyphs (CLI)

**Generate all homoglyphs for a given input sentence.**

> An homoglyph is one of two or more graphemes, characters, or glyphs with shapes that appear identical or very similar. The designation is also applied to sequences of characters sharing these properties.
>
> The Unicode character set contains many strongly homoglyphic characters known as "confusables". These present security risks in a variety of situations. One might deliberately spoof a domain name by replacing one character with its homoglyph, thus creating a second domain name, not readily distinguishable from the first, that can be exploited in phishing.
> \- *[wikipedia](https://en.wikipedia.org/wiki/Homoglyph)*

## Install

```bash
cargo install homoglyphs
```

## Examples

### Generate all possible homoglyphs with all the confusable characters

```bash
homoglyphs -a "rust"
```

### Generate homoglyphs with default settings (100 homoglyphs, 8 confusables for each characters of input sentence)

```bash
homoglyphs "rust is nice"
```

### Generate 'n' homoglyphs

```bash
homoglyphs -n 3000 "rust is awesome"
```

### Generate homoglyphs possible for the given number of confusable

```bash
homoglyphs -n 3000 "rust is incredible"
```

### Try to generate a maximum of 'n' homoglyphs with 'c' confusable for each characters of input sentence

```bash
homoglyphs -n 500 -c 6 "rust is the best"
```

### Print output in different format

#### In array with ascii, empty, markdown, modern and psql styles

```bash
homoglyphs -n 25 "rust is the best" -f markdown
```

#### In json or just raw

```bash
homoglyphs -n 25 "rust is the best" -f json
```
