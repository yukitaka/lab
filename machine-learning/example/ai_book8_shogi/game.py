import random
import math

class State:
    def __init__(self, pieces=None, enemy_pieces=None, depth=0):
        self.dxy = ((0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1))

        self.pieces = pieces if pieces != None else [0] * (12+3)
        self.enemy_pieces = enemy_pieces if enemy_pieces != None else [0] * (12+3)
        self.depth = depth

        if pieces == None or enemy_pieces == None:
            self.pieces = [0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 4, 3, 0, 0, 0]
            self.enemy_pieces = [0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 4, 3, 0, 0, 0]

    def is_lose(self):
        for i in range(12):
            if self.pieces[i] == 4:
                return False
        return True

    def is_draw(self):
        return self.depth >= 300

    def is_done(self):
        return self.is_lose() or self.is_draw()

    def pieces_array(self):
        def pieces_array_of(pieces):
            table_list = []
            for j in range(1, 5):
                table = [0] * 12
                table_list.append(table)
                for i in range(12):
                    if pieces[i] == j:
                        table[i] = 1

            for j in range(1, 4):
                flag = 1 if pieces[11+j] > 0 else 0
                table = [flag] * 12
                table_list.append(table)
            return table_list

        return [pieces_array_of(self.pieces), pieces_array_of(self.enemy_pieces)]

    def position_to_action(self, position, direction):
        return position * 11 + direction

    def action_to_position(self, action):
        return (int(action/11), action%11)

    def legal_actions(self):
        actions = []
        for p in range(12):
            if self.pieces[p] != 0:
                actions.extend(self.legal_actions_pos(p))

            if self.pieces[p] == 0 and self.enemy_pieces[11-p] == 0:
                for capture in range(1, 4):
                    if self.pieces[11+capture] != 0:
                        actions.append(self.position_to_action(p, 8-1+capture))
        return actions

    def legal_actions_pos(self, position_src):
        actions = []

        piece_type = self.pieces[position_src]
        if piece_type > 4:
            piece_type-=4
        directions = []
        if piece_type == 1:
            directions = [0]
        elif piece_type == 2:
            directions = [1, 3, 5, 7]
        elif piece_type == 3:
            directions = [0, 2, 4, 6]
        elif piece_type == 4:
            direction = [0, 1, 2, 3, 4, 5, 6, 7]

        for direction in directions:
            x = position_src%3 + self.dxy[direction][0]
            y = int(position_src/3) + self.dxy[direction][1]
            p = x + y * 3

            if 0 <= x and x <= 2 and 0 <= y and y <= 3 and self.pieces[p] == 0:
                actions.append(self.position_to_action(p, direction))
        return actions

    def next(self, action):
        state = State(self.pieces.copy(), self.enemy_pieces.copy(), self.depth+1)
        position_dst, position_src = self.action_to_position(action)

        if position_src < 8:
            x = position_dst%3 - self.dxy[position_src][0]
            y = int(position_dst/3) - self.dxy[position_src][1]
            position_src = x + y * 3

            state.pieces[position_dst] = state.pieces[position_src]
            state.pieces[position_src] = 0

            piece_type = state.enemy_pieces[11-position_dst]
            if piece_type != 0:
                if piece_type != 4:
                    state.pieces[11+piece_type] += 1
                state.enemy_pieces[11-position_dst] = 0
        else:
            capture = position_src-7
            state.pieces[position_dst] = capture
            state.pieces[11+capture] -= 1
        w = state.pieces
        state.pieces = state.enemy_pieces
        state.enemy_pieces = w
        return state

    def is_first_player(self):
        return self.depth%2 == 0

    def __str__(self):
        pieces0 = self.pieces if self.is_first_player() else self.enemy_pieces
        pieces1 = self.enemy_pieces if self.is_first_player() else self.pieces
        hzkr0 = ('', 'H', 'Z', 'K', 'R')
        hzkr1 = ('', 'h', 'z', 'k', 'r')

        str = '['
        for i in range(12, 15):
            if pieces1[i] >= 2:
                str += hzkr1[i-11]
            if pieces1[i] >= 1:
                str += hzkr1[i-11]
        str += ']\n'

        for i in range(12):
            if pieces0[i] != 0:
                str += hzkr0[pieces0[i]]
            elif pieces1[11-i] != 0:
                str += hzkr1[pieces1[11-i]]
            else:
                str += '-'
            if i%3 == 2:
                str += '\n'
        str += '['
        for i in range(12, 15):
            if pieces0[i] >= 2:
                str += hzkr0[i-11]
            if pieces0[i] >= 1:
                str += hzkr0[i-11]
        str += ']\n'
        return str


def random_action(state):
    legal_actions = state.legal_actions()
    return legal_actions[random.randint(0, len(legal_actions)-1)]


if __name__ == '__main__':
    state = State()

    while True:
        if state.is_done():
            break

        state = state.next(random_action(state))

        print(state)
        print("")
