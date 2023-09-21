import json
import random

with open('material/german/process_description.json', encoding='utf8') as file:
    topic = json.load(file)

topic_name, cards = topic['name'], topic['cards']

while True:
    random_card = random.choice(cards)

    print('\n' * 40)
    question, answer = random_card['question'], random_card['answer']
    print(f'### Question ###')
    print(topic_name)
    print()
    print(question)
    input()
    print(f'### Answer ###\n{answer}')
    input()
