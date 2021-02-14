# Kanji-XCompose

This is a project for writing kana and composing kanji with a normal .XCompose file and an XKB layout.

# Why?

IBus etc. allow you to compose kanji by writing the pronunciation, then clicking
a button until you find the right kanji. This is however problematic, because you
can not know how many times you have to click since the sorting is "smart".
Even if it wasn't smart however, clicking e.g. space 20 times in a row isn't optimal.

# Usage

## Set-up

Just pipe in the kanjidic2.xml file from https://www.edrdg.org/wiki/index.php/KANJIDIC_Project
and it will spit out a compose file. You will also need to use the corresponding
XKB layout when attempting to compose kanji. To use the XKB layout,
simply put the file into ~/.xkb/symbols/ with a fitting name
(e.g. "kanji-xcompose") and switch to that layout.
E.g. with sway: `swaymsg input '*' xkb_layout kanji-xcompose`

## Writing kana

You have a key set up for all of the normal kana (without dakuten).
Applying dakuten is done by pressing the dakuten button (left of right shift)
and then pressing a button for a kana that supports the dakuten. Press the
dakuten button twice to get a combining character that works with all characters.
Writing small kana is done by simply pressing the compose key
and then the kana that should be small. Not all kana support this
with this layout yet.

## Writing full-width characters

Just hold down RALT and you have a normal QWERTY layout.
Not all non-alphanumeric characters are supported yet.

## Writing kanji

### Step 1

Press Caps-Lock

### Step 2

Enter the first kana of the first on'yomi reading of the kanji (in
some arbitrary order decided by kanjidic), DISREGARDING any
dakuten/voicedness, e.g. さ instead of ざ.

### Step 3

You then specify what the ending of the of the reading is,
which can be any of the following:

|1 - 9|か-へ|さ-め|た-れ|
|-----|-----|-----|-----|
|イ   |ュウ |イ   |ュウ |
|ウ   |ュク |ウ   |ュク |
|キ   |ュツ |キ   |ュツ |
|ク   |ュン |ク   |ュン |
|チ   |ョ   |チ   |ョ   |
|ツ   |ョウ |ツ   |ョウ |
|ャ   |ョク |ャ   |ョク |
|ャク |ン   |ャク |ン   |
|ュ   |none |ュ   |none |

As you can probably tell, the rows are duplicated once,
but the duplicate indicates that the **first** kana is
voiced, i.e. has dakuten.
In addition, they are sorted alphabetically, so if you
can remember what endings are valid, then you should
be able to figure out what key an ending is.

### Step 4

Enter the number of strokes.
The first row is 1-10, the second 11-20, etc.

### Step 5

If multiple kanji match this criteria, you need to select which
to use using the numeric keys again, sorted by frequency.
If there is only one solution, then you do not need to select
anything here.
