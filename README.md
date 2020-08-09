# hrkk
hrkk lists up aws resources and select some with fuzzy finder, and print its resource id like ec2 instance-id to console.

Use it to input parameter for aws cli, or just to see aws resource status from console.

## usage
### see aws resources
Without sub command, hrkk shows all available resource types. Select with enter key to see the resources.

![commands](https://user-images.githubusercontent.com/367828/89723683-8b649200-da34-11ea-939a-b571977a547a.gif)

### to aws cli parameter
use hrkk like this to input aws cli parameter.

```sh
# tail cloudwatch logs
aws logs tail --since 100d $(hrkk logs log-group)

# tail -f
aws logs tail --follow $(hrkk -r 10 logs log-group)

# ec2 start session
aws ssm start-session --target $(hrkk ec2 instance)
```

![param](https://user-images.githubusercontent.com/367828/89723685-90294600-da34-11ea-9994-788a50719c43.gif)

## key bindings
small letters to filter left pane. Shift or Ctrl + letter for commands.

- a-z: small letters, numeric and symbols to filter list items
- ESC: clear filter, back to menu or quit this command
- O: open aws console in a browser for the selected resource
- G: get all resource detail with get api if the current resource has get api
- Enter: select resource to print the name and exit
- TAB: mark resource to select
- E: create yaml file of marked resources in the current directory
- A: fetch resources if there still have been resource to fetch
- R: reload resources
- Y: toggle viewer mode between yaml and summary
- BS: delete filtering texts
- ↑↓: move list(left side)
- B/F: move list(left side) 1/2 screen
- K/J: scroll viewer(right side)
- U/D: scroll viewer(right side) 1/2 screen
- L: popup log window
- H: popup help window
- V: popup viewer window
- C: quit this command

## current available resource types

- acm: certificate
- athena: query_execution
- autoscaling: auto_scaling_group
- cloudformation: stack
- cloudfront: distribution
- cloudwatch: alarm, alarm_history, dashboard, metric
- ec2: image, instance, launch_template, security_group, subnet, vpc
- elasticache: cache_cluster
- elastictranscoder: pipeline
- elb: load_balancer
- es: domain
- firehose: delivery_stream
- iam: group, mfa_device, policy, role, user,
- kinesis: stream
- lambda: function
- logs: log_group, log_stream
- rds: db_instance
- route53: hosted_zone, resource_record_set
- s3: bucket
- ssm: automation_execution, document, session

## auth
With rusoto_credential, hrkk use aws cli profile and credentials.

## Installation

### Using homebrew
With this command, download binary for mac/linux.

```
brew install K2Da/tap/hrkk
```

### Using cargo

```bash
cargo install hrkk
```
