# Commander
Microservice for executing shell commands via API requests

## Introduction
Commander acts like a scripting interface to any Linux computer.
The service is able to receive commands via its built-in API server and interprets them using the provided shell.
The main purpose of this software is to be used alongside Smarthome-server in order to allow Homescript to control remote computers.

**Warning**: Even though access requires a token,  running commander comes at your own risk because it opens up a potent entrance for possible attackers to use.

## API Usage
To execute an arbitrary command, issue a `POST` request using following parameters.

| Key     | Value                            |
|---------|----------------------------------|
| Method  | `POST`                           |
| URL     | `http://ip/exec`                 |
| Header1 | `Token: your_token`              |
| Header2 | `Content-Type: application/json` |
| Body    | `{ command: "ls"}` (*as JSON*)   |

### Usage from Smarthome via Homescript
**Note**: This example uses Smarthome-server `v0.0.47` (which uses Homescript `v0.15.1`).

To test this code, execute `your_id` using the `command` argument with your command as its value.
```python
# Calling `your_id` from outside
exec(
    'your_id',
    pair('command', "ls")
)

# This is the beginning of `your_id`
http(
    'http://computer.box/exec',
    'POST',
    concat('{', '"command":"', getArg('command'), '"}'),
    pair('Content-Type', 'application/json'),
    pair('Token', 'test'),
)
```

## Installation
### Manual installation
#### Cloning
```bash
git clone git@github.com:smarthome-go/commander
cd commander
```
#### Installation
```bash
make install
```

#### Installing a Systemd Service (Optional)
```bash
sudo cp commander@.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl start commander@$USER
```

##### Additional configuration
**Note**: Modifying the `.service` file is not recommended (*except for environment variables*)

When using Systemd, the service is configured using environment variables.
The most important configuration parameters are explained below.

| Parameter         | Explanation                                                       |
|-------------------|-------------------------------------------------------------------|
| `ROCKET_PORT`     | The port on which commander listens on                            |
| `COMMANDER_SHELL` | The shell which commander uses to interpret the received commands |
| `COMMANDER_TOKEN` | Unencrypted user authentication token                             |


Representation in the `.service` file (which is located at `/etc/systemd/system/commander@.service`)
```bash
# Environment variables for the service
Environment=ROCKET_PORT=7070
Environment=COMMANDER_SHELL=/bin/bash
Environment=COMMANDER_TOKEN=test
```

In some cases, you might want to start applications with a GUI or which depend on audio.
On those occasions, the command might fail due to missing environment variables.
You can feel free to include the missing variables under the `# Put user environment variables here` marker.
When you don't want to take on the hassle of figuring our which missing environment variable is causing the issue, you can inclue *all* your current variables in the file.
The command for listing your environment variables on Unix is `env`.
But don't forget to run `sudo systemctl daemon-reload` after writing modifications to the service file.

**Note**: It is not recommended to include *all* of your environment variables in the service file.



### Arch Linux
If using Arch Linux, the most convenient way of installing *commander* is via the AUR (*Arch Linux User Repository*).
For this, an AUR helper like `paru` or `yay` is used.

**Note**: This method automatically installs a Systemd service, however it is not enabled by default.

```bash
paru -S commander
```

