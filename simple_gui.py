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

    root.show = tk.Button(root, text='Karte aufdecken')
    root.show.grid(row=3, column=1, columnspan=2)

    root.mainloop()


if __name__ == '__main__':
    main()
