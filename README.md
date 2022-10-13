# <h1 align="center"> exfac.rs </h1>

*Rust bindings & CLI to the ExFac API*

## CLI Usage

### Onboarding

First, get your API Key by running `conduit login`, which will prompt you to log in via the website.

Then, run `conduit user`, and retrieve the `organization` field.

```
export API_KEY=<...>
export ORGANIZATION=<..>
```

List all existing networks: `conduit network list`. This should be empty.

### Network

You can create a network as follows: `conduit network create --name demo`.

If you list the networks again, you'll see your network and its uuid under the `testnet` field.

You can delete the network by doing `conduit network delete --network <testnet uuid>`

### Job Templates

You can list all job templates by doing: `conduit job-template list`. This should initially
have a template with `jobTemplate` uuid equal to all zeroes. This is the default Foundry
template and cannot be modified. 

You can create a template via `conduit job-template create-or-update --repository <...> --name <...>`. This will proceed
to clone the specified repository and create a job template with that name. Now if we list the job templates
again we will see the job template we created is available.

### Jobs

Once a job template is created, it can be assigned to a specific network as a job. Jobs can be run:
1. On Start
2. On End
3. On Demand
4. [TODO] On Interval


You can assign a job by doing:: `conduit job assign --template <...> --network <...> --name <...> --type <0|1|2>`

This will take a previously created job template and assign it to the specified network with a name and a schedule.
The network parameter should be the same uuid as the one shown above for the `delete` command.

You can further configure jobs such as overriding the command that gets run.

You can also view the status of all running and historical jobs with `conduit job list|status`.
