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
        }]
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
    const xAxis = data.labels;
    xAxis.push(++this.x);
    tensions.push(newValue);
    // if (xAxis.length > 200) {
    //   xAxis.shift();
    //   tensions.shift();
    // }
    this.chart.update();
  }
}