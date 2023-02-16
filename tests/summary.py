import sys
from datetime import datetime

labels = [
    'mu:',
    'core:',
]

with open(sys.argv[1]) as f: test_results = f.readlines()
date = datetime.now().strftime('%m/%d/%Y %H:%M:%S')

print(f'Test Summary: {date:<10}')
print('-----------------------')

for label in labels:
    test_total = 0
    test_fails = 0
    test_aborts = 0
    for test in test_results:
        fields=test[:-1].split(',')
        if fields[0] == label:
            print(f'{fields[0]:<10} {fields[1]:<14} total: {fields[2]:<8} failed: {fields[3]:<8} aborted: {fields[4]:<8}')
            test_total += int(fields[2])
            test_fails += int(fields[3])
            test_aborts += int(fields[4])

    test_passes = test_total - (test_fails + test_aborts)
    print('-----------------------')
    print(f'{label:<11}', end='')
    print(f'passed: {test_passes:<7}', end='')
    print(f'total: {test_total:<9}', end='')
    print(f'failed: {test_fails:<9}', end='')
    print(f'aborted: {test_aborts:<10}')
    print()
