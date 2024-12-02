//alerts dashboard tool
document.addEventListener('DOMContentLoaded', function () {
  function fetchSysStatus() {
    const endpoint = '/api/sys-status';
    fetch(endpoint)
      .then(response => {
        if (!response.ok) {
          throw new Error('Sys-status: Network response was not ok')
        }
        return response.json();
      })
      .then(data => {
        // Get the current time
        const now = new Date();

        // Loop through the data and check timestamps
        data.forEach(item => {
          if (item.latest_timestamp) {
            const timestamp = new Date(item.latest_timestamp);
            const diffSeconds = (now - timestamp) / 1000; // Difference in seconds
            if (diffSeconds > 60) {
              console.log(item.name); // Log the name if timestamp is not within the last 60 seconds
              document.getElementById("alert_" +item.id).innerHTML = "<td><i class='bi bi-dash-circle-fill error-icon'></i> "+ item.name + " is offline since "+item.latest_timestamp +" </td>"
            }
          } else if(item.latest_timestamp == null) {
            document.getElementById("alert_" +item.id).innerHTML = "<td><i class='bi bi-exclamation-diamond-fill warning-icon'></i> "+ item.name + " has not been set up </td>"
          }
          else{
            document.getElementById("alert_" +item.id).innerHTML = "<td><i class='bi bi-check-circle-fill ok-icon'></i> "+ item.name + " is online </td>" 
          }
        });
      })
  }
 fetchSysStatus();
//  setInterval(fetchSysStatus, 10);
});




//dashboard line graph device count by category
//-----------------------------------------
document.addEventListener('DOMContentLoaded', function () {
  var chartDom = document.getElementById('deviceCount');
  var myChart = echarts.init(chartDom);

  // Arrays to store the last 100 data points
  let xAxisData = [];
  let maliciousData = [];
  let nonMaliciousData = [];
  let totalData = [];

  // Fetch function to get device counts
  function fetchDeviceCount() {
    const endpoint = '/api/device-count/';
    fetch(endpoint)
      .then(response => {
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        return response.json();
      })
      .then(data => {
        //console.log(data);

        let now = new Date().toLocaleTimeString(); // Current time for x-axis

        // Append new data
        xAxisData.push(now);
        maliciousData.push(data.malicious_devices);
        nonMaliciousData.push(data.non_malicious_devices);
        totalData.push(data.all_devices);

        // Ensure only the last 100 data points are kept
        if (xAxisData.length > 100) {
          xAxisData.shift();
          maliciousData.shift();
          nonMaliciousData.shift();
          totalData.shift();
        }

        // Update the chart with the latest data
        myChart.setOption({
          xAxis: {
            type: 'category',
            boundaryGap: false,
            data: xAxisData // Use the updated x-axis labels
          },
          yAxis: {
            type: 'value',
            boundaryGap: [0, 0], // Remove extra padding on the y-axis
            minInterval: 1, // Force whole numbers on the y-axis
            splitLine: {
              show: false
            }
          },
          series: [
            {
              name: 'Malicious Devices',
              type: 'line',
              data: maliciousData, // Use the updated malicious data
              color: 'red',
              smooth: true
            },
            {
              name: 'Non-Malicious Devices',
              type: 'line',
              data: nonMaliciousData, // Use the updated non-malicious data
              color: 'green',
              smooth: true
            },
            {
              name: 'Total Devices',
              type: 'line',
              data: totalData, // Use the updated total data
              color: 'blue',
              smooth: true
            }
          ],
          legend: {
            data: ['Malicious Devices', 'Non-Malicious Devices', 'Total Devices'],
            bottom: 'bottom',
            textStyle: {
              color: '#000',
              fontSize: 14
            }
          }
        });
      })
      .catch(error => {
        console.error('Fetch error:', error);
      });
  }

  // Set the initial option for the chart
  const option = {
    tooltip: {
      trigger: 'axis',
      formatter: function (params) {
        let tooltip = params[0].name + '<br/>';
        params.forEach(param => {
          tooltip += `${param.seriesName}: ${param.value}<br/>`;
        });
        return tooltip;
      }
    },
    xAxis: {
      type: 'category',
      boundaryGap: false, // Align data points with the axis ticks
      data: [] // Initialize with no data
    },
    yAxis: {
      type: 'value',
      boundaryGap: [0, 0], // Remove extra padding on the y-axis
      minInterval: 1, // Force whole numbers on the y-axis
      splitLine: {
        show: false
      }
    },
    series: [
      { name: 'Malicious Devices', type: 'line', data: [], color: 'red', smooth: true },
      { name: 'Non-Malicious Devices', type: 'line', data: [], color: 'green', smooth: true },
      { name: 'Total Devices', type: 'line', data: [], color: 'blue', smooth: true }
    ],
    legend: {
      data: ['Malicious Devices', 'Non-Malicious Devices', 'Total Devices'],
      bottom: 'bottom',
      textStyle: {
        color: '#000',
        fontSize: 14
      }
    }
  };

  // Set the initial chart option
  myChart.setOption(option);

  // Fetch data every 60 seconds
  fetchDeviceCount(); // Initial fetch
  setInterval(fetchDeviceCount, 10000);
});
//-----------------------------------------

// Scanner System Metrics
//-----------------------------------------
document.addEventListener('DOMContentLoaded', function () {
  var chartDom = document.getElementById('systemMetrics');
  var myChart = echarts.init(chartDom);

  // Arrays to store the last 100 data points
  let memoryData = [];
  let swapData = [];
  let cpuData = [];
  let timeData = [];

  // Fetch function to get system metrics
  function fetchSystemMetrics(scannerID) {
    const endpoint = `/scanner/${scannerID}/metrics/`;
    fetch(endpoint)
      .then(response => {
        if (!response.ok) {
          document.getElementById('systemMetrics').innerHTML = "<h2>Network Error</h2> Please choose a valid scanner";
          throw new Error('Network response was not ok');
        }
        return response.json();
      })
      .then(data => {
        console.log(data);
        if (data.error) {
          document.getElementById('systemMetrics').innerHTML = "<h2>ERROR: THIS SENSOR IS OFFLINE.</h2> The last heartbeat was detected at " + data.time + ".";
        }
        else {
          let now = new Date(); // Current time for x-axis

          // Add the new data point to the respective arrays
          timeData.push(now.toLocaleTimeString());
          memoryData.push(data.mem_perc);
          swapData.push(data.swap_perc);
          cpuData.push(data.total_cpu);

          // Retain only the last 100 data points
          if (timeData.length > 100) {
            timeData.shift();
            memoryData.shift();
            swapData.shift();
            cpuData.shift();
          }

          // Update the chart with the latest data
          myChart.setOption({
            tooltip: {
              trigger: 'axis',
              formatter: function (params) {
                let tooltip = params[0].name + '<br/>';
                params.forEach(param => {
                  tooltip += `${param.seriesName}: ${param.value}%<br/>`;
                });
                return tooltip;
              }
            },
            xAxis: {
              type: 'category',
              boundaryGap: false,
              data: timeData // Use the updated time data
            },
            yAxis: {
              type: 'value',
              boundaryGap: [0, '100%'],
              minInterval: 1,
              splitLine: {
                show: false
              },
              axisLabel: {
                formatter: '{value}%' // Show percentage on the y-axis
              }
            },
            series: [
              {
                name: 'Memory Usage (%)',
                type: 'line',
                data: memoryData, // Use the updated memory data
                color: 'green',
                smooth: true
              },
              {
                name: 'Swap Usage (%)',
                type: 'line',
                data: swapData, // Use the updated swap data
                color: 'orange',
                smooth: true
              },
              {
                name: 'CPU Usage (%)',
                type: 'line',
                data: cpuData, // Use the updated CPU data
                color: 'blue',
                smooth: true
              }
            ],
            legend: {
              data: ['Memory Usage (%)', 'Swap Usage (%)', 'CPU Usage (%)'],
              bottom: 'bottom',
              textStyle: {
                color: '#000',
                fontSize: 14
              }
            }
          });
        }
      })
      .catch(error => {
        console.error('Fetch error:', error);
      });
  }

  // Set the initial option for the chart
  const option = {
    tooltip: {
      trigger: 'axis',
      formatter: function (params) {
        let tooltip = params[0].name + '<br/>';
        params.forEach(param => {
          tooltip += `${param.seriesName}: ${param.value}%<br/>`;
        });
        return tooltip;
      }
    },
    xAxis: {
      type: 'category',
      boundaryGap: false,
      splitLine: {
        show: false
      }
    },
    yAxis: {
      type: 'value',
      boundaryGap: [0, '100%'],
      minInterval: 1,
      splitLine: {
        show: false
      },
      axisLabel: {
        formatter: '{value}%' // Display percentage on y-axis
      }
    },
    series: [],
    legend: {
      data: ['Memory Usage (%)', 'Swap Usage (%)', 'CPU Usage (%)'],
      bottom: 'bottom',
      textStyle: {
        color: '#000',
        fontSize: 14
      }
    }
  };

  // Set the initial chart option
  myChart.setOption(option);

  // Fetch metrics when the user changes the scanner selection
  const scannerSelect = document.getElementById("scannerSelect");
  scannerSelect.addEventListener('change', function () {
    const scannerID = scannerSelect.options[scannerSelect.selectedIndex].value;
    // Clear data when scanner changes
    memoryData = [];
    swapData = [];
    cpuData = [];
    timeData = [];
    fetchSystemMetrics(scannerID); // Fetch data for the new scanner
  });

  // Initial fetch for the default scanner
  const initialScannerID = scannerSelect.options[scannerSelect.selectedIndex].value;
  fetchSystemMetrics(initialScannerID);

  // Periodically fetch data for the currently selected scanner
  setInterval(() => {
    const scannerID = scannerSelect.options[scannerSelect.selectedIndex].value;
    fetchSystemMetrics(scannerID);
  }, 10000); // Update every 10 seconds
});
//-----------------------------------------





//allow HTML to be inserted as data in table
const safeHtmlRenderer = (_instance, td, _row, _col, _prop, value) => {
  td.innerHTML = value;
};


//packets table
//-----------------------------------------
document.addEventListener('DOMContentLoaded', function () {

  const deviceElement = document.getElementById("device-data");
  const devicePk = deviceElement.getAttribute("data-device-pk");

  const endpoint = '/api/fetch-pkt-data/' + devicePk;

  let packets = []; // array will hold packets
  let currentPage = 1;  //track the current page**

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

        //console.log(packets);

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
            afterFilter: function () {
              document.getElementById("rowDisplay").innerHTML = "Rows Displayed: " + window.hotDT.countRows();
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
    document.getElementById("rowDisplay").innerHTML = "Rows Displayed: " + window.hotDT.countRows();
    //this is called on page load and after each pagination
    //this function cannot be called on afterFilter tag so if updated here it must also be updated there
  }
});
//-----------------------------------------

//all devices table
//-----------------------------------------
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
      /**
      console.log(data)
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
//-----------------------------------------

//Donut chart and vulnerable devices table
//-----------------------------------------
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

  // Add event listeners for quick date buttons
  document.getElementById('last7').addEventListener('click', () => {
    setDateRange('last7');
    fetchDataAndUpdateChart();
  });
  document.getElementById('last30').addEventListener('click', () => {
    setDateRange('last30');
    fetchDataAndUpdateChart();
  });

  document.getElementById('ytd').addEventListener('click', () => {
    setDateRange('ytd');
    fetchDataAndUpdateChart();
  });
  //date range set to year to date on page load
  document.addEventListener("DOMContentLoaded", setDateRange('ytd'));
});


// Initialize donut chart and vulnerable table on page load
document.addEventListener("DOMContentLoaded", fetchDataAndUpdateChart);

// Update chart on button click
document.getElementById("updateButton").addEventListener("click", fetchDataAndUpdateChart);

//end donut chart and vulnerable groups table
//-----------------------------------------

