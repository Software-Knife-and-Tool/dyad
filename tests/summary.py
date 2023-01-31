import sys
from datetime import datetime

labels = [
    "mu:",
    "core:",
#    "preface:",
]

with open(sys.argv[1]) as f: test_results = f.readlines()
date = datetime.now().strftime("%m/%d/%Y %H:%M:%S")

print(f"Test Summary: {date:<10}")
print("------------------------")
for label in labels:
    test_total = 0
    test_fails = 0
    for test in test_results:
        if test[0] == '-':
            continue
        if len(test.split()) != 4:
            print(test)
        else:
            test, name, total, failures = enumerate(test.split())
            print(f"{test[1]:<5} {name[1]:<20} {total[1]:<4} {failures[1]:<4}")
            if test[1] == label:
                test_total += int(total[1])
                test_fails += int(failures[1])

    print("------------------------")
    print(f"{label:<6}", end="")
    print(f"total {test_total:<11}", end="")
    print(f"failures {test_fails:<10}")
