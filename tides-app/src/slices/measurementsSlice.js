import { createSlice } from '@reduxjs/toolkit';

import {
    FETCH_MEASUREMENTS,
    append as appendLoading,
    remove as removeLoading
} from '../slices/loadingSlice';
import {fetchMeasurements as fetchFromApi} from '../utils/fetch';

export const FETCH_STATE = {
    INITIAL: 0,
    FETCHING: 1,
    FETCHED: 2
};

export const measurementsSlice = createSlice({
    name: 'measurements',
    initialState: {
        value: {
            fetching: FETCH_STATE.INITIAL,
            value: []
        },
    },
    reducers: {
        append: (state, action) => {
            state.value.value.push.apply(state.value, action.payload)
        },
        set: (state, action) => { state.value.value = action.payload; },
        fetching: (state, action) => {
            state.value.fetching = action.payload
                ? FETCH_STATE.FETCHING
                : FETCH_STATE.FETCHED;
            },
        clear: state => {
            state.value.fetching = FETCH_STATE.INITIAL;
            state.value.value = [];
        }
    }
});

export const { append, set, fetching, clear } = measurementsSlice.actions;

export const fetchMeasurements = stationId => dispatch => {
    dispatch(fetching(true));
    dispatch(appendLoading({
        key: FETCH_MEASUREMENTS,
        message: "Fetching measurement data and computing predictions."
    }));
    fetchFromApi(stationId)
        .then(measurements => {
            dispatch(set(measurements));
            dispatch(fetching(false));
            dispatch(removeLoading(FETCH_MEASUREMENTS));
        });
};

export const selectMeasurements = state => state.measurements.value;

export default measurementsSlice.reducer;