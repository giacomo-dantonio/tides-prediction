import { createSlice } from '@reduxjs/toolkit';

import fetchFromApi from './fetch';

export const measurementsSlice = createSlice({
    name: 'measurements',
    initialState: {
        value: [],
    },
    reducers: {
        append: (state, action) =>
            state.value.push.apply(state.value, action.payload),
        set: (state, action) => { state.value = action.payload; }
    }
});

export const { append, set } = measurementsSlice.actions;

export const fetchMeasurements = stationId => dispatch => {
    fetchFromApi(stationId)
        .then(measurements => dispatch(set(measurements)));
};

export const selectMeasurements = state => state.measurements.value;

export default measurementsSlice.reducer;