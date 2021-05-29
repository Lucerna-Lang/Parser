import re

with open("Cargo.lock", 'r') as f:
    lines = f.readlines()

PARSER_VER = "UNKNOWN"
STRUCTURES_VER = "UNKNOWN"
LIBER_VER = "UNKNOWN"
WEB_VER = "UNKNOWN"
i = 0
for i in range(len(lines)):
    line = lines[i]
    if line.startswith("name") :
        if "lucerna" in line:
            PARSER_VER = re.search(r'[\'"]([^\'"]*)[\'"]', lines[i+1]).group(1)
        if "structures" in line:
            STRUCTURES_VER = re.search(r'[\'"]([^\'"]*)[\'"]', lines[i+1]).group(1)
        if "liber" in line:
            LIBER_VER = re.search(r'[\'"]([^\'"]*)[\'"]', lines[i+1]).group(1)
        if "name = \"web\"" in line:
            WEB_VER = re.search(r'[\'"]([^\'"]*)[\'"]', lines[i+1]).group(1)

print("Versions:")
print("{} = v{}\n{} = v{}\n{} = v{}\n{} = v{}".format("parser", PARSER_VER, "structures", STRUCTURES_VER, "liber", LIBER_VER, "web", WEB_VER))
