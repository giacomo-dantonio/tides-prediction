import React, { useEffect } from 'react';
import Chart, {chartColors} from 'chart.js';

export default function ChartComponent(props) {
    const canvasId = props.canvasId || "chart-canvas";
    const canvasLabel = props.label || "Data chart";

    const options = makeOptions(props);
    const datasets = makeDatasets(props);
    const config = {
        type: 'line',
        data: {
            datasets
        },
        options
    };

    // FIXME: only create the chart once after first rendering,
    // and update it every time the data change.
    useEffect(() => {
        const ctx = document.getElementById(canvasId).getContext('2d');
        new Chart(ctx, config);
    });

    return <div>
        <canvas
            id={canvasId}
            aria-label={canvasLabel}
            role="img"
        />
    </div>;
}

function makeOptions(props) {
    return Object.assign({
        scales: {
            xAxes: [{
                type: 'time',
                time: {
                    unit: 'hour',
                    displayFormats: {
                        hour: 'D MMM YYYY H:mm'
                    }
                }
            }]
        }
    },
        props.options);
}

function makeDatasets(props) {
    const datasets = [];
    for (const dataSeries of (props.series || [])) {
        const dataset = {
            label: dataSeries.name,
            backgroundColor: dataSeries.color,
            borderColor: dataSeries.color,
            fill: false,
            data: dataSeries.values.map(entry => ({
                x: new Date(entry.timestamp * 1000),
                y: entry.value
            }))
        };
        datasets.push(dataset);
    }
    return datasets;
}
