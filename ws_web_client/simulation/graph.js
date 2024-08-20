const steps = 20;

export class Graph {
  constructor(id) {
    this.id = id;
    this.x = 0;
    const ctx = document.getElementById(this.id);

    this.chart = new Chart(ctx, {
      type: 'line',
      data: {
        labels: [],
        datasets: [{
          label: "tension",
          data: [],
          borderWidth: 1
        }, {
          label: `other metric ${steps}`,
          data: [],
          borderWidth: 1,
        }
        ]
        // labels: ['Red', 'Blue', 'Yellow', 'Green', 'Purple', 'Orange'],
        // datasets: [{
        //   label: '# of Votes',
        //   data: [12, 19, 3, 5, 2, 3],
        //   borderWidth: 1
        // }]
      },
      options: {
        scales: {
          y: {
            beginAtZero: true
          }
        },
        animation: {
          duration: 0,
        }
      },
    });
  }

  update(newValue) {
    const data = this.chart.config.data;
    const tensions = data.datasets[0].data;
    const avgs = data.datasets[1].data;
    const xAxis = data.labels;
    xAxis.push(++this.x);
    tensions.push(newValue);
    avgs.push(avgOverSteps(tensions));
    // if (xAxis.length > 200) {
    //   xAxis.shift();
    //   tensions.shift();
    // }
    this.chart.update();
  }
}

function avgOverSteps(arr) {
  if (arr.length < steps) {
    return 0;
  }

  let slice = [];
  for (let i = 1; i <= steps; ++i) {
    slice.push(arr[arr.length - i]);
  }
  const min = Math.min(...slice);
  const max = Math.max(...slice);
  return max - min;
}