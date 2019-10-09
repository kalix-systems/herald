
## headless herald
---
this small executable runs select actions from herald core.
on failure of any action it exits with an error code equal to the scheduled
order of that action.


### Script
---
script is a misnomer, this program takes JSON files which include a list of action to attemp to complete or fail trying.

example invocation:

```bash
~ cargo run -- -i scripts/bob_send_message.json
```

scripts/bob_send_message.json
---
```JSON
{
    "userid": "Bob",
    "actions": [
        {
            "action_type": "Send",
            "to": "Alice",
            "body": "Hello Alice"
        },
        {
            "type": "Await",
            "what": "MessageAck",
            "from": "Alice",
            "timeOut": 2
        }
    ]
}
```

## Supported Actions 
---