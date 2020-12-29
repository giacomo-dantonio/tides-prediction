import { createSlice } from '@reduxjs/toolkit';

export const predictionsSlice = createSlice({
    name: 'predictions',
    initialState: {
        value: [],
    },
    reducers: {
        batchSet: (state, action) => {
            state.value = action.payload;
        },
        set: (state, action) => {
            state.value.push(action.payload);
        },
        clear: (state, action) => {
            state.value = [];
        }
    }
});

export const { set, batchSet, clear } = predictionsSlice.actions;

export const selectPredictions = state => state.predictions.value;

export default predictionsSlice.reducer;