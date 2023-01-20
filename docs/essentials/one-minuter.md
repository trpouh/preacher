---
title: One Minuter
layout: default
parent: Essentials
nav_order: 1
---

**Reading time**: 1 Minute (max!) 

Preacher is an Open-Source automation tool used to configure, orchestrate and administrate devices. 

Its a binary application with a very small footprint (Build for Apple: ~700kb) that uses so called **Sermons** to summarize subtasks (i.e. copying a file, executing a script) that can be configured and reused throughout your environment. The default langauge for Sermons is YAML however support for langauges such as JSON or XML is planned. 

To copy a file to a target device a sermon could look as simple as this:

```yaml
# sermon.yaml
psalms:
- id: copy webpage
  type: File
  source: "index.html"
  destination: "index.html"
```

The preacher initiates a `Worship` in which a `Sermon` is _preached_. The worship contains all necessary information about the `Sermon` and how the preaching should be done (target folder, source folder, etc.). The fastest way to start preaching is by using a local sermon:

```bash
$ preacher -s ./sermon.yaml
```

However, we know that IaC combined with version control can be very powerful so Preacher supports fetching sermons from git repositories as well:

```bash
$ preacher -r http://github.com/repo.git \
           -b main \
           -s sermon.yaml
```

This was a quick introdution about the features of the Preacher. To starting experimenting with preacher right away (the installation is a one-minuter as well) check the [Quickstart Guide](/docs/essentials/quickstart.html). If you just want to look around, check the [Next steps](/docs/essentials/next-steps.html) to find the right docs for all your questions.