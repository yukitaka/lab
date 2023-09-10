import tkinter as tk

class EmptyUI(tk.Frame):
    def __init__(self, master=None):
        tk.Frame.__init__(self, master)
        self.master.title('Hello World')

        self.c = tk.Canvas(self, width = 240, height = 240, highlightthickness = 0)
        self.c.pack()


class GraphicUI(tk.Frame):
    def __init__(self, master=None):
        tk.Frame.__init__(self, master)

        self.master.title('Draw Graphic')

        self.c = tk.Canvas(self, width = 240, height = 240, highlightthickness = 0)
        self.c.pack()

        self.on_draw()

    def on_draw(self):
        self.c.delete('all')
        self.c.create_line(10, 30, 230, 30, width = 2.0, fill = '#FF0000')
        self.c.create_oval(10, 70, 50, 110, width = 2.0, outline = '#00FF00')
        self.c.create_oval(70, 70, 110, 110, width = 0.0, fill = '#00FF00')
        self.c.create_rectangle(10, 130, 50, 170, width = 2.0, outline = '#00A0FF')
        self.c.create_rectangle(70, 130, 110, 170, width = 0.0, fill = '#00A0FF')
        self.c.create_text(10, 200, text = 'Hello World', font='courier 20', anchor = tk.NW)


f = GraphicUI()
f.pack()
f.mainloop()
