This is a WIP project. Some of following features are yet to be implemented.

Current status of this project is MVP. You can now use it as a timer, but not as a tracker/logger or statistics viewer.

`pomodoro` stores data in a `.pomodoro` file of project root folder. Here's an example `.pomodoro` file.

```pomodoro
+ 2020-02-21T14:30:32 PT25M design #35
- 2020-02-21T14:55:32 PT5M
+ 2020-02-21T15:00:32 PT16M28S implement #35
```

# Config file
`pomodoro` stores user configurations in `~/.config/pomodoro/config.toml` file.

```toml
duration = 25
short_break = 5
long_break = 20
repetition = 4
```

- command line options/environment varialbes
- user defaults (~/.config/pomodoro), system defaults (/etc) ??

## Initiate new pomodoro project
```sh
pomodoro init
```

Makes empty `.pomodoro` file

## Run pomodoro timer
```sh
$ pomodoro do design
On task coding

✓ Pomodoro	 00:00:01 [#------------------------]
```

## Show stats
```sh
pomodoro stats
```
