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
Hey there stranger! Congratulations to your first successful worship.

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

Psalms are the heart of every worship. They are defined in a list in the `sermon.yaml` file.

They have simple structure:

```yaml
- type: String
  id: Optional<String>
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

## Deacons
> A deacon is a member of the diaconate, an office in Christian churches that is generally associated with service of some kind [...]

Deacons are datatypes to help standardize common processes. They are documented as datatypes in psalms. 

```yaml
# Example "Copy Psalm"
- type: Copy
  source: FileSource # << Deacon (see FileSource)
  destination: FileDestination # << Deacon (see FileDestination) 
```

#### FileSource

Files can have multiple sources:

* git
* http
* local

provide them as follows:

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

# git (todo!)
field: 
  repo: String
  path: String
  branch: Optional<String>
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