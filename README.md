# Kanji-XCompose

This is a project for composing kanji with a normal .XCompose file.

# Why?

IBus etc. allow you to compose kanji by writing the pronunciation, then clicking
a button until you find the right kanji. This is however problematic, because you
can not know how many times you have to click since the sorting is "smart".
Even if it wasn't smart however, clicking e.g. space 20 times in a row isn't optimal.

# Usage

Just pipe in the kanjidic2.xml file and it will spit out a compose file.

You start by inserting the U+FFE0 char, then selecting the first kana
of the pronunciation of the letter disregarding voicedness, then
you select the ending of the pronunciation which also chooses voicedness,
then you choose the number of strokes, and if necessary, which letter
that fits all these criteria sorted with frequency of use.
