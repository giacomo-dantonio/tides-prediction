import {clear as clearMeasurements} from '../slices/measurementsSlice';
import {clear as clearPredictions} from '../slices/predictionsSlice';

export function clearAndThen(action) {
    // thunk
    return dispatch => {
        dispatch(clearMeasurements());
        dispatch(clearPredictions());
        dispatch(action);
    }
}