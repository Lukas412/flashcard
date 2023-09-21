import json
import random

with open('material/german/cards.json', encoding='utf8') as file:
    cards = json.load(file)

while True:
    random_card = random.choice(cards)

    print('\n' * 40)
    question, answer = random_card['question'], random_card['answer']
    print(f'### Question ###\n{question}')
    input()
    print(f'### Answer ###\n{answer}')
    input()

