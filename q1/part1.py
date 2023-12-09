with open('input.txt', 'r') as f:
    lines = f.read().split('\n')

total = 0
for line in lines:
    start = ''
    end = ''

    for char in line:
        if char in '0123456789':
            start = char
            break

    for char in line[::-1]:
        if char in '0123456789':
            end = char
            break

    total += int(start+end)

print(total)