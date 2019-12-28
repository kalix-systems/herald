#!/usr/bin/env python3
import json
import requests

ouput_path = "../src/emoji_keyboard/picker_struct.rs"
emoji_struct_f_string = open("templates/emoji_struct.rs", "r").read()
rust_module_f_string = open("templates/rust_module.rs", "r").read()

# url for emoji data
emoji_json_url = "https://raw.githubusercontent.com/github/gemoji/master/db/emoji.json"

# the underlying json is structured as
# List<Object>
# Where Object has fields
# {
#   emoji: Char
#   description: String
#   category: String
#   aliases: List<String>
#   tags: List<String>
#   unicode_version: Float -- unused
#   ios_version: Float -- unused
# }


def format_struct(json_emoji_data_obj):
    return emoji_struct_f_string.format(emoji=json_emoji_data_obj["emoji"], comma_seperated_tags=json_emoji_data_obj["tags"])


def output_to_file():
    print("pushing output to file")


def combine_tags(obj):
    obj["tags"].extend(
        list(map(lambda x: x.replace("_", " "), obj["aliases"])))
    obj["tags"].insert(0, obj["description"])
    obj["tags"] = list(dict.fromkeys(obj["tags"]))
    return obj["tags"]


def main():
    print("Launching requests")

    raw_json = requests.get(emoji_json_url)

    raw_obj = list(map(lambda obj: {
                   "emoji": "\"" + obj["emoji"] + "\"", "tags": combine_tags(obj)}, json.loads(raw_json.text)))
    struct_strings = [format_struct(i) for i in raw_obj]

    struct_strings = ",".join(struct_strings)

    struct_strings = "[\n" + struct_strings + "]"

    module_string = rust_module_f_string.format(
        emoji_data=struct_strings, length=len(raw_obj))

    open(ouput_path, "w").write(module_string)


if __name__ == "__main__":
    main()
