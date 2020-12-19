data Rule = Token Char | Tokens [Char] | And [Rule] | Or [Rule]

exampleRules = lines "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: 'a'\n5: 'b'\n"

exampleStrings = lines "ababbb\nbababa\nabbbab\naaabbb\naaaabbb"

parseRules