import React, { useEffect, useState } from 'react';
import Chart from 'chart.js';

export default function ChartComponent(props) {
    const canvasId = props.canvasId || "chart-canvas";
    const canvasLabel = props.label || "Data chart";

    // initialize timeRange to 3 days (in minutes)
    const [timeRange, setTimeRange] = useState(3 * 24 * 60);
    const [chart, setChart] = useState(null);

    const eventCallback = event => {
        if (event.type === "wheel") {
            setTimeRange(timeRange - 10 * event.deltaY);
        }
    };

    // Only create the chart once after first rendering,
    // and update it every time the data change.
    useEffect(() => {
        const options = makeOptions({timeRange, eventCallback});
        const datasets = makeDatasets(props.series);
        const config = {
            type: 'line',
            data: {
                datasets
            },
            options
        };
    
        const ctx = document.getElementById(canvasId).getContext('2d');
        setChart(new Chart(ctx, config));
    }, []);

    useEffect(() => {
        if (chart !== null) {
            chart.options.onHover = eventCallback;
            chart.options.scales.xAxes[0].ticks = makeTicks(timeRange);
            chart.update();
        }
    }, [timeRange]);

    useEffect(() => {
        if (chart !== null) {
            chart.config.data.datasets = makeDatasets(props.series);
            chart.update();
        }
    }, props.series);

    return <div>
        <canvas
            id={canvasId}
            aria-label={canvasLabel}
            role="img"
        />
    </div>;
}

function makeOptions(args) {
    return Object.assign({
        hover: {
            animationDuration: 0
        },
        events: [
            'mousemove',
            'mouseout',
            'click',
            'touchstart',
            'touchmove',
            'wheel'
        ],
        onHover: args.eventCallback,
        scales: {
            xAxes: [{
                type: "time",
                time: {
                    unit: "hour",
                    displayFormats: {
                        hour: "D MMM YYYY H:mm",
                    }
                },
                ticks : makeTicks(args.timeRange)
            }],
            yAxes: [{
                id: "pegel-axis",
                gridLines: {
                    display: false
                }
            }],
        }
    });
}

function makeDatasets(series) {
    const datasets = [];
    for (const dataSeries of (series || [])) {
        const dataset = {
            label: dataSeries.name,
            backgroundColor: dataSeries.color,
            borderColor: dataSeries.color,
            fill: false,
            yAxisID: "pegel-axis",
            data: dataSeries.values.map(entry => ({
                x: new Date(entry.timestamp * 1000),
                y: entry.value
            }))
        };
        datasets.push(dataset);
    }
    return datasets;
}

function makeTicks(timeRange) {
    return {
        min: new Date(Date.now() - timeRange * 60 * 1000),
        max: new Date(Date.now() + timeRange * 60 * 1000),
        sampleSize: 7,
    };
}
