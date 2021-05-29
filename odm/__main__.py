import argparse
import sys
import logging
import os
import numpy as np
import cv2

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.DEBUG)

# Config
BLUR_KERNEL_SIZE = 3
THRESH_MIN_PIXEL = 32
BLOB_MIN_AREA = 120
HEAT_OVERLAY_ALPHA = 0.5

# Load in libodm
sys.path.append(os.getcwd() + "/target/debug")
import libpylibodm as pylibodm

def handle_image_data(stereo, left_image, right_image):

     # Convert images to something opencv can use
    left_image_cv = np.frombuffer(bytes(left_image.buffer), np.uint8).reshape(
        left_image.height, left_image.width)
    right_image_cv = np.frombuffer(bytes(right_image.buffer), np.uint8).reshape(
        left_image.height, left_image.width)

    # Smooth out the raw images
    kernel = np.ones((BLUR_KERNEL_SIZE,BLUR_KERNEL_SIZE),np.float32)/(BLUR_KERNEL_SIZE * BLUR_KERNEL_SIZE)
    left_image_cv_smooth = cv2.filter2D(left_image_cv,-1,kernel)
    right_image_cv_smooth = cv2.filter2D(right_image_cv,-1,kernel)

    # Compute a disparity map
    disparity = stereo.compute(left_image_cv_smooth, right_image_cv_smooth)
    disparity = cv2.convertScaleAbs(disparity, alpha=1.5)

    # Contour filtering
    ret, threshold = cv2.threshold(disparity, THRESH_MIN_PIXEL, 255, cv2.THRESH_BINARY)
    contours, hierarchy = cv2.findContours(threshold,
                                           cv2.RETR_TREE, cv2.CHAIN_APPROX_NONE)
    filtered_contours = [cnt for cnt in contours if cv2.contourArea(cnt) > BLOB_MIN_AREA]      
    mask = np.zeros(disparity.shape, np.uint8)                                 
    cv2.drawContours(mask, filtered_contours, -1, (255), thickness=cv2.FILLED)

    # Build a heatmap
    heatmap = cv2.applyColorMap(disparity, cv2.COLORMAP_JET)
    masked_heatmap = cv2.bitwise_and(heatmap, heatmap, mask=mask)

    # Create an overlay from the heatmap
    output_base = cv2.cvtColor(left_image_cv,cv2.COLOR_GRAY2RGB)
    output_base_mod = cv2.convertScaleAbs(output_base, alpha=1.5, beta= 20)
    output = cv2.addWeighted(masked_heatmap, HEAT_OVERLAY_ALPHA, output_base_mod, 1.0, 0.0)

    cv2.imshow("Left", left_image_cv)
    cv2.imshow("Right", right_image_cv)
    cv2.imshow("Disparity", disparity)
    cv2.imshow("Heatmap", masked_heatmap)
    cv2.imshow("Output", output)

def main() -> int:
    # Handle program arguments
    ap = argparse.ArgumentParser(
        prog='odm.py', description='Stream 3D data from a LeapMotion camera')
    args = ap.parse_args()

    # Connect to the leapmotion device
    print("Connecting to LeapMotion")
    pylibodm.connect(4)

    # Set up depth mapping
    stereo = cv2.StereoBM_create(numDisparities=32, blockSize=15)

    # Handle data
    while True:
        frame = pylibodm.get_frame()
        handle_image_data(stereo, frame.left_camera, frame.right_camera)

        if cv2.waitKey(25) & 0xFF == ord('q'):
            break

    return 0


if __name__ == "__main__":
    sys.exit(main())