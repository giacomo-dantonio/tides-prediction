import { configureStore } from '@reduxjs/toolkit';
import measurements from '../features/station/measurementsSlice';

export default configureStore({
  reducer: {
    measurements,
  },
});
