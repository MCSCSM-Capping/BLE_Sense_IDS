

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

//Current Packets Count line chart
//
var chartDom = document.getElementById('packetCount');
var myChart = echarts.init(chartDom);
var option;

// Function to fetch packet count from API
function fetchPacketCount() {
  return fetch('/api/fetch-pkt-count')
    .then(response => response.json())
    .then(data => data.pkt_count)  
    .catch(error => {
      console.error('Error fetching packet count:', error);
      return 0;  // Return 0 if there was an error
    });
}

// Function to generate new data with timestamp and packet count from API
function pktCount() {
  now = new Date(+now + oneSec);  // Increment time by one second
  return fetchPacketCount().then(packetCount => {
    return {
      name: now.toString(),  // Timestamp as name
      value: [
        now.getTime(),  // Use the timestamp in milliseconds for time axis
        packetCount  // Packet count for Y-axis
      ]
    };
  });
}

// Initialize chart data, current time, and value
let data = [];
let now = new Date();
let oneSec = 1000;  // One second in milliseconds

// Generate initial data
(async function initializeData() {
  for (var i = 0; i < 100; i++) {  // Initialize with 100 points
    let dataPoint = await pktCount();
    data.push(dataPoint);
  }
})();

// Define chart options with hidden xAxis labels and custom tooltip
option = {

  tooltip: {
    trigger: 'axis',
    formatter: function (params) {
      params = params[0];
      var date = new Date(params.value[0]);  // Convert timestamp to date
      var hours = date.getHours().toString().padStart(2, '0');  // Format hours
      var minutes = date.getMinutes().toString().padStart(2, '0');  // Format minutes
      var seconds = date.getSeconds().toString().padStart(2, '0');  // Format seconds
      var time = hours + ':' + minutes + ':' + seconds;
      
      return `Time: ${time} <br/> Packet Count: ${params.value[1]}`;  // Show time and packet count
    },
    axisPointer: {
      animation: false
    }
  },
  xAxis: {
    type: 'time',
    splitLine: {
      show: false
    },
    axisLabel: {
      show: false  // Hide x-axis labels
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
      name: 'Packet Count',
      type: 'line',
      showSymbol: false,
      data: data
    }
  ]
};

// Update chart every 1000 milliseconds (1 second)
setInterval(async function () {
  for (var i = 0; i < 5; i++) {  // Shift out old data and add new data
    data.shift();  // Remove oldest data point
    let newData = await pktCount();
    data.push(newData);  // Add new data from API
  }

  // Update chart with the new data
  myChart.setOption({
    series: [
      {
        data: data
      }
    ]
  });
}, 1000);  // Update interval is 1000 milliseconds (1 second)

option && myChart.setOption(option);





//Ongoing Attacks line chart
document.addEventListener("DOMContentLoaded", () => {
    new Chart(document.querySelector('#ongoingAttacks'), {
        type: 'line',
        data: {
            labels: ['Time 1', '2', '3', '4', '5', '6', '7'],
            datasets: [{
                label: '1',
                data: [200, 250, 120, 113, 209, 267, 300],
                fill: false,
                borderColor: 'rgb(75, 192, 192)',
                tension: 0.1
            },
            {label: '2',
                data: [100, 150, 20, 13, 109, 167, 200],
                fill: false,
                borderColor: 'rgb(235, 232, 61)',
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