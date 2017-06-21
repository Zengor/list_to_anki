# list_to_anki

Takes a list Japanese words and makes them into an anki deck file ready to be imported.
Uses the [jisho.org](http://jisho.org) API.

It's far from sofisticated, nor do I recommend it for every learner of Japanese, much less
to beginners.

## Installation

Requires Rust nightly.

```
cargo install --git https://github.com/Zengor/list_to_anki.git
```

### Building from repository
1. `git clone https://github.com/Zengor/list_to_anki.git`
2. `cd list_to_anki`
3. `cargo build`

## Usage
* NOTE: These are far from the best cards. You may want to do some editing as you run into them.

Pass the program the path to a list of words (one word per line), for example:
```
list_to_anki words
```
And it'll generate `words_generated_deck` which can be imported to Anki. You can use the `-o` argument to set the output filename and the `-a` flag to append to the file instead of truncating.

```
list_to_anki words -o my_deck.txt -a
```

It's assumed you have [Japanese Support](https://ankiweb.net/shared/info/3918629684) 
installed to your Anki. The card layout is a simple "front" and "back", where front 
has the format `<kanji>[reading]`. If you wish for the reading to show as furigana,
edit your card file css to use `{{furigana:Front}}`. And if you wish NOT to show the 
reading, use `{{kanji:Front}}`
_Note:_ `Front` will be whatever you have set as the corresponding to `field 1` when importing

To actually import to Anki, open your desktop application and go to `File > Import` and
select the generated deck. You'll be prompted to several settings, leave 
`Allow HTML in fields` *ON* then choose the other options based on which deck you want to
import to with what card style.
