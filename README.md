# Persistent Isolated Development Environments

Use docker to create fully persistent and isolated development environments. Think `virtualenv` from the python world, only for a whole system.

## Use case

I develop a lot of proof of concept and rapidly prototyped embedded systems. This means I end up with a lot of vendor-specific toolchains and components. Often instructions provided by a vendor will only work with very specific environment versions (e.g. "Only works on Ubuntu 14.04 with Python 2.6 and GCC 4.8.4").

This tool allows me to isolate each of those environments within a docker container, while also allowing me to still use the workflow I am used to.

## Installation

If you have rust + cargo installed: `cargo install pide-rs`.

Binaries coming soon.

### Dependencies

* docker (tested with v17.03)

## Usage

First, create a dockerfile somewhere like `~/dockerfiles/debian-jessie-vim`. The contents should look something like this:

```dockerfile
FROM debian:jessie

RUN apt-get update

RUN apt-get install -y \
    vim
```

Next, move to the working directory of your project. Here I will use `~/demo`. Initialize `pide` to your local working directory. This step may take some time, depending on how long it takes to build your dockerfile for the first time.

```bash
~ cd ~/demo
~ pide init ~/dockerfiles/debian-jessie-vim
Building dockerfile (if necessary)...
Initialized pidefile
```

Now you have a persistent working environment! jump into it by calling `pide resume`.

```bash
~ ls -a
.  ..  a.txt  b.txt  c.txt  .pide
~ pide resume
Running `debian-jessie-vim` for the first time...
root@ae6da64ee5e2:/# ls /host
a.txt  b.txt  c.txt
root@ae6da64ee5e2:/# echo Hello World > hello_file
root@ae6da64ee5e2:/# cat hello_file
Hello World
root@ae6da64ee5e2:/# exit
Committing container history...
```

Note that the location you call `pide resume` from is mapped as `/host` in the docker environment. This is dangerous and bad. Be careful with this.

In this example, we created a file called `hello_file`. If we resume again, it will still be there! This is also true for any system changes made, for example installing packages. Lets see what happens when we run it again:

```bash
~ pide resume
Resuming `debian-jessie-vim` where you left off...
root@9691f706c0e5:/# ls -l | grep hello_file
-rw-r--r--   1 root root    12 Mar 19 18:39 hello_file
```

## Multiple Instances

If you re-use the same dockerfile in separate places, they will not interact with each other (though the `init` stage will run much faster, as the original cached docker image is used). Example:

```bash
~ mkdir demo2
~ cd demo2
~ pide init ~/dockerfiles/debian-jessie-vim
Building dockerfile (if necessary)...
Initialized pidefile
~ pide resume
Running `debian-jessie-vim` for the first time...
root@b64b1d8e24da:/# ls
bin  boot  dev  etc  home  host  lib  lib64  media  mnt  opt  proc  root  run  sbin  srv  sys  tmp  usr  var
root@b64b1d8e24da:/# exit
Committing container history...
~
```

Using multiple dockerfiles in one working directory is not currently supported.

## TODOs, Notes, Other

**First of all, this is a really hacky tool meant to support my workflow. It should NOT be used for production, and may misbehave in any number of unsafe or dangerous ways. Use at your own risk.**

### Known issues/problems

* This will create a lot of orphaned docker images. This generally wastes space on your machine. From docker v1.13+, you can run `docker system prune` to clean up a bit. This is a bigger problem on Mac/Windows, where docker runs within a virtual machine with fairly limited total space. In those environments, I suggest using vagrant instead of this tool (you're running a VM anyway)
* This tool maps your current working directory to docker. The root user can then do all sorts of bad things with UID 0, including making things executable, deleting the mapped directory, etc. Assume anything you do inside docker could have impact outside of docker. Use this only for dependency isolation, NOT security isolation
* Basically nothing is configurable yet. Eventually I would like to support more functionality from tools like `docker-compose` to map ports, volumes, networks, etc.

## License

This tool is licensed under the MIT license.