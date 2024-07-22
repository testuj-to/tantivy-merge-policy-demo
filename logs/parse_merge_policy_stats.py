import sys
import os
import json

log_file_path = sys.argv[1]

if not os.path.isfile(log_file_path):
    raise Exception(f'Invalid log file: "{log_file_path}"')

stats = {
    'calls': 0,
    'lengths': {},
}

for line in open(log_file_path):
    try:
        line_json = json.loads(line)

        if type(line_json['run']) != str:
            continue

        count = int(line_json['count'])

        stats['calls'] += 1

        if line_json['count'] not in stats['lengths']:
            stats['lengths'][line_json['count']] = 0

        stats['lengths'][line_json['count']] += 1
    except:
        print(f'Non merge policy stat line: "{line.strip()}"')

print(json.dumps(stats, indent=2))
