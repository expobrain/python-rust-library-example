# -*- coding: utf8 -*-

import example

print(example.hello())
print(example.greetings(u'ネコ'))
try:
    print(example.greetings(u'\ud83f'))
except UnicodeDecodeError:
    print("Invalid unicode character")
