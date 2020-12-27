import { configureStore } from '@reduxjs/toolkit';
import measurements from '../features/station/measurementsSlice';
import predictions from '../features/station/predictionsSlice';
import selectedStation from '../features/station/selectedStationSlice';
import stations from '../features/panel/stationsSlice';

export default configureStore({
  reducer: {
    measurements,
    predictions,
    stations,
    selectedStation
  },
});
