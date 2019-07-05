## Local Monitor

Check local service status.

#### config.toml

```
[<name>]
name=<name>
exe=<exe>
recovery = {cmd=<cmd>, cwd=<cwd>}

[<name>]
...
```

Check if there is a process whose name contains `<name>` and exe contains `<exe>`.
If a service is not running, restart using `recovery`.

Example:
```toml
[Finder]
name="Finder"

[Alacritty]
name="alacritty"
recovery = {cmd="alacritty", cwd="/"}

[Docker]
exe='dockerd'
recovery = {cmd="sudo service docker start"}

[ssh-agent]
exe='/usr/bin/ssh-agent -l'

[Chrome]
name="Google Chrome"
exe="/Applications/Google Chrome.app"
```
