import { createSlice } from '@reduxjs/toolkit';

import {fetchStations as fetchFromApi} from '../utils/fetch';

export const FETCH_STATE = {
    INITIAL: 0,
    FETCHING: 1,
    FETCHED: 2
};

export const stationsSlice = createSlice({
    name: 'stations',
    initialState: {
        value: {
            fetching: FETCH_STATE.INITIAL,
            value: []
        },
    },
    reducers: {
        set: (state, action) => { state.value.value = action.payload; },
        fetching: (state, action) => {
            state.value.fetching = action.payload
                ? FETCH_STATE.FETCHING
                : FETCH_STATE.FETCHED;
            },
    }
});

export const { set, fetching } = stationsSlice.actions;

export const fetchStations = () => dispatch => {
    dispatch(fetching(true));
    // Fetch stations in a radius of 120 km from Helgoland.
    // This way we will only get the north sea area.
    fetchFromApi({
        latitude: 54.18,
        longitude: 7.88,
        radius: 120
    })
        .then(stations => {
            dispatch(set(stations));
            dispatch(fetching(false));
        });
};

export const selectStations = state => state.stations.value;

export default stationsSlice.reducer;