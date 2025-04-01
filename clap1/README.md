# Readme

Set an alias in the Fish configuration file,

```sh
vim ~/.config/fish/config.fish
```

Add the following function to the file,

```sh
function cl
    ./target/debug/clap1 $argv
end
```

Apply the changes,

```sh
source ~/.config/fish/config.fish
```

Test the command,

```sh
cl add 5 3
```
