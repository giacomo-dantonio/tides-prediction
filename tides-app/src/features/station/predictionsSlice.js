import { createSlice } from '@reduxjs/toolkit';

export const predictionsSlice = createSlice({
    name: 'predictions',
    initialState: {
        value: {},
    },
    reducers: {
        batchSet: (state, action) => {
            for (const entry of action.payload) {
                state.value[entry.timestamp] = entry.value;
            }
        },
        set: (state, action) => {
            const {timestamp, value} = action.payload;
            state.value[timestamp] = value;
        }
    }
});

export const { set, batchSet } = predictionsSlice.actions;

export const selectPredictions = state => state.predictions.value;

export default predictionsSlice.reducer;