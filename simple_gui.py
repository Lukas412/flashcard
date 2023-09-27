import csv
import functools
import random
import tkinter as tk
from dataclasses import dataclass
from itertools import repeat


class FlashCardApp(tk.Tk):
    STATE_UNCOVERED = 'uncovered'
    STATE_COVERED = 'covered'

    def __init__(self):
        super().__init__()

        self.flash_card_store = FlashCardStore()
        self.flash_card_store.load_from_simple_csv('FU0_2023-09-26_Klausurvorbereitung_Flashcards.csv')
        self.card = None

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

        self.uncover = HideAbleButton(self, text='Karte aufdecken\n<Space>')
        self.uncover.grid(row=3, column=1, columnspan=2)

        self.wrong = (HideAbleButton(self, text='Falsch\n<F>')
                      .grid(row=3, column=1)
                      .hide())
        self.right = (HideAbleButton(self, text='Richtig\n<J>')
                      .grid(row=3, column=2)
                      .hide())

        self.bind('<KeyPress>', self.on_key_press)

        self.state = None
        self.set_covered_state()

    def set_covered_state(self):
        if self.state not in (None, self.STATE_UNCOVERED):
            return
        self.state = self.STATE_COVERED
        self.card = self.flash_card_store.next_card()
        self.text.configure(text=self.card.question)
        self.uncover.show()
        self.wrong.hide()
        self.right.hide()

    def set_uncovered_state(self):
        if self.state not in (self.STATE_COVERED,):
            return
        self.state = 'uncovered'
        self.text.configure(text=self.card.answer)
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
                self.flash_card_store.add_wrong_card(self.card)
                self.set_covered_state()
            if event.char == 'j':
                self.flash_card_store.add_right_card(self.card)
                self.set_covered_state()


class FlashCardStore:

    def __init__(self):
        self.max_piles = 5
        self.piles = [*repeat([], times=self.max_piles)]

    def load_from_simple_csv(self, path):
        with open(path, mode='r') as file:
            csv_content = csv.DictReader(file)
            self.piles[0].extend(FlashCard(pile=0, question=card['front'], answer=card['back']) for card in csv_content)

    def next_card(self):
        piles_count = len(self.piles)
        pile_weights = tuple((piles_count - index) * 2 * len(self.piles[index]) for index in range(piles_count))
        print(pile_weights)
        random_pile = random.choices(self.piles, weights=pile_weights, k=1)[0]
        random_card = random.choice(random_pile)
        random_pile.remove(random_card)
        return random_card

    def add_right_card(self, card):
        card.pile = min(self.max_piles - 1, card.pile + 1)
        self._add_card(card)

    def add_wrong_card(self, card):
        card.pile = max(0, card - 1)
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
