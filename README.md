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

upon installing and creating a sermon (see [your first sermon](#your_first_sermon)) the preacher can simply be invoked in your prefered terminal:


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
## what exactly happend now?

Upon invoking, the preacher looks for the sermon as provided by the user. In the first example we did not provide a sermon via `-s` so the default `sermon.yaml` was used. 

The whole folder in which the sermon resides will be copied into a temporary directory. A sermon can also be downloaded from a git repository, to use e.g. the sermon in this repository invoke the preacher like this:

```
$> ./preacher --repo https://github.com/trpouh/preacher --branch wip --sermon examples/hello-sermon.yaml
```

All psalms will then be _read_ in the order they are defined in. paths defined in psalms (to e.g. delete/create files) will be relative to `target-folder` (default: `.`).


## psalms

Currently the following psalms are implemented:


<!---
The architecture of the preacher is best described in this picture.

<p align="center">
  <img src="https://github.com/trpouh/preacher/blob/docs/docs/arch.svg?raw=true" alt="Preachers architecture"/>
</p>
--->
