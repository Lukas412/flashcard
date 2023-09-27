import csv
import functools
import os.path
import pathlib
import pickle
import random
import tkinter as tk
from dataclasses import dataclass
from tkinter import font


class Theme:
    @classmethod
    def title_font(cls):
        return font.Font(size=20, underline=True)

    @classmethod
    def text_font(cls):
        return font.Font(size=16)

    @classmethod
    def action_font(cls):
        return font.Font(size=10)


class FlashCardApp(tk.Tk):
    STATE_UNCOVERED = 'uncovered'
    STATE_COVERED = 'covered'

    def __init__(self, path):
        super().__init__()

        self.store = FlashCardStore(path)
        self.store.load()
        self.card = None

        self.title('FlashCards')

        self.grid_rowconfigure(0, minsize=80)
        self.grid_rowconfigure(8, minsize=20)
        self.grid_rowconfigure(10, weight=1)
        self.grid_rowconfigure(15, minsize=48)
        self.grid_rowconfigure(20, pad=8)
        self.grid_rowconfigure(25, minsize=32)

        self.grid_columnconfigure(0, minsize=32)
        self.grid_columnconfigure(1, weight=1, pad=8, minsize=150)
        self.grid_columnconfigure(2, weight=1, pad=8, minsize=150)
        self.grid_columnconfigure(3, minsize=32)
        self.grid_columnconfigure(4, pad=32)

        self.piles_label = tk.Label(self, text='', justify=tk.RIGHT)
        self.piles_label.grid(row=0, column=2, columnspan=2, sticky='ne')

        self.title_label = tk.Label(self, text='', justify=tk.LEFT, font=Theme.title_font())
        self.title_label.grid(row=5, column=1, columnspan=2, sticky='nw')

        self.pile_label = tk.Label(self, text='', justify=tk.LEFT)
        self.pile_label.grid(row=4, column=1, sticky='nw')

        self.text_label = tk.Label(self, text='', justify=tk.LEFT, wraplength=300, font=Theme.text_font())
        self.text_label.grid(row=10, column=1, columnspan=2, sticky='nw')

        self.uncover_button = (HideAbleButton(self, text='Aufdecken <Space>', font=Theme.action_font(), border=0)
                               .grid(row=20, column=1, columnspan=2, sticky='nw'))

        self.wrong_button = (HideAbleButton(self, text='Falsch <F>', font=Theme.action_font(), border=0)
                             .grid(row=20, column=1, sticky='nw')
                             .hide())
        self.right_button = (HideAbleButton(self, text='Richtig <J>', font=Theme.action_font(), border=0)
                             .grid(row=20, column=2, sticky='nw')
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
        self.title_label.configure(text='Frage:')
        self.pile_label.configure(text=f'Aus Stapel {self.card.pile + 1}')
        self.text_label.configure(text=self.card.question)
        self.uncover_button.show()
        self.wrong_button.hide()
        self.right_button.hide()

    def set_uncovered_state(self):
        if self.state not in (self.STATE_COVERED,):
            return
        self.state = 'uncovered'
        self.title_label.configure(text='Antwort:')
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

    def save(self):
        self.store.add_card(self.card)
        self.store.save()


class FlashCardStore:

    def __init__(self, path):
        self.version = '1.0'
        self.path = path
        self.max_piles = 5
        self.piles = [[] for _ in range(self.max_piles)]

    @property
    def pickle_path(self):
        return pathlib.Path(self.path).with_suffix('.pkl')

    def load(self):
        if os.path.exists(self.pickle_path):
            with open(self.pickle_path, mode='rb') as file:
                prev_store = pickle.load(file)
            if prev_store.version != self.version:
                raise TypeError(f'Could not load old data, because the versions do not match:'
                                f' {prev_store.version} != {self.version}.')
            self.max_piles = prev_store.max_piles
            self.piles = prev_store.piles
            return
        with open(self.path, mode='r') as file:
            csv_content = csv.DictReader(file)
            self.piles[0].extend(FlashCard(pile=0, question=card['front'], answer=card['back']) for card in csv_content)

    def save(self):
        with open(self.pickle_path, mode='wb') as file:
            pickle.dump(self, file)

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
        self.add_card(card)

    def add_wrong_card(self, card):
        card.pile = max(0, card.pile - 1)
        self.add_card(card)

    def add_card(self, card):
        self.piles[card.pile].append(card)


@dataclass
class FlashCard:
    pile: int
    question: str
    answer: str


class HideAbleButton(tk.Label):

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._grid_options = [], {}

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
    app = FlashCardApp('FU0_2023-09-26_Klausurvorbereitung_Flashcards.csv')
    try:
        app.mainloop()
    finally:
        app.save()
