---
title: Quickstart Guide
layout: default
parent: Essentials
nav_order: 2
---

# Quickstart Guide

In this guide you will get preacher up and running and have your first worship.

## Prerequisites

Preacher can be run on most unix based systems. If you're not sure whether your target OS is supported or not please take a look at the [releases](https://github.com/trpouh/preacher/releases). Depending on what kind of psalms you want to run, additional tools may be required as well.

## Installation

Download the the [latest release](https://github.com/trpouh/preacher/releases) for your OS. Alternatively you can use the installation kickstarter script.

### Mac OS
 
If you are using Mac OS you can download the kickstart script to install preacher. 
```
$ curl -fsSL -o install_preacher.sh https://raw.githubusercontent.com/trpouh/preacher/wip/kickstarter/install-mac.sh
$ chmod 700 install_preacher.sh
$ ./install_preacher.sh
```

## Create a sermon

The default language for sermons is YAML (Support for other markup languages like JSON or XML is planned). The most important parts of a sermon are: `variables` and `psalms`. 

```yaml
# sermon.yaml
vars: {}
psalms: []
```

Psalms contain information about the to be automated tasks. The simplest one is the `Hello` task.

```yaml
# sermon.yaml
vars: {}
psalms:
- type: Hello
```

running this sermon via 

```bash
$ ./preacher -s sermon.yaml
```

produces the following output (ommited for readability):

```
hey there stranger! congratulations to your first successful worship.
```

The `Hello` psalms provides a mechanism to customize the output message via the `name` attribute. Let's set the name and run it again.

```yaml
# sermon.yaml
# vars ommited because not needed and nullable
psalms:
- type: Hello
  name: trpouh
```


```
hey there trpouh! congratulations to your first successful worship.
```

## Level it up with Deacons

Seems pretty straight forward but the real power of `Psalms` can be seen when leveraging `Deacons`. We believe that many different tasks come down to a small set of principles/processes. So to allow for high flexbility in your automation with a simple API we came up with `Deacons`. 

Let's look at an example: Copying content from A to B is a pretty standard task. In Preacher this can be done with the `File` psalm:

```yaml
# sermon.yaml
psalms:
- type: File
  source: "myfile.txt"
  destination: "dir/myfile.txt"
```

So far so good. But what if you want to copy the file from a git repo to the given destination? The logic (Copy content from A to B) stays the same. The only thing that changes is the file `source`, and thats what we're going to do:

```yaml
# sermon.yaml
psalms:
- type: File
  source: 
    repo: https://server.com/repo.git
    branch: main
    file: myfile.txt
  destination: "dir/myfile.txt"
```

Neat, right? If you now wanted to template this file in some automation-tools this can be [a bit tricky](https://stackoverflow.com/questions/33163204/ansible-remote-templates). In Preacher its as simple as adding a `template` parameter like so: 

```yaml
# sermon.yaml
psalms:
- type: File
  source: 
    repo: https://server.com/repo.git
    branch: main
    file: myfile.txt.j2
  template:
    flavor: J2
  destination: "dir/myfile.txt"
```

## Real-life example

As we learned in the previous chapter, we can easily swap parts of the psalm without having to change the logic (i.e. the `type` of a psalm). To showcase this, we're gonna create a psalm that contains a templatable literal, that uses a variable defined in the `vars` object of the psalm.

First define the variable:

> The configuration of variables is designed to work similarly to [Microprofile-Config](https://microprofile.io/microprofile-config/). 


```yaml
# sermon.yaml
vars: 
  name: 
    env: NAME
    default: trpouh
psalms: []
```

Then the psalm (this time we are using the `content`-method to define the source):

```yaml
# sermon.yaml
vars: 
  name: 
    env: NAME
    default: trpouh
psalms:
- type: File
  source: 
    content: |
      { { name } } was here!
  template:
    flavor: J2
  destination: "dir/myfile.txt"
```

which results in the following result:

```bash
$ cat dir/myfile.txt
trpouh was here!
```

or - when setting the environment variable - like this:

```bash
$ NAME=nottrpouh ./preacher
[...]
$ cat dir/myfile.txt
nottrpouh was here!
```

Thanks for reading the Quickstart Guide. If you want to continue your learning journey, please refer to the [Next Steps](next-steps.html).


## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}