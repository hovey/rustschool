"""This module is the subject exercise of the Coursera CLI course.

To run:
cd ~/rustschool/coursera_cli
source .venv/bin/activate.fish
python simple.py list

"""

import subprocess
import shlex
import json
import platform


def run_command(command):
    """create a function that runs suprocess and returns the output"""
    cmd = shlex.split(command)
    output = subprocess.check_output(cmd)
    return output


def run_lsblk(device):
    """
    Runs lsblk command and produces JSON output:

    lsblk -J -o NAME,SIZE,TYPE,MOUNTPOINT
    {
    "blockdevices": [
        {"name": "vda", "size": "59.6G", "type": "disk", "mountpoint": null,
            "children": [
                {"name": "vda1", "size": "59.6G", "type": "part", "mountpoint": "/etc/hosts"}
            ]
        }
    ]
    }
    """
    # command = f"lsblk -J -o NAME,SIZE,TYPE,MOUNTPOINT"
    # default is to assume a macOS
    command = "diskutil"
    if platform.system() == "Linux":
        # overwrite for Linux
        command = "lsblk -J -o NAME,SIZE,TYPE,MOUNTPOINT"  # overwrite
    output = run_command(command)
    devices = json.loads(output)["blockdevices"]
    for parent in devices:
        if parent["name"] == device:
            return parent
        for child in parent.get("children", []):
            if child["name"] == device:
                return child


def main(device):
    """The main entry point via
    > python simple.py
    """
    print(f"Processing device: {device}")
    print(f"         '{run_lsblk(device)}'")


if __name__ == "__main__":
    import sys

    main(sys.argv[-1])
