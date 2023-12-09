words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
left = dict()
right = dict()

def TrieInsert(trie, word, i, val):
    if i == len(word):
        trie['.'] = val
        return
    
    if word[i] not in trie:
        trie[word[i]] = dict()
    
    TrieInsert(trie[word[i]], word, i+1, val)

for i, word in enumerate(words):
    TrieInsert(left, word, 0, i)
    TrieInsert(right, word[::-1], 0, i)

with open('input.txt', 'r') as f:
    lines = f.read().split('\n')

total = 0
for line in lines:
    lefttrie = left
    leftchar = 0
    for char in line:
        if '.' in lefttrie:
            leftchar = lefttrie['.']
            break

        if char in '0123456789':
            leftchar = int(char)
            break

        if char in lefttrie:
            lefttrie = lefttrie[char]

        else:
            lefttrie = left
            if char in lefttrie:
                lefttrie = lefttrie[char]


    righttrie = right
    rightchar = 0
    for char in line[::-1]:
        if '.' in righttrie:
            rightchar = righttrie['.']
            break
        
        if char in '0123456789':
            rightchar = int(char)
            break

        if char in righttrie:
            righttrie = righttrie[char]
        else:
            righttrie = right
            if char in righttrie:
                righttrie = righttrie[char]
    print(line, leftchar, rightchar)
    total += leftchar*10 + rightchar

print(total)