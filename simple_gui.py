import functools
import tkinter as tk


class FlashCardApp(tk.Tk):
    class State:
        COVERED = 'covered'
        UNCOVERED = 'uncovered'

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

        self.mainloop()

    def on_key_press(self, event):
        if event.char == 'q':
            self.quit()
        if event.char == ' ':
            print('aufdecken')
        if event.char == 'f':
            print('wrong')
        if event.char == 'j':
            print('right')


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
    main()
