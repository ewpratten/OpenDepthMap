import argparse
import sys
import os

# Load in libodm
sys.path.append(os.getcwd() + "/target/debug/")
import pylibodm

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