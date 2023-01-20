---
title: Deacons
layout: default
parent: Resources
nav_order: 4
---
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