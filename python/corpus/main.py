from nltk.corpus import gutenberg
from nltk.text import Text

print(gutenberg.raw("carroll-alice.txt")[:255])

print(gutenberg.sents("carroll-alice.txt")[0:3])

book = Text(gutenberg.words("carroll-alice.txt"))
print("words: %d" % len(book))
print("unique words: %d" % len(set(book)))
print(book.concordance("Alice"))

