import json
import os
import subprocess
import xml.etree.ElementTree as ET
import re

with open("tsconfig.json") as f:
    tsconfig = json.load(f)

subprocess.run(["tsc"])

files = map(
    lambda parts: parts[0],
    filter(
        lambda parts: len(parts) == 2,
        map(lambda s: s.split("."), set(tsconfig["files"])),
    ),
)

mjs_files = set()

for file in files:
    dest = file + ".mjs"
    mjs_files.add(dest)

    os.renames(file + ".js", dest)

with open("qml.qrc") as f:
    root = ET.fromstring(f.read())

qrc_mjs_files = set()
mjs_pattern = re.compile(r".*\.mjs")

for file_name in root.iter("file"):
    inner_text = next(file_name.itertext())
    if (inner_text is not None) and mjs_pattern.match(inner_text):
        qrc_mjs_files.add(inner_text)

qresource = next(root.iter("qresource"))

diff = mjs_files.difference(qrc_mjs_files)

for file in diff:
    file_elem = ET.SubElement(qresource, "file")
    file_elem.text = file
# fix the formatting by hand, because this project has enough
# languages already
lines = ET.tostring(root, encoding="unicode").splitlines()

lines[-2] = lines[-2].replace("><", ">\n<")

parts = lines[-2].splitlines()

indent = " " * 4
double_ident = indent * 2

for i in range(0, len(parts) - 1):
    parts[i] = parts[i].replace(indent, indent * 2)

parts[-1] = indent + parts[-1]

lines[-2] = "\n".join(parts)

with open("qml.qrc", "w") as f:
    f.write("\n".join(lines))
