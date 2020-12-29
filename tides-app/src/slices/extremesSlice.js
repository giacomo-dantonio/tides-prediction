import { createSlice } from '@reduxjs/toolkit';

export const extremesSlice = createSlice({
    name: 'extremes',
    initialState: {
        value: {
            center: Date.now(),
            minima: [],
            maxima: []
        },
    },
    reducers: {
        set: (state, action) => {
            state.value = action.payload;
        },
        setCenter: (state, action) => {
            state.value = {
                center: action.payload,
                minima: [],
                maxima: []
            }
        }
    }
});

export const { set, setCenter } = extremesSlice.actions;

export const select = state => state.extremes.value;

export default extremesSlice.reducer;