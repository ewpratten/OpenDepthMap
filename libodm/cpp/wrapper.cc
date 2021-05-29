#include <iostream>
#include "../lib/Leap.h"

using namespace Leap;

//---- Start Public Functions ----//
extern "C" void beginEventLoop();
extern "C" bool isControllerCreated();
extern "C" void endEventLoop();
extern "C" bool imageExists();
extern "C" int getImageHeight();
extern "C" int getImageWidth();
extern "C" int getImageBPP();
extern "C" const unsigned char *getImageLeft();
extern "C" const unsigned char *getImageRight();
//---- End Public Functions ----//

//---- Start Globals ----//
Controller *controller = nullptr;
// ImageList images;
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
        // LeapEventListener listener;
        // controller->addListener(listener);
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
bool imageExists() { return controller->frame().images().count() == 2; }
int getImageHeight() { return controller->frame().images()[0].height(); }
int getImageWidth() { return controller->frame().images()[0].width(); }
int getImageBPP() { return controller->frame().images()[0].bytesPerPixel(); }
const unsigned char *getImageLeft() { return controller->frame().images()[0].data(); }
const unsigned char *getImageRight() { return controller->frame().images()[1].data(); }

//---- End Public Function Impls ----//
