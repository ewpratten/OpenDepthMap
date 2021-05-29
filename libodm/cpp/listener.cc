class LeapEventListener : public Listener
{
public:
    virtual void onConnect(const Controller &);
    virtual void onDisconnect(const Controller &);
    virtual void onFrame(const Controller &);
};

void LeapEventListener::onConnect(const Controller &controller)
{
    // std::cout << "LeapMotion Controller: Connected" << std::endl;
}

//Not dispatched when running in a debugger
void LeapEventListener::onDisconnect(const Controller &controller)
{
    // std::cout << "LeapMotion Controller: Disconnected" << std::endl;
}

void LeapEventListener::onFrame(const Controller &controller)
{
    // std::cout << "New frame available" << std::endl;
    // Frame frame = controller.frame();

    // // Get the camera images
    // images = frame.images();

}