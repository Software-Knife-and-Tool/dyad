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

line_no = 0
for test in test_results:
    global totals

    fields=test[:-1].split(",")

    if fields[0] in labels:
        totals = fields
    else:
        form = (fields[0][:27] + '...') if len(fields[0]) > 30 else fields[0]
        if len(fields) != 4:
            if len(fields) == 1:
                pass
            else:
                if len(fields) == 2:
                    if fields[0].find('panicked') != -1:
                        pass
                    elif fields[0].find('exception') != -1:
                        pass
                    else:
                        line_no += 1
                        print(f'{line_no:>3} {form:<30} aborted')
        else:
            line_no += 1
            print(f'{line_no:>3} {form:<30} {fields[1]:<15} {fields[2]:<15} {fields[3]:<15}')
print('----------------------')
print(f'{totals[0]} {totals[1]:<26} total: {totals[2]:<8} failed: {totals[3]:<8} aborted: {totals[4]:<8}')    
