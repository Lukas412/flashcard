import json
import random

with open('material/german/process_description.json', encoding='utf8') as file:
    topic = json.load(file)

topic_name, cards = topic['name'], topic['cards']

try:
    while True:
        random_card = random.choice(cards)
        question, correct_answer = random_card['question'], random_card['answer']

        print('\n' * 40)
        print(f'### Question ###')
        print(topic_name)
        print()
        print(question)
        real_answer = input()
        print(f'### Answer ###\n{correct_answer}')
        if not real_answer or real_answer.isspace():
            input()
        else:
            correctness = input('where you right?')
            if 'corrections' not in random_card:
                random_card['corrections'] = []
            random_card['corrections'].append({
                'real': real_answer,
                'correctness': correctness
            })
finally:
    with open('material/german/process_description.json', mode='w', encoding='utf8') as file:
        json.dump(topic, file, indent=2, ensure_ascii=False)
