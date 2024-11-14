# Exercism

* Install the Exercism CLI

```sh
brew update
brew install exercism
exersism version # should be at least 3.x (it is 3.5.2)
exercism configure --token=9e9b6ea3-3996-49e7-816d-98d5a40195eb
```

output

```sh
You have configured the Exercism command-line client:

Config dir:                       /Users/chovey/.config/exercism
Token:         (-t, --token)      9e9b6ea3-3996-49e7-816d-98d5a40195eb
Workspace:     (-w, --workspace)  /Users/chovey/Exercism
API Base URL:  (-a, --api)        https://api.exercism.io/v1
```

* Install the Rust tooling
* Download this exercise
```sh
 (main) chovey@s1088757/Users/chovey/rustschool/exercism> exercism download --track=rust --exercise=hello-world
```
```sh
 (main) chovey@s1088757/Users/chovey/rustschool/exercism> exercism download --track=rust --exercise=hello-world

Downloaded to
/Users/chovey/Exercism/rust/hello-world
```

* Solve exercise on your local machine, then sumbit via

```sh
  exercism submit .
```

