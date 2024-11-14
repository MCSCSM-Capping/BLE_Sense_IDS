

/*--
//Current Packets Count line chart
// This graph shows the overall count of all packets
//I need to change this to show packets at a specific time stamp as malicious or non-malicious
var pktCountChartDom = document.getElementById('packetCount');
var pktCountChart = echarts.init(pktCountChartDom);
var option;
//^This is where graphic is assigned to DOM element

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
pktCountChart.setOption({
  series: [
    {
      data: data
    }
  ]
});
}, 1000);  // Update interval is 1000 milliseconds (1 second)

option && pktCountChart.setOption(option);

----------------------------------------------*/
/*------------------------------------------------------------*/





/*--
//Current Packets Count line chart
// This graph shows the overall count of all packets
//I need to change this to show packets at a specific time stamp as malicious or non-malicious
var pktCountChartDom = document.getElementById('packetCount');
var pktCountChart = echarts.init(pktCountChartDom);
var option;
//^This is where graphic is assigned to DOM element

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
pktCountChart.setOption({
  series: [
    {
      data: data
      }
      ]
      });
      }, 1000);  // Update interval is 1000 milliseconds (1 second)
      
      option && pktCountChart.setOption(option);
      
      ----------------------------------------------*/
/*------------------------------------------------------------*/

//allow HTML to be inserted as data in table
const safeHtmlRenderer = (_instance, td, _row, _col, _prop, value) => {
  td.innerHTML = value;
};


//packets by Device table
document.addEventListener('DOMContentLoaded', function () {

  const deviceElement = document.getElementById("device-data");
  const devicePk = deviceElement.getAttribute("data-device-pk");

  const endpoint = '/api/fetch-pkt-data/' + devicePk;

  let packets = []; // array will hold packets
  let currentPage = 1;  // New variable to track the current page**

  // Function to fetch packet data with pagination
  function loadPacketData(page) {
    fetch(`${endpoint}?page=${page}`)
      .then(response => {
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        return response.json();
      })
      .then(data => { // response is ok
        // Append fetched packets to packets array

        data.packets.forEach(packet => {
          let packetObj = {};
          packetObj.id = `${packet.pk}`;
          packetObj.advertising_address = `${packet.advertising_address}`;
          packetObj.power_level = `${packet.power_level}`;
          packetObj.company_id = `${packet.company_id}`;
          packetObj.time_stamp = `${packet.time_stamp}`;
          packetObj.rssi = `${packet.rssi}`;
          packetObj.channel_index = `${packet.channel_index}`;
          packetObj.counter = `${packet.counter}`;
          packetObj.protocol_version = `${packet.protocol_version}`;
          packetObj.malicious = `${packet.malicious}`;
          packets.push(packetObj);
        });

        console.log(packets);

        // If Handsontable instance already exists, update data
        const packetsTable = document.getElementById('packetsTable');
        if (window.hotDT) {
          window.hotDT.loadData(packets);  // Load the accumulated packet data
          countRows();
        } else {
          // Create Handsontable instance if it doesn't exist
          document.getElementById("packetHead").innerHTML += "<h2> Packets for Device ID " + devicePk + "</h2>";
          window.hotDT = new Handsontable(packetsTable, {
            data: packets,
            columns: [
              { title: 'Packet ID', type: 'numeric', data: 'id' },
              { title: 'Time Stamp', type: 'text', data: 'time_stamp' },
              { title: 'Advertising Address', type: 'text', data: 'advertising_address' },
              { title: 'Power Level', type: 'text', data: 'power_level' },
              { title: 'Company ID', type: 'text', data: 'company_id' },
              { title: 'rssi', type: 'text', data: 'rssi' },
              { title: 'Channel Index', type: 'text', data: "channel_index" },
              { title: 'Counter', type: 'text', data: "counter" },
              { title: 'Protocol Version', type: 'text', data: "protocol_version" },
              { title: 'Malicious', type: 'text', data: "malicious" }
            ],
            filters: true,
            dropdownMenu: ['filter_by_condition', 'filter_by_value', 'filter_action_bar'],
            height: 'auto',
            autoWrapRow: true,
            autoWrapCol: true,
            readOnly: true,
            stretchH: 'all',
            width: '100%',
            afterFilter: function(){
              document.getElementById("rowDisplay").innerHTML = "Rows Displayed: " + window.hotDT.countRows(); // have this appear on page as filter results
            },
            licenseKey: 'non-commercial-and-evaluation',
          });
          countRows();
          
          
        }

        // Check if more packets are available and display "Load More" button if so
        const loadMoreButton = document.getElementById('loadMoreButton');
        if (data.has_more_packets) {
          loadMoreButton.style.display = 'block';
          currentPage = data.next_page; // Update currentPage for the next fetch
        } else {
          loadMoreButton.style.display = 'none'; // Hide button if no more packets
        }
      })
      .catch(error => {
        console.error('Error fetching packet data:', error);
      });
  }

 

  // Initial data load
  loadPacketData(currentPage);

  // Event listener for the "Load More" button
  document.getElementById('loadMoreButton').addEventListener('click', function () {
    loadPacketData(currentPage); // Load the next page of data
    
  });

  function countRows() {
    document.getElementById("rowDisplay").innerHTML = "Rows Displayed: " + window.hotDT.countRows(); // have this appear on page as filter results
  
  }
});


//all devices table
document.addEventListener('DOMContentLoaded', function () { //is page loaded?

  //data that will be directly inserted into table
  let devices = [];

  const endpoint = '/api/fetch-devices';
  fetch(endpoint) //get data for all devices
    .then(response => {
      if (!response.ok) {
        throw new Error('Netowrk response was not ok');
      }
      return response.json();
    })
    .then(data => { //response is ok 
      console.log(data)
      /**
       * fetch object structure
       * device_data = {
      "id": device.id,
      "name": device.name,
      "oui": device.oui,
      "company_id": latest_packet.company_id,
      "time_stamp": latest_packet.time_stamp,
      "scanner name": scan.scanner.name if scan else None,
      "group": scan.scanner.group.name if scan else None,
      "malicious": has_malicious_packet
      }
      */

      //loop through fetch response and insert into table data
      data.forEach(device => {

        let deviceObj = {};
        deviceObj.id = `${device.id}`;
        deviceObj.OUI = `${device.oui}`;
        deviceObj.comp_id = `${device.company_id}`;
        deviceObj.btn = `<a href="/packets/${device.id}"> View Packets </a>`;
        deviceObj.time = `${device.time_stamp}`
        deviceObj.scanner = `${device.scanner_name}`
        deviceObj.group = `${device.group}`
        deviceObj.malicious = `${device.malicious}`
        devices.push(deviceObj);
      })

      const deviceTable = document.getElementById('deviceTable');

      const hotDT = new Handsontable(deviceTable, {
        data: devices,
        columns: [
          {
            title: 'ID',
            type: 'numeric',
            data: 'id',
          },
          {
            title: 'Malicious?',
            type: 'text',
            data: 'malicious'
          },
          {
            title: 'OUI',
            type: 'text',
            data: 'OUI',
          },
          {
            title: 'Company ID',
            type: 'text',
            data: 'comp_id',
          },
          {
            title: 'Scanned by',
            type: 'text',
            data: 'scanner',
          },
          {
            title: 'Last seen at',
            type: 'text',
            data: 'group',
          },
          {
            title: 'Last detected at',
            type: 'text',
            data: 'time'
          },
          {
            title: 'View Packets',
            type: 'text',
            data: "btn",
            renderer: safeHtmlRenderer,
          }
        ],
        // enable filtering
        filters: true,
        // enable the column menu
        dropdownMenu: ['filter_by_condition', 'filter_by_value', 'filter_action_bar'],
        height: 'auto',
        autoWrapRow: true,
        autoWrapCol: true,
        readOnly: true,
        stretchH: 'all',
        width: '100%',
        afterFilter: function () {
          console.log(hotDT.countRows()) // have this appear on page as filter results
        },
        licenseKey: 'non-commercial-and-evaluation',
      });
    })
})

let donutChart;
let hotDT; // Store the Handsontable instance

function fetchDataAndUpdateChart() {
  const startDate = document.getElementById("startDate").value;
  const endDate = document.getElementById("endDate").value;

  const endpoint = `/api/device-stats/?start_date=${startDate}&end_date=${endDate}`;
  fetch(endpoint)
    .then(response => {
      if (!response.ok) {
        throw new Error('Network response was not ok');
      }
      return response.json();
    })
    .then(data => {
      // Update donut chart
      if (donutChart) {
        donutChart.data.datasets[0].data = [
          data.malicious_devices,
          data.non_malicious_devices
        ];
        donutChart.update();
      } else {
        donutChart = new Chart(document.querySelector('#totalAttacks'), {
          type: 'doughnut',
          data: {
            labels: [
              'Malicious Devices',
              'Non-malicious Devices'
            ],
            datasets: [{
              label: 'Total Attacks',
              data: [
                data.malicious_devices,
                data.non_malicious_devices
              ],
              backgroundColor: [
                'rgb(255, 99, 132)',
                'rgb(54, 162, 235)',
              ],
              hoverOffset: 4
            }]
          }
        });
      }

      // Update Handsontable
      if (hotDT) {
        // Update the data in the table
        hotDT.loadData(data.malicious_by_group);
      } else {
        // Initialize Handsontable if not already initialized
        const vulnLocation = document.getElementById("vulnerableLocations");
        hotDT = new Handsontable(vulnLocation, {
          data: data.malicious_by_group,
          columns: [
            {
              title: 'Group Name',
              type: 'text',
              data: 'name'
            },
            {
              title: 'Number of Malicious Devices',
              type: 'numeric',
              data: 'malicious_device_count'
            }
          ],
          height: 'auto',
          className: 'customFilterButtonExample1',
          autoWrapRow: true,
          autoWrapCol: true,
          readOnly: true,
          stretchH: 'all',
          width: '100%',
          licenseKey: 'non-commercial-and-evaluation',
        });

      }
    })
    .catch(error => {
      console.error('Error fetching data:', error);
    });
}

//date range quick select
document.addEventListener('DOMContentLoaded', function () {
  const startDateInput = document.getElementById('startDate');
  const endDateInput = document.getElementById('endDate');

  // Function to handle date range logic
  function setDateRange(rangeType) {
    let startDate, endDate;
    const today = new Date();
    const currentYear = today.getFullYear();

    switch (rangeType) {
      case 'last30':
        // Last 30 days
        endDate = today.toISOString().split('T')[0]; // Get today's date
        startDate = new Date(today);
        startDate.setDate(today.getDate() - 30);
        startDate = startDate.toISOString().split('T')[0]; // 30 days ago
        break;

      case 'ytd':
        // Year to Date
        startDate = new Date(currentYear, 0, 1).toISOString().split('T')[0]; // January 1st of current year
        endDate = today.toISOString().split('T')[0]; // Today's date
        break;

      case 'last7':
        // Last 7 days
        endDate = today.toISOString().split('T')[0]; // Get today's date
        startDate = new Date(today);
        startDate.setDate(today.getDate() - 7);
        startDate = startDate.toISOString().split('T')[0]; // 7 days ago
        break;
    }

    // Set the date values in the inputs
    startDateInput.value = startDate;
    endDateInput.value = endDate;
  }

  // Add event listeners for the buttons
  document.getElementById('last7').addEventListener('click', function () {
    setDateRange('last7');
  });
  document.getElementById('last30').addEventListener('click', function () {
    setDateRange('last30');
  });

  document.getElementById('ytd').addEventListener('click', function () {
    setDateRange('ytd');
  });
  //date range set to year to date on page load
  document.addEventListener("DOMContentLoaded", setDateRange('ytd'));
});


// Initialize chart on page load
document.addEventListener("DOMContentLoaded", fetchDataAndUpdateChart);

// Update chart on button click
document.getElementById("updateButton").addEventListener("click", fetchDataAndUpdateChart);


