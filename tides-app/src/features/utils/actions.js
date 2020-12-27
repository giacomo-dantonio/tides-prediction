import {clear as clearMeasurements} from '../station/measurementsSlice';
import {clear as clearPredictions} from '../station/predictionsSlice';

export function clearAndThen(action) {
    // thunk
    return dispatch => {
        dispatch(clearMeasurements());
        dispatch(clearPredictions());
        dispatch(action);
    }
}