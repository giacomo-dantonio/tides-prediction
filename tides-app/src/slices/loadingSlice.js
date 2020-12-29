import { createSlice } from '@reduxjs/toolkit';

export const FETCH_MEASUREMENTS = "FETCH_MEASUREMENTS";
export const COMPUTE_PREDICTIONS = "COMPUTE_PREDICTIONS";

export const loadingSlice = createSlice({
    name: 'loading',
    initialState: {
        value: [],
    },
    reducers: {
        clear: (state, action) => {
            state.value = [];
        },
        append: (state, action) => {
            state.value.push(action.payload)
        },
        remove: (state, action) => {
            state.value = state.value.filter(entry => entry.key != action.payload);
        }
    }
});

export const { clear, append, remove } = loadingSlice.actions;

export const select = state => state.loading.value;

export default loadingSlice.reducer;