Table of Contents

- [introduction](#introduction)
  - [motivation](#motivation)
- [getting started](#getting-started)
  - [Windows](#windows)
  - [MacOS](#macos)
  - [Linux](#linux)
- [your first sermon](#your-first-sermon)
- [start preaching](#start-preaching)
- [psalms](#psalms)
  - [deacons](#deacons)
    - [File](#file)
  - [Hello](#hello)
  - [Timezone](#timezone)
  - [Yaml](#yaml)


## introduction

preacher is a lightweight automation tool written in rust. to start working with preacher one should know and understand the concepts of a `worship`, `sermon` and `psalm`. 

| term    | definition
|--       |--
| worship | all neccessary information needed to deliver a `sermon` (aka preaching)
| sermon  | contains a collection of `psalms` that shell be read
| psalm   | subset of instructions

Currently, the following platforms are supported:

* x86_64-unknown-linux-musl
* x86_64-apple-darwin

### motivation

This project serves two functions:

* (mainly) teach me rust
* provide a lightweight alternative to ansible(-pull) that can easily run on embedded devices

## getting started

to download the binary see [releases](https://github.com/trpouh/preacher/releases) 
### Windows

Not yet!

### MacOS

Kickstart with the installation script:

```shell
curl -s -L https://raw.githubusercontent.com/trpouh/preacher/wip/kickstarter/install-mac.sh | bash
```

### Linux

If `zsh` is installed see [MacOS](#macos).

## your first sermon

the preacher requires a yaml file containing all psalms that shall be read. it can be as simple as that:

```yaml
# sermon.yaml
psalms:
- type: Hello
```

## start preaching

upon installing and creating a sermon (see [your first sermon](#your-first-sermon)) the preacher can simply be invoked in your prefered terminal:


```
$> ./preacher -h
Usage: preacher [OPTIONS]

Options:
  -r, --repo <REPO>                    
  -b, --branch <BRANCH>                
      --source-folder <SOURCE_FOLDER>  [default: ./]
  -s, --sermon <SERMON>                [default: sermon.yaml]
  -t, --target-folder <TARGET_FOLDER>  [default: ./]
      --tmp-dir <TMP_DIR>              [default: .preacher/tmp]
  -h, --help                           Print help information
  -V, --version                        Print version information

$> ./preacher
```

which results in the following output (shortened for better readability):

```
hey there stranger! congratulations to your first successful worship.

psalm with id unknown was successful

Cleanup finished. The worship is over.
```

however, by adding a bit of informationwe can change the output of the `Hello` psalm:

```yaml
# sermon.yaml
psalms:
- type: Hello
  id: hello_psalm
  name: john doe
```
resulting in the following output: 

```
hey there john doe! congratulations to your first successful worship.

psalm with id hello_psalm was successful

Cleanup finished. The worship is over.
```

> Did you know? 

You can initiate a sermon with the hello psalm directly from this repository! Just provide it when starting preacher:


```bash
$> ./preacher --repo https://github.com/trpouh/preacher --branch wip --sermon examples/hello-sermon.yaml
```

<!---
## what exactly happend now?

Upon invoking, the preacher looks for the sermon as provided by the user. In the first example we did not provide a sermon via `-s` so the default `sermon.yaml` was used. 

The whole folder in which the sermon resides will be copied into a temporary directory. A sermon can also be downloaded from a git repository, to use e.g. the sermon in this repository invoke the preacher like this:

All psalms will then be _read_ in the order they are defined in. paths defined in psalms (to e.g. delete/create files) will be relative to `target-folder` (default: `.`).

--->

## psalms

Psalms are the heart of the preacher. They are defined as list items in the `sermon.yaml` file. Every psalm is defined in its own chapter, however psalms shares a couple of properties to allow for (future) logic:

```yaml
- type: <Generic-Psalm>
  # optional, useful when you want to implement
  # logic -> start only when x was successful
  id: String
```

### deacons

Deacons are datatypes to help standardize common processes. A file psalm that manipulates a file for example will always have to have a valid input path (see [File](#file).

#### File

There are two ways define a file. The simple on is by providing just a filename: 

```yaml
field: String
```

If you want to provide more settings, there is also the `complex` option:

```yaml
field:
  # required
  name: String
  # optional; default: false
  create: boolean 
```

### Hello

hello world to verify a successful installation of _the preacher_

```yaml
- type: Hello
  # optional
  name: String
```

### Timezone

change the timezone by leveraging `datetimectl`

```yaml
- type: Timezone
  # timezone as defined in the tz database
  # see: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
  # example: Austria/Vienna
  tz: String
```

### Yaml

manipulate the yaml content of a file (see [File](#file)). it will override the object provided via the path. If you set the path `$` it will override the whole file.

```yaml
- type: Yaml
  file: File
  # can be inline, multiline, whatever
  override: String
  # simplified version of a json path
  # optional; default: $
  path: String
```

example (add a child to the root object):

```yaml
- type: Yaml
  file: "my-file.yaml"
  override: |
    name: John Doe
    age: 25
  path: "$.participant"
```

will result in:

```yaml
# my-file.yaml (before)
foo: "bar"

# my-file.yaml (after)
foo: "bar"
participant:
  name: "John Doe"
  age: "25"
```


<!---
The architecture of the preacher is best described in this picture.

<p align="center">
  <img src="https://github.com/trpouh/preacher/blob/docs/docs/arch.svg?raw=true" alt="Preachers architecture"/>
</p>
--->
