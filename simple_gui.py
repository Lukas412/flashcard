import functools
import tkinter as tk


def main():
    root = tk.Tk()
    root.title('FlashCards')
    root.grid_rowconfigure(0, minsize=32)
    root.grid_rowconfigure(1, weight=1)
    root.grid_rowconfigure(2, minsize=32)
    root.grid_rowconfigure(3, pad=8)
    root.grid_columnconfigure(4, pad=32)
    root.grid_columnconfigure(0, minsize=48)
    root.grid_columnconfigure(1, weight=1, pad=8)
    root.grid_columnconfigure(2, weight=1, pad=8)
    root.grid_columnconfigure(3, minsize=48)

    root.text = tk.Label(root, text='Hier kommt die Frage hin.')
    root.text.grid(row=1, column=1, columnspan=2)

    root.uncover = HideAbleButton(root, text='Karte aufdecken')
    root.uncover.grid(row=3, column=1, columnspan=2)

    root.after(1000, lambda: root.uncover.hide())
    root.after(2000, lambda: root.uncover.show())
    root.after(3000, lambda: root.uncover.hide())
    root.after(4000, lambda: root.uncover.show())

    root.mainloop()


class HideAbleButton(tk.Button):
    @functools.wraps(tk.Button.__init__)
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._grid_options = [], {}

    @functools.wraps(tk.Button.grid)
    def grid(self, *args, **kwargs):
        super().grid(*args, **kwargs)
        self._grid_options = args, kwargs

    def hide(self):
        self.grid_forget()

    def show(self):
        args, kwargs = self._grid_options
        super().grid(*args, **kwargs)


if __name__ == '__main__':
    main()
