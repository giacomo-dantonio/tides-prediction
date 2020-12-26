import { createSlice } from '@reduxjs/toolkit';

import fetchFromApi from './fetch';

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
    }
});

export const { append, set, fetching } = measurementsSlice.actions;

export const fetchMeasurements = stationId => dispatch => {
    dispatch(fetching(true));
    fetchFromApi(stationId)
        .then(measurements => {
            dispatch(set(measurements));
            dispatch(fetching(false));
        });
};

export const selectMeasurements = state => state.measurements.value;

export default measurementsSlice.reducer;