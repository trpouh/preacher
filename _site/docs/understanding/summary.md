## The Preacher

Preacher is a lightweight automation tool written in rust. If focuses on simplicity and speed. 

Its main goals are to 

* provide a lightweight alternative to `ansible-pull` that can run on an embedded device  
* teach me rust (so use at your own risk)

Currently, the following platforms are supported:

* x86_64-unknown-linux-musl
* x86_64-apple-darwin

## Installation

To download the binary for your platform see [releases](https://github.com/trpouh/preacher/releases)  


**Windows**  

Not in the forseeable future. (But who uses windows on embedded devices)


**MacOS**

Kickstart with the installation script:

```bash
$ curl -s -L https://raw.githubusercontent.com/trpouh/preacher/wip/kickstarter/install-mac.sh | bash
```

this will install the `preacher` executable in your current directory and print some useful information into your terminal.

**Linux**

If `zsh` is installed see [MacOS](#macos). Otherwise see the [releases](https://github.com/trpouh/preacher/releases).

## Your first sermon

The preacher requires a yaml file containing all psalms that shall be read. It can be as simple as that:

```yaml
# sermon.yaml
psalms:
- type: Hello
```

## Start the preaching

Upon installing and creating a sermon (see [your first sermon](#your-first-sermon)) the preacher can simply be invoked in your prefered terminal:


```bash
$ ./preacher -h
Usage: preacher [OPTIONS]

Options:
  -r, --repo <REPO>                    
  -b, --branch <BRANCH>                
      --source-folder <SOURCE_FOLDER>  [default: ./]
  -s, --sermon <SERMON>                [default: sermon.yaml]
  -t, --target-folder <TARGET_FOLDER>  [default: ./]
      --worship-dir <worship_dir>         [default: .preacher/tmp]
  -h, --help                           Print help information
  -V, --version                        Print version information
```

prints all the possible parameters but just invoking the preacher without providing any arguments should be enough:

```
$ ./preacher
```

which results in the following output (shortened for better readability):

```
hey there stranger! congratulations to your first successful worship.

psalm with id n/a was successful: ok

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

psalm with id hello_psalm was successful: ok

Cleanup finished. The worship is over.
```

You can initiate a worship with the hello psalm directly from this repository! Just provide it when starting preacher:


```bash
./preacher --repo https://github.com/trpouh/preacher --branch wip --sermon examples/hello-sermon.yaml
```

## Psalms
> a sacred song or poem used in worship

Psalms are the heart of every worship. They are defined in a list in the `sermon.yaml` file. They have a simple structure:

```yaml
- type: String
  id: Optional<String>
  name: Optional<String>
```

* `type` defines its purpose.
* `id` can be used for logic
* `name` can be used for documentation/logging purposes


### Hello

hello world to verify a successful installation of _the preacher_

```yaml
- type: Hello
  # optional
  name: String
```

### Timezone

change the timezone by leveraging `datetimectl` (needs `sudo`)

```yaml
- type: Timezone
  # timezone as defined in the tz database
  # see: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
  # example: Austria/Vienna
  tz: String
```

### Yaml

Manipulate the yaml content of a file (see [File](#file)). it will override the object provided via the path. If you set the path `$` it will override the whole file.

```yaml
- type: Yaml
  source: FileSource # << Deacon (see FileSource)
  destination: FileDestination # << Deacon (see FileDestination)
  yaml_path: String # optional; default: $ 
  
```

example (add a child to the root object):

```yaml
- type: Yaml
  source: 
    content: |
      name: John Doe
      age: 25
  target: "my-file.yaml"
  yaml_path: "$.participant"
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

### File

A simple psalm to copy a file to your target machine

```yaml
# Example "File Psalm"
- type: File
  source: FileSource # << Deacon (see FileSource)
  destination: FileDestination # << Deacon (see FileDestination) 
```

## Deacons
> A deacon is a member of the diaconate, an office in Christian churches that is generally associated with service of some kind [...]

Deacons are powerful datatypes to help standardize common processes. They are documented as datatypes in psalms. 

#### FileSource

This deacon provides the ability to easily access files from different sources, such as:

* git
* http (todo!)
* local
* inline

the type that's being used is decided by the structure of the given data:

```yaml
# local simple
field: String

# local complex
field: 
  path: String
  # default: false (path is relative to worship, otherwise
  # is relative to target_folder )
  in_worship: Optional<boolean>; 

# http (todo!)
field:
  url: String

# git
field: 
  repo: String
  file: String
  branch: Optional<String>

inline:
  content: String
```

Let's look at an example to see why this is so powerful. A simple file copy (using the [`File` Psalm](#file)). 

```yaml
# sermon.yaml
psalms:
- type: File
  source: "file.txt"
  destination: "file-target.txt"
```

This is a simple as it can get. But what if you want to store the file in a git repository? No problem. Just change the structure of `source`:

```yaml
# sermon.yaml
psalms:
- type: File
  source: 
    repo: https://github.com/<your-repo>.git
    branch: main
    file: file.txt
  destination: "file-target.txt"
```

If you don't want to bother creating and downloading a file, just use `inline`.

```yaml
# sermon.yaml
psalms:
- type: File
  source: 
    content: |
      hey!
      this is my file.
  destination: "file-target.txt"
```

**Templating**

Every `FileSource` is templatable. It doesn't matter whether its retrieved via `git`, `http` or `inline`. Just configure it by adding a templating flavor (currently supported is `J2`):

```yaml
vars: 
  name: trpouh
# sermon.yaml
psalms:
- type: File
  source: 
    content: |
      hey!
      this is {{ name }}.
    template:
      flavor: J2
  destination: "file-target.txt"
```

Same applies to a git retrieved file:
```yaml
vars: 
  name: trpouh
# sermon.yaml
psalms:
- type: File
  source: 
    repo: https://github.com/<your-repo>.git
    branch: main
    file: file.txt
    template:
      flavor: J2
  destination: "file-target.txt"
```

Compared to ansible-pull, where you would have to use different modules for every use case, this seems like a much more elegant solution.

```yaml
# Simple file copy
- name: copy
  ansible.builtin.copy:
    src: file.txt
    dest: /file-target.txt

# Templating a local file
- name: template
  template: 
    src: templates/file.j2
    dest: /file-target.txt

# Downloading from git 
- name: Git checkout
  ansible.builtin.git:
    repo: https://github.com/<your-repo>.git
    dest: /file-target.txt

# Template file from git
# According to https://stackoverflow.com/questions/33163204/ansible-remote-templates
# Found no example in the official ansible doc
- name       : clone repo on Ansible host
  hosts      : localhost
  connection : local
  git  :
    repo : {{ git_repo_src }}
    dest : {{ git_repo_local_dest }}

- name     : template remote hosts
  template :
    src   : {{ template_local_src }}
    dest  : {{ templated_file_dest }}
    owner : {{ templated_file_owner }}
    group : {{ templated_file_group }}
    mode  : {{ templated_file_mode }}

```

#### FileDestination

There are multiple ways to define a file destination. The simple on is by providing just a filename: 

```yaml
# will fail if the target file does not exist
field: String
```

If you want to provide more settings, there is also the `complex` option:

```yaml
field:
  # required
  name: String
  # optional; default: false
  create: boolean 
  # optional; default: false
  create_parents: boolean
```