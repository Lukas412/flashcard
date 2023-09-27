import functools
import tkinter as tk


class FlashCardApp(tk.Tk):
    STATE_UNCOVERED = 'uncovered'
    STATE_COVERED = 'covered'

    def __init__(self):
        super().__init__()

        self.title('FlashCards')

        self.grid_rowconfigure(0, minsize=32)
        self.grid_rowconfigure(1, weight=1)
        self.grid_rowconfigure(2, minsize=32)
        self.grid_rowconfigure(3, pad=8)
        self.grid_columnconfigure(4, pad=32)
        self.grid_columnconfigure(0, minsize=48)
        self.grid_columnconfigure(1, weight=1, pad=8)
        self.grid_columnconfigure(2, weight=1, pad=8)
        self.grid_columnconfigure(3, minsize=48)

        self.text = tk.Label(self, text='Hier kommt die Frage hin.')
        self.text.grid(row=1, column=1, columnspan=2)

        self.uncover = HideAbleButton(self, text='Karte aufdecken')
        self.uncover.grid(row=3, column=1, columnspan=2)

        self.wrong = (HideAbleButton(self, text='Falsch')
                      .grid(row=3, column=1)
                      .hide())
        self.right = (HideAbleButton(self, text='Richtig')
                      .grid(row=3, column=2)
                      .hide())

        self.bind('<KeyPress>', self.on_key_press)

        self.state = None
        self.set_covered_state()

    def set_covered_state(self):
        if self.state not in (None, self.STATE_UNCOVERED):
            return
        self.state = self.STATE_COVERED
        self.uncover.show()
        self.wrong.hide()
        self.right.hide()

    def set_uncovered_state(self):
        if self.state not in (self.STATE_COVERED, ):
            return
        self.state = 'uncovered'
        self.uncover.hide()
        self.wrong.show()
        self.right.show()

    def is_covered(self):
        return self.state == self.STATE_COVERED

    def is_uncovered(self):
        return self.state == self.STATE_UNCOVERED

    def on_key_press(self, event):
        if event.char == 'q':
            self.quit()
        if self.is_covered():
            if event.char == ' ':
                self.set_uncovered_state()
        if self.is_uncovered():
            if event.char == 'f':
                self.set_covered_state()
            if event.char == 'j':
                self.set_covered_state()


class HideAbleButton(tk.Button):
    @functools.wraps(tk.Button.__init__)
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._grid_options = [], {}

    @functools.wraps(tk.Button.grid)
    def grid(self, *args, **kwargs):
        super().grid(*args, **kwargs)
        self._grid_options = args, kwargs
        return self

    def hide(self):
        self.grid_forget()
        return self

    def show(self):
        args, kwargs = self._grid_options
        super().grid(*args, **kwargs)
        return self


if __name__ == '__main__':
    app = FlashCardApp()
    app.mainloop()
