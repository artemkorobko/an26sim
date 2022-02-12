use crate::{
    app::handle_params,
    params::{Params, SM2MParamsState},
};

const START_MARKER_PARAM: u16 = 0x5555;

pub fn handle_params(cx: handle_params::Context, param: u16) {
    let state = cx.local.state;
    match state {
        SM2MParamsState::DetectMarker => {
            if is_start_marker(param) {
                *state = SM2MParamsState::DetectParamsCount(0);
            }
        }
        SM2MParamsState::DetectParamsCount(count) => {
            let is_not_start_marker = !is_start_marker(param);
            if is_not_start_marker {
                *count += 1
            } else {
                *state = SM2MParamsState::ReadParams(Params::new(*count))
            }
        }
        SM2MParamsState::WaitForMarker(count) => {
            if is_start_marker(param) {
                *state = SM2MParamsState::ReadParams(Params::new(*count))
            }
        }
        SM2MParamsState::ReadParams(params) => {
            let completed = !params.register(param);
            if completed {
                *state = SM2MParamsState::WaitForMarker(params.count)
            }
        }
    }
}

fn is_start_marker(param: u16) -> bool {
    param == START_MARKER_PARAM
}
