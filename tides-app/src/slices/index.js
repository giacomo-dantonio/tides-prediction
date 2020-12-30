import measurements from './measurementsSlice';
import predictions from './predictionsSlice';
import selectedStation from './selectedStationSlice';
import stations from './stationsSlice';
import extremes from './extremesSlice';
import loading from './loadingSlice';

const reducers = {
    measurements,
    predictions,
    stations,
    selectedStation,
    extremes,
    loading
};

export default reducers;