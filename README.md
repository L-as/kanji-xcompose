# Kanji-XCompose

This is a project for writing kana and composing kanji with a normal .XCompose file and an XKB layout.

# Why?

IBus etc. allow you to compose kanji by writing the pronunciation, then clicking
a button until you find the right kanji. This is however problematic, because you
can not know how many times you have to click since the sorting is "smart".
Even if it wasn't smart however, clicking e.g. space 20 times in a row isn't optimal.

# Usage

## Set-up

Just pipe in the kanjidic2.xml file and it will spit out a compose file.
You will also need to use the corresponding XKB layout when attempting
to compose kanji. To use the XKB layout, simply put the file into ~/.xkb/symbols/
with a fitting name (e.g. "kanji-xcompose") and switch to that layout.
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

You start by pressing Caps-Lock, then selecting the first kana
of the pronunciation of the letter disregarding voicedness, then
you select the ending of the pronunciation which also chooses voicedness,
then you choose the number of strokes, and if necessary, which letter
that fits all these criteria sorted with frequency of use.

As for selecting the ending, 9 keys are used of each row, with the bottom
two rows indicating that the beginning of the pronunciation is voiced.
The endings are then:
- イ,ウ,キ,ク,チ,ツ,ャ,ャク,ュ
- ュウ,ュク,ュツ,ュン,ョ,ョウ,ョク,ン,(no ending)
