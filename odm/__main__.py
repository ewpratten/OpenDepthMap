import argparse
import sys
import logging
import os
import numpy as np
import cv2

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.DEBUG)

# Load in libodm
sys.path.append(os.getcwd() + "/target/debug")
import libpylibodm as pylibodm

def handle_image_data(stereo, left_image, right_image):

    # Convert images to something opencv can use
    left_image_cv = np.frombuffer(bytes(left_image.buffer), np.uint8).reshape(
        left_image.height, left_image.width)
    right_image_cv = np.frombuffer(bytes(right_image.buffer), np.uint8).reshape(
        left_image.height, left_image.width)

    # Compute a disparity map
    disparity = stereo.compute(left_image_cv, right_image_cv)
    disparity = cv2.convertScaleAbs(disparity, alpha=1.5)

    cv2.imshow("Left", left_image_cv)
    cv2.imshow("Right", right_image_cv)
    cv2.imshow("Disparity", disparity)


def main() -> int:
    # Handle program arguments
    ap = argparse.ArgumentParser(
        prog='odm.py', description='Stream 3D data from a LeapMotion camera')
    args = ap.parse_args()

    # Connect to the leapmotion device
    print("Connecting to LeapMotion")
    pylibodm.connect(4)

    # Set up depth mapping
    stereo = cv2.StereoBM_create(numDisparities=32, blockSize=11)

    # Handle data
    while True:
        frame = pylibodm.get_frame()
        handle_image_data(stereo, frame.left_camera, frame.right_camera)

        if cv2.waitKey(25) & 0xFF == ord('q'):
            break

    return 0


if __name__ == "__main__":
    sys.exit(main())