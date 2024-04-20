import json

# Read original resources.js file
with open('resources.json', 'r') as file:
    data = file.read()

# Parse JSON data
items = json.loads(data)

# Extract necessary information for each item
extracted_items = []
for item in items:
    extracted_item = {
        "name": item.get("name", ""),
        "type": item.get("type", ""),
        "rarity": item.get("rarity", None)
    }
    extracted_items.append(extracted_item)

# Rewrite resources.js file with only necessary data
with open('resources.js', 'w') as file:
    json.dump(extracted_items, file, indent=4)
