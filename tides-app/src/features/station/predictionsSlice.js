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
        }
    }
});

export const { set, batchSet } = predictionsSlice.actions;

export const selectPredictions = state => state.predictions.value;

export default predictionsSlice.reducer;