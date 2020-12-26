import { configureStore } from '@reduxjs/toolkit';
import measurementsReducer from '../features/station/stationSlice';

export default configureStore({
  reducer: {
    measurements: measurementsReducer,
  },
});
