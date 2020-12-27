import React from 'react';
import './App.css';
import Station from './features/station';
import StationChooser from './features/panel/StationChooser';

function App() {
  return (
    <div className="App">
      <div className="station-chart">
        <Station stationId="d3f822a0-e201-4a61-8913-589c74818ae0"/>
      </div>
      <div className="side-panel">
        <StationChooser/>
      </div>
    </div>
  );
}

export default App;
