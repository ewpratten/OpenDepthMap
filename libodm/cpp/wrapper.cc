#include <iostream>
#include "../lib/Leap.h"

using namespace Leap;

class LeapEventListener : public Listener
{
public:
    virtual void onConnect(const Controller &);
    virtual void onDisconnect(const Controller &);
    virtual void onFrame(const Controller &);
};

void LeapEventListener::onConnect(const Controller &controller)
{
    std::cout << "Connected" << std::endl;
    // Enable gestures, set Config values:
    controller.enableGesture(Gesture::TYPE_SWIPE);
    controller.config().setFloat("Gesture.Swipe.MinLength", 200.0);
    controller.config().save();
}

//Not dispatched when running in a debugger
void LeapEventListener::onDisconnect(const Controller &controller)
{
    std::cout << "Disconnected" << std::endl;
}

void LeapEventListener::onFrame(const Controller &controller)
{
    std::cout << "New frame available" << std::endl;
    Frame frame = controller.frame();
    
    // Get the camera images
    ImageList images = frame.images();

    // We require two images
    if (images.count() != 2){
        std::cout << "Not enough images received" << std::endl;
        return;
    }
    
    // Build a protobuf repr of the image data
    FrameStreamMessage message;
    message.set_frame_height(images[0].height());
    message.set_frame_width(images[0].width());
    message.set_pixel_bytes(images[0].bytesPerPixel());
    message.set_left_image((char*) images[0].data());
    message.set_right_image((char*) images[1].data());

    
}

int main()
{

    // Access to the LeapMotion device along with callback setup
    Controller controller;
    LeapEventListener listener;
    controller.addListener(listener);
    controller.setPolicy(Leap::Controller::POLICY_IMAGES);

    while(true){

    }

    return 0;
}