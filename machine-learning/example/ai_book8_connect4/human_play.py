from game import State
from pv_mcts import pv_mcts_action
from tensorflow.keras.models import load_model
from pathlib import Path
from threading import Thread
import tkinter as tk


model = load_model('./model/best.keras')

class GameUI(tk.Frame):
    def __init__(self, master=None, model=None):
        tk.Frame.__init__(self, master)
        self.master.title('Connect 4')
        self.state = State()
        self.next_action = pv_mcts_action(model, 0.0)
        self.c = tk.Canvas(self, width = 280, height = 240, highlightthickness = 0)
        self.c.bind('<Button-1>', self.turn_of_human)
        self.c.pack()

        self.on_draw()

    def turn_of_human(self, event):
        if self.state.is_done():
            self.state = State()
            self.on_draw()
            return

        if not self.state.is_first_player():
            return

        x = int(event.x/40)
        if x < 0 or 6 < x:
            return
        action = x

        if not (action in self.state.legal_actions()):
            return

        self.state = self.state.next(action)
        self.on_draw()

        self.master.after(1, self.turn_of_ai)

    def turn_of_ai(self):
        if self.state.is_done():
            return

        action = self.next_action(self.state)
        self.state = self.state.next(action)
        self.on_draw()

    def draw_piece(self, index, first_player):
        x = (index%7)*40+5
        y = int(index/7)*40+5
        if first_player:
            self.c.create_oval(x, y, x+30, y+30, width = 1.0, outline= '#FF0000')
        else:
            self.c.create_oval(x, y, x+30, y+30, width = 1.0, fill = '#FFFF00')

    def on_draw(self):
        self.c.delete('all')
        self.c.create_rectangle(0, 0, 280, 240, width = 0.0, fill = '#00A0FF')
        for i in range(42):
            x = (i%7)*40+5
            y = int(i/7)*40+5
            self.c.create_oval(x, y, x+30, y+30, width = 1.0, fill = '#FFFFFF')

        for i in range(42):
            if self.state.pieces[i] == 1:
                self.draw_piece(i, self.state.is_first_player())
            if self.state.enemy_pieces[i] == 1:
                self.draw_piece(i, not self.state.is_first_player())


f = GameUI(model=model)
f.pack()
f.mainloop()

