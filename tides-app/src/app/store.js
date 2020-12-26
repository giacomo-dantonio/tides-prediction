import { configureStore } from '@reduxjs/toolkit';
import measurements from '../features/station/measurementsSlice';
import predictions from '../features/station/predictionsSlice';

export default configureStore({
  reducer: {
    measurements,
    predictions
  },
});
