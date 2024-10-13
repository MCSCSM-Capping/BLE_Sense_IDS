

document.addEventListener('DOMContentLoaded', function () {

    const endpoint = '/api/fetch-data';

    fetch(endpoint)
        .then(response => {
            if (!response.ok) {
                throw new Error('Netowrk response was not ok');
            }
            return response.json();
        })
        .then(data => {
            console.log(data);
        })
        .catch(error => {
            console.error('issue with fetch operation', error);
        });
});

//Ongoing Attacks line chart
//
var chartDom = document.getElementById('ongoingAttacks');
var myChart = echarts.init(chartDom);
var option;

function randomData() {
  now = new Date(+now + oneDay);
  value = value + Math.random() * 21 - 10;
  return {
    name: now.toString(),
    value: [
      [now.getFullYear(), now.getMonth() + 1, now.getDate()].join('/'),
      Math.round(value)
    ]
  };
}
let data = [];
let now = new Date(1997, 9, 3);
let oneDay = 24 * 3600 * 1000;
let value = Math.random() * 1000;
for (var i = 0; i < 1000; i++) {
  data.push(randomData());
}
option = {
  tooltip: {
    trigger: 'axis',
    formatter: function (params) {
      params = params[0];
      var date = new Date(params.name);
      return (
        date.getDate() +
        '/' +
        (date.getMonth() + 1) +
        '/' +
        date.getFullYear() +
        ' : ' +
        params.value[1]
      );
    },
    axisPointer: {
      animation: false
    }
  },
  xAxis: {
    type: 'time',
    splitLine: {
      show: false
    }
  },
  yAxis: {
    type: 'value',
    boundaryGap: [0, '100%'],
    splitLine: {
      show: false
    }
  },
  series: [
    {
      name: 'Fake Data',
      type: 'line',
      showSymbol: false,
      data: data
    }
  ]
};
setInterval(function () {
  for (var i = 0; i < 5; i++) {
    data.shift();
    data.push(randomData());
  }
  myChart.setOption({
    series: [
      {
        data: data
      }
    ]
  });
}, 100);

option && myChart.setOption(option);




/*
document.addEventListener("DOMContentLoaded", () => {
    new Chart(document.querySelector('#ongoingAttacks'), {
        type: 'line',
        data: {
            labels: ['Time 1', '2', '3', '4', '5', '6', '7'],
            datasets: [{
                label: 'Severity level 1',
                data: [65, 59, 80, 81, 56, 55, 40],
                fill: false,
                borderColor: 'rgb(75, 192, 192)',
                tension: 0.1
            },
            {
                label: 'Severity level 2',
                data: [50, 60, 70, 20, 60, 55, 80],
                fill: false,
                borderColor: 'rgb(255, 0, 0)',
                tension: 0.1

            }
            ]
        },
        options: {
            scales: {
                y: {
                    beginAtZero: true
                }
            }
        }
    });
});
*/
//Current Devices line chart
document.addEventListener("DOMContentLoaded", () => {
    new Chart(document.querySelector('#deviceCount'), {
        type: 'line',
        data: {
            labels: ['Time 1', '2', '3', '4', '5', '6', '7'],
            datasets: [{
                label: 'count',
                data: [200, 250, 120, 113, 209, 267, 300],
                fill: false,
                borderColor: 'rgb(75, 192, 192)',
                tension: 0.1
            },
            ]
        },
        options: {
            scales: {
                y: {
                    beginAtZero: true
                }
            }
        }
    });
});

//total Attacks Donut Chart
document.addEventListener("DOMContentLoaded", () => {
    new Chart(document.querySelector('#totalAttacks'), {
        type: 'doughnut',
        data: {
            labels: [
                'Severity level 1',
                '2', '3', '4'
            ],
            datasets: [{
                label: 'Total Attacks',
                data: [300, 50, 100, 200],
                backgroundColor: [
                    'rgb(255, 99, 132)',
                    'rgb(54, 162, 235)',
                    'rgb(255, 205, 86)',
                    'rgb(255, 0, 0)'
                ],
                hoverOffset: 4
            }]
        }
    });
});