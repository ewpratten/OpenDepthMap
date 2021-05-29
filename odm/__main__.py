import argparse
import sys
import logging
import os

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.DEBUG)

# Load in libodm
sys.path.append(os.getcwd() + "/target/debug")
import libpylibodm as pylibodm

def main() -> int:
    # Handle program arguments
    ap = argparse.ArgumentParser(prog='odm.py', description='Stream 3D data from a LeapMotion camera')
    args = ap.parse_args()

    # Connect to the leapmotion device
    print("Connecting to LeapMotion")
    print(dir(pylibodm))
    pylibodm.connect(4)

    return 0

if __name__ == "__main__":
    sys.exit(main())