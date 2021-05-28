use leap_sys::{
    LEAP_CONFIG_CHANGE_EVENT, LEAP_CONFIG_RESPONSE_EVENT, LEAP_CONNECTION_EVENT,
    LEAP_CONNECTION_LOST_EVENT, LEAP_CONNECTION_MESSAGE, LEAP_DEVICE_EVENT,
    LEAP_DEVICE_FAILURE_EVENT, LEAP_HEAD_POSE_EVENT, LEAP_IMAGE_EVENT, LEAP_LOG_EVENT,
    LEAP_LOG_EVENTS, LEAP_POINT_MAPPING_CHANGE_EVENT, LEAP_POLICY_EVENT, LEAP_TRACKING_EVENT,
};

#[derive(Debug)]
pub enum EventType {
    Invalid,
    Connection(*const LEAP_CONNECTION_EVENT),
    ConnectionLost(*const LEAP_CONNECTION_LOST_EVENT),
    Device(*const LEAP_DEVICE_EVENT),
    DeviceLost(*const LEAP_DEVICE_EVENT),
    DeviceFailure(*const LEAP_DEVICE_FAILURE_EVENT),
    Tracking(*const LEAP_TRACKING_EVENT),
    ImageComplete,
    ImageRequestError,
    LogEvent(*const LEAP_LOG_EVENT),
    Policy(*const LEAP_POLICY_EVENT),
    ConfigChange(*const LEAP_CONFIG_CHANGE_EVENT),
    ConfigResponse(*const LEAP_CONFIG_RESPONSE_EVENT),
    Image(*const LEAP_IMAGE_EVENT),
    PointMappingChange(*const LEAP_POINT_MAPPING_CHANGE_EVENT),
    LogEvents(*const LEAP_LOG_EVENTS),
    HeadPose(*const LEAP_HEAD_POSE_EVENT),
}

impl From<LEAP_CONNECTION_MESSAGE> for EventType {
    #[allow(non_snake_case)]
    fn from(message: LEAP_CONNECTION_MESSAGE) -> Self {
        unsafe {
            match message.type_ {
                leap_sys::_eLeapEventType_eLeapEventType_Connection => {
                    Self::Connection(message.__bindgen_anon_1.connection_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_ConnectionLost => {
                    Self::ConnectionLost(message.__bindgen_anon_1.connection_lost_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_Device => {
                    Self::Device(message.__bindgen_anon_1.device_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_DeviceLost => {
                    Self::DeviceLost(message.__bindgen_anon_1.device_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_DeviceFailure => {
                    Self::DeviceFailure(message.__bindgen_anon_1.device_failure_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_Tracking => {
                    Self::Tracking(message.__bindgen_anon_1.tracking_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_ImageComplete => Self::ImageComplete,
                leap_sys::_eLeapEventType_eLeapEventType_ImageRequestError => {
                    Self::ImageRequestError
                }
                leap_sys::_eLeapEventType_eLeapEventType_LogEvent => {
                    Self::LogEvent(message.__bindgen_anon_1.log_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_Policy => {
                    Self::Policy(message.__bindgen_anon_1.policy_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_ConfigChange => {
                    Self::ConfigChange(message.__bindgen_anon_1.config_change_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_ConfigResponse => {
                    Self::ConfigResponse(message.__bindgen_anon_1.config_response_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_Image => {
                    Self::Image(message.__bindgen_anon_1.image_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_PointMappingChange => {
                    Self::PointMappingChange(message.__bindgen_anon_1.point_mapping_change_event)
                }
                leap_sys::_eLeapEventType_eLeapEventType_LogEvents => {
                    Self::LogEvents(message.__bindgen_anon_1.log_events)
                }
                leap_sys::_eLeapEventType_eLeapEventType_HeadPose => {
                    Self::HeadPose(message.__bindgen_anon_1.head_pose_event)
                }
                _ => Self::Invalid,
            }
        }
    }
}
