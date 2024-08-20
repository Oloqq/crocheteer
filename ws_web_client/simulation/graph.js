export class Graph {
  constructor(id) {
    this.id = id;
    const ctx = document.getElementById(this.id);

    new Chart(ctx, {
      type: 'line',
      data: {
        labels: [1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
        datasets: [{
          label: "bruh",
          data: [1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
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
}