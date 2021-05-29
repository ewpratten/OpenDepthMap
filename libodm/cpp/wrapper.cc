#include <iostream>
#include "../lib/Leap.h"

using namespace Leap;

//---- Start Public Functions ----//
extern "C"
{
    void beginEventLoop();
    bool isControllerCreated();
    void endEventLoop();
    bool imageExists();
    int getImageHeight();
    int getImageWidth();
    int getImageBPP();
    const unsigned char *getImageLeft();
    const unsigned char *getImageRight();
}
//---- End Public Functions ----//

//---- Start Globals ----//
Controller *controller = nullptr;
ImageList images;
//---- End Globals ----//

#include "listener.cc"

//---- Start Public Function Impls ----//

void beginEventLoop()
{
    if (controller == nullptr)
    {
        // Create a controller
        controller = new Controller();

        // Set device policy
        controller->setPolicyFlags(Controller::POLICY_IMAGES);

        // Set up event handling
        LeapEventListener listener;
        controller->addListener(listener);
    }
}

void endEventLoop()
{
    if (controller != nullptr)
    {
        delete controller;
    }
}

bool isControllerCreated() { return controller != nullptr; }
bool imageExists() { return images.count() == 2; }
int getImageHeight() { return images[0].height(); }
int getImageWidth() { return images[0].width(); }
int getImageBPP() { return images[0].bytesPerPixel(); }
const unsigned char *getImageLeft() { return images[0].data(); }
const unsigned char *getImageRight() { return images[1].data(); }

//---- End Public Function Impls ----//
