# Python and Rust with Linux Command Line Tools

## Module 1: Introduction

* create virtual environment 

```bash
python3 --help
python3 -m venv .venv
find .venv
which python3
/usr/local/bin/python3

source .venv/bin/activate.fish
which python
/Users/chovey/rustschool/coursera_cli
which python3
/Users/chovey/rustschool/coursera_cli

which pip
/Users/chovey/rustschool/coursera_cli/.venv/bin/pip

pip install -r requirements.txt
```

### Block devices

In Linux, a block device is a type of device file that provides buffered access to hardware devices. These devices allow for reading and writing data in fixed-size blocks, typically 512 bytes or larger. Block devices are used for storage devices like hard drives, SSDs, and optical drives. Examples of block devices in Linux include:

* Hard Disk Drives (HDDs): Represented by device files such as /dev/sda, /dev/sdb, etc.
* Solid State Drives (SSDs): Also represented by device files like /dev/sda, /dev/sdb, etc.
* USB Storage Devices: Represented by device files such as /dev/sdb, /dev/sdc, etc.
* Optical Drives (CD/DVD/Blu-ray): Represented by device files like /dev/sr0, /dev/sr1, etc.
* Loopback Devices: Virtual block devices that map a file to a block device, represented by /dev/loop0, /dev/loop1, etc.
* RAID Arrays: Represented by device files such as /dev/md0, /dev/md1, etc.
* Logical Volumes: Managed by the Logical Volume Manager (LVM), represented by device files like /dev/mapper/vgname-lvname.

You can list block devices on your system using the lsblk command, which provides a tree-like view of all block devices and their partitions:

```bash
lsblk
```

Alternatively, you can use the blkid command to list block devices along with their UUIDs and filesystem types:

```bash
sudo blkid
```

These commands help you identify and manage block devices on your Linux system.

### Lab

`lsblk` lists block devices

```bash
lsblk
# on macOS, equivalent is
diskutil
Disk Utility Tool
Utility to manage local disks and volumes
Most commands require an administrator or root user

WARNING: Most destructive operations are not prompted

Usage:  diskutil [quiet] <verb> <options>, where <verb> is as follows:
// etc...

diskutil list
/dev/disk0 (internal, physical):
   #:                       TYPE NAME                    SIZE       IDENTIFIER
   0:      GUID_partition_scheme                        *1.0 TB     disk0
   1:             Apple_APFS_ISC Container disk1         524.3 MB   disk0s1
   2:                 Apple_APFS Container disk3         994.7 GB   disk0s2
   3:        Apple_APFS_Recovery Container disk2         5.4 GB     disk0s3

/dev/disk3 (synthesized):
   #:                       TYPE NAME                    SIZE       IDENTIFIER
   0:      APFS Container Scheme -                      +994.7 GB   disk3
                                 Physical Store disk0s2
   1:                APFS Volume Macintosh HD - Data     225.9 GB   disk3s1
   2:                APFS Volume Macintosh HD            10.3 GB    disk3s3
   3:              APFS Snapshot com.apple.os.update-... 10.3 GB    disk3s3s1
   4:                APFS Volume Preboot                 6.2 GB     disk3s4
   5:                APFS Volume Recovery                940.1 MB   disk3s5
   6:                APFS Volume VM                      20.5 KB    disk3s6

df
Filesystem     512-blocks      Used  Available Capacity iused      ifree %iused  Mounted on
/dev/disk3s3s1 1942700360  20038816 1467000160     2%  404167 4292891722    0%   /
devfs                 396       396          0   100%     686          0  100%   /dev
/dev/disk3s6   1942700360        40 1467000160     1%       0 7335000800    0%   /System/Volumes/VM
/dev/disk3s4   1942700360  12014984 1467000160     1%    1138 7335000800    0%   /System/Volumes/Preboot
/dev/disk3s2   1942700360    140824 1467000160     1%      52 7335000800    0%   /System/Volumes/Update
/dev/disk1s2      1024000     12328     983888     2%       1    4919440    0%   /System/Volumes/xarts
/dev/disk1s1      1024000     12632     983888     2%      37    4919440    0%   /System/Volumes/iSCPreboot
/dev/disk1s3      1024000      5432     983888     1%      49    4919440    0%   /System/Volumes/Hardware
/dev/disk3s1   1942700360 441265640 1467000160    24% 1545463 7335000800    0%   /System/Volumes/Data
map auto_home           0         0          0   100%       0          0     -   /System/Volumes/Data/home
map -fstab              0         0          0   100%       0          0     -   /System/Volumes/Data/Network/Servers
```

> Goal: build a command line python tool that can be distributed without library dependencies and without needing to package the python.

On macOS, the lsblk command is not available by default. However, you can use alternative commands to list and get information about block devices. One commonly used command is diskutil, which provides detailed information about disks and volumes. Here are some useful diskutil commands:

```bash
# This command provides a tree-like view of all disks and their partitions, similar to lsblk.
diskutil list

# Get detailed information about a specific disk:
diskutil info /dev/disk0
```

The [Click](https://paiml.com/docs/home/books/python-command-line-tools/chapter01-getting-started-click/) framework, as an alternative for `argparse`.

# Rust

* References
  * https://github.com/alfredodeza/rust-cli-example/blob/main/README.md
  * https://github.com/alfredodeza/rust-cli-example/tree/main/examples/simple
  * clap: https://github.com/clap-rs/clap
  * clap alternatives: quicli, structopt

```bash
cargo fmt # format the rust source code
# The rust analyzer calls out 'clippy' and 'check'
cargo clippy # implementation linter
cargo check # a precompiler, alternative to cargo build
```

* Lab: Building a basic Rust CLI: https://www.coursera.org/learn/python-rust-linux/supplement/QSnbR/external-lab-build-a-basic-rust-cli 

# Python

```bash
import some_module

# see path where the module originates
some_module # or
print(some_module)
```

Example:

```bash
cd ~/autotwin/mesh
source .venv/bin/activate.fish

pip list
Package         Version     Editable project location
--------------- ----------- ---------------------------
atmesh          0.0.7       /Users/chovey/autotwin/mesh
...
numpy           1.26.4

python

import atmesh
print(atmesh)
<module 'atmesh' from '/Users/chovey/autotwin/mesh/src/atmesh/__init__.py'>

import numpy
print(numpy)
<module 'numpy' from '/Users/chovey/autotwin/mesh/.venv/lib/python3.11/site-packages/numpy/__init__.py'>

quit()

deactivate

cd ~/autotwin/automesh
source .venv/bin/activate.fish

pip list
Package      Version Editable project location
------------ ------- -------------------------------
automesh     0.1.3   /Users/chovey/autotwin/automesh
...
numpy        2.0.0

python

import automesh
print(automesh)
<module 'automesh' from '/Users/chovey/autotwin/automesh/.venv/lib/python3.11/site-packages/automesh/__init__.py'>

print(numpy)
module 'numpy' from '/Users/chovey/autotwin/automesh/.venv/lib/python3.11/site-packages/numpy/__init__.py'>

quit()
```
