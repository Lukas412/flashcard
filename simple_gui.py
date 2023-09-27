import csv
import functools
import random
import tkinter as tk
from dataclasses import dataclass


class Theme:
    pass


class FlashCardApp(tk.Tk):
    STATE_UNCOVERED = 'uncovered'
    STATE_COVERED = 'covered'

    def __init__(self):
        super().__init__()

        self.store = FlashCardStore()
        self.store.load_from_simple_csv('FU0_2023-09-26_Klausurvorbereitung_Flashcards.csv')
        self.card = None

        self.title('FlashCards')

        self.grid_rowconfigure(0, minsize=80)
        self.grid_rowconfigure(1, weight=1)
        self.grid_rowconfigure(2, minsize=32)
        self.grid_rowconfigure(3, pad=8)
        self.grid_rowconfigure(4, minsize=32)

        self.grid_columnconfigure(4, pad=32)
        self.grid_columnconfigure(0, minsize=48)
        self.grid_columnconfigure(1, weight=1, pad=8, minsize=150)
        self.grid_columnconfigure(2, weight=1, pad=8, minsize=150)
        self.grid_columnconfigure(3, minsize=48)

        self.piles_label = tk.Label(self, text='', justify=tk.RIGHT)
        self.piles_label.grid(row=0, column=2, columnspan=2, sticky='ne')

        self.text_label = tk.Label(self, text='Hier kommt die Frage hin.', wraplength=300, justify=tk.CENTER)
        self.text_label.grid(row=1, column=1, columnspan=2)

        self.uncover_button = HideAbleButton(self, text='Karte aufdecken\n<Space>')
        self.uncover_button.grid(row=3, column=1, columnspan=2)

        self.wrong_button = (HideAbleButton(self, text='Falsch\n<F>')
                             .grid(row=3, column=1)
                             .hide())
        self.right_button = (HideAbleButton(self, text='Richtig\n<J>')
                             .grid(row=3, column=2)
                             .hide())

        self.bind('<KeyPress>', self.on_key_press)

        self.state = None
        self.set_covered_state()

    def set_covered_state(self):
        if self.state not in (None, self.STATE_UNCOVERED):
            return
        self.state = self.STATE_COVERED
        self.piles_label.configure(text=self.store.format_pile_sizes())
        self.card = self.store.next_card()
        self.text_label.configure(text=self.card.question)
        self.uncover_button.show()
        self.wrong_button.hide()
        self.right_button.hide()

    def set_uncovered_state(self):
        if self.state not in (self.STATE_COVERED,):
            return
        self.state = 'uncovered'
        self.text_label.configure(text=self.card.answer)
        self.uncover_button.hide()
        self.wrong_button.show()
        self.right_button.show()

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
                self.store.add_wrong_card(self.card)
                self.set_covered_state()
            if event.char == 'j':
                self.store.add_right_card(self.card)
                self.set_covered_state()


class FlashCardStore:

    def __init__(self):
        self.max_piles = 5
        self.piles = [[] for _ in range(self.max_piles)]

    def load_from_simple_csv(self, path):
        with open(path, mode='r') as file:
            csv_content = csv.DictReader(file)
            self.piles[0].extend(FlashCard(pile=0, question=card['front'], answer=card['back']) for card in csv_content)

    def format_pile_sizes(self):
        return str.join('/', map(str, map(len, self.piles)))

    def next_card(self):
        pile_weights = tuple((self.max_piles - index) ** 2 * len(pile) for (index, pile) in enumerate(self.piles))
        random_pile = random.choices(self.piles, weights=pile_weights, k=1)[0]
        random_card = random.choice(random_pile)
        random_pile.remove(random_card)
        return random_card

    def add_right_card(self, card):
        card.pile = min(self.max_piles - 1, card.pile + 1)
        self._add_card(card)

    def add_wrong_card(self, card):
        card.pile = max(0, card.pile - 1)
        self._add_card(card)

    def _add_card(self, card):
        self.piles[card.pile].append(card)


@dataclass
class FlashCard:
    pile: int
    question: str
    answer: str


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
