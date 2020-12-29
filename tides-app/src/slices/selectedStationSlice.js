import { createSlice } from '@reduxjs/toolkit';

export const selectedStationSlice = createSlice({
    name: 'selectedStation',
    initialState: {
        // Bremerhaven
        value: "d3f822a0-e201-4a61-8913-589c74818ae0",
    },
    reducers: {
        setStation: (state, action) => { state.value = action.payload; },
    }
});

export const { setStation } = selectedStationSlice.actions;

export const selectStation = state => state.selectedStation.value;

export default selectedStationSlice.reducer;