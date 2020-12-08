import re
import sys

validPart1 = 0
validPart2 = 0
pattern = '^(\d+)-(\d+)\s+(.):\s+(.*)$'
for line in map(lambda l: l.rstrip(), sys.stdin):

    match = re.search(pattern, line)
    first = int(match.group(1))
    second = int(match.group(2))
    char = match.group(3)[0]
    password = match.group(4)

    matching_chars = len([c for c in  match.group(4) if c == char])

    if first <= matching_chars and matching_chars <= second:
        validPart1 += 1

    if len(password) > (first-1) and password[first-1] == char:
        if len(password) <= (second-1) or password[second-1] != char:
            validPart2 += 1
    else:
        if len(password) > (second-1) and password[second-1] == char:
            validPart2 += 1

print(f'part 1: {validPart1}')
print(f'part 2: {validPart2}')
