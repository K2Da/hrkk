# hrkk
hrkk lists up aws resources and select some with fuzzy finder(skim), and print its id like ec2 instance-id to console.

Use it to input parameter for aws cli, or just to see aws resource status in console.

![ss](https://user-images.githubusercontent.com/367828/85136202-40bb6a80-b27a-11ea-9fc8-aca763d9f1ad.gif)

### current available resource types

- ssm : session, document, automation-execution
- rds : db-instance
- logs : log-stream, log-group
- ec2 :instance
- cloudwatch : alarm-history, alarm


## Installation
### Using cargo

```bash
cargo install hrkk
```

### Using binary

Get the binary in release page and copy it to the directory in the PATH environment variable.
Binaries are for linux and mac.