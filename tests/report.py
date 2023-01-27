import sys
from datetime import datetime

labels = [
    'mu:',
    'core:',
    'preface:',
]

with open(sys.argv[1]) as f: test_results = f.readlines()
date = datetime.now().strftime('%m/%d/%Y %H:%M:%S')

print(f'Test Report: {date:<10}')
print('----------------------')

for test in test_results:
    if test[0] == '-':
        fields = test.split("--")
        if len(fields) != 3:
            print(test, end="")
        else:
            print(f'{fields[1]:<15} expected: {fields[2]:<15} result: {fields[3]:<15}', end="")
    else:
        if len(test.split()) != 3:
            print(test, end="")
        else:
            label = test.split()[0]
            source = test.split()[1]
            print(f'{label}:{source}',end="")
