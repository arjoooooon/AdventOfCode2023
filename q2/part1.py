with open('input.txt', 'r') as f:
    lines = f.read().split('\n')

cmap = {'red': 12, 'green': 13, 'blue': 14}

total = 0
power = 0
for line in lines:
    game, moveline = line.split(':')
    id_ = int(game.split(' ')[1])

    maxcolors = {
        'red': 0,
        'blue': 0,
        'green': 0
    }

    moves = moveline.split(';')
    flag = True
    for move in moves:
        blocks = move.split(',')
        # print(blocks)

        for block in blocks:
            number, color = int(block.split(' ')[1]), block.split(' ')[2] 
            maxcolors[color] = max(maxcolors[color], number)
            
            if number > cmap[color]:
                flag = False

    if flag:
        total += id_
    
    this_power = maxcolors['red'] * maxcolors['blue'] * maxcolors['green']
    # print(line, this_power)
    power += this_power

print(total, power)