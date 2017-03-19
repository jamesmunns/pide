#!/usr/bin/python
import sys
import json
import os
import re
import uuid
import time

from subprocess import check_output as cmd
from subprocess import Popen

def init(args):
    dockerfile = args[0]
    workingpath = os.path.dirname(dockerfile) # todo, don't hardcode this

    # NOTE: name isn't necessary yet. Don't hardcode it in the creation.
    name = "pide:{}".format(str(uuid.uuid4()))

    print("Building dockerfile (if necessary)...")
    output = cmd(["docker", "build", "-f", dockerfile, workingpath])
    image_id = re.search('Successfully built ([a-f0-9]*)', output.decode()).group(1)

    with open('.pide', 'w') as ofile:
        data = {
            "name": name,
            "image_id": image_id,
            "original_dockerfile": dockerfile,
            "original_workingpath": workingpath,
        }

        json.dump(data, ofile, sort_keys=True, indent=4, separators=(',', ': '))

    print("Initialized pidefile")

def resume(args):
    with open('.pide', 'r') as ifile:
        pidef = json.load(ifile)

    existing_images = cmd(["docker", "images"]).decode()

    # Whooo... are yoooouuuu?
    if pidef["name"].split(':')[-1] not in existing_images:
        # probably never been comitted before. Use the image id
        print("first!")
        name = pidef["image_id"]
    else:
        # probably has been comitted before. Use the comitted name
        print("resuming!")
        name = pidef["name"]

    temp_name = str(uuid.uuid4())

    # Todo: allowable run args?
    x = Popen(["docker", "run", "--name", temp_name, "-it", name, "/bin/bash"])

    # This is maybe a bad idea? seems to work...
    while(True):
        foo = x.communicate()

        if foo == (None, None):
            break

        print(foo)
        time.sleep(0.016) #60Hz, yo

    # Okay, we're done with that, now lets grab the latest image

    print("Committing container history...")
    cmd(["docker", "commit", temp_name, pidef["name"]])

def main():

    if len(sys.argv) > 1 and sys.argv[1] in DISPATCH:
        DISPATCH[sys.argv[1]](sys.argv[2:])
    else:
        print("Bad arg!")

DISPATCH = {
    "init": init,
    "run": resume,
    "resume": resume,
}

if __name__ == '__main__':
    main()