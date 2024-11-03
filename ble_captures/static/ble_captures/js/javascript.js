
/* this is a test function
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
*/


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

const safeHtmlRenderer = (_instance, td, _row, _col, _prop, value) => { //this allows HTML to be inserted as data in table
  td.innerHTML = value;
};

document.addEventListener('DOMContentLoaded', function(){
  
  const deviceElement = document.getElementById("device-data");
  const devicePk = deviceElement.getAttribute("data-device-pk");

  const endpoint = '/api/fetch-pkt-data/' +devicePk;

  let packets = []; //array will hold packets
  fetch(endpoint)
    .then(response => {
      if(!response.ok){
        throw new Error('Network response was not ok');
      }
      return response.json();
    })
    .then( data => {//response is ok
      //console.log(data);
      for (const [key, packet] of Object.entries(data.packets)){
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
      }
      console.log(packets);
      

      const packetsTable = document.getElementById('packetsTable');

      const hotDT = new Handsontable(packetsTable, {
        data: packets,
        columns: [
          {
            title: 'ID',
            type: 'numeric',
            data: 'id',
          },
          {
            title: 'Advertising Address',
            type: 'text',
            data: 'advertising_address',
          },
          {
            title: 'Power Level',
            type: 'text',
            data: 'power_level',
          },
          {
            title: 'Company ID',
            type: 'text',
            data: 'company_id',
          },
          {
            title: 'Time Stamp',
            type: 'text',
            data: 'time_stamp',
          },
          {
            title: 'rssi',
            type: 'text',
            data: 'rssi',
          },
          {
            title: 'Channel Index',
            type:'text',
            data: "channel_index",
          },
          {
            title: 'Counter',
            type:'text',
            data: "counter",
          },
          {
            title: 'Protocol Version',
            type:'text',
            data: "protocol_version",
          },
          {
            title: 'Malicious',
            type:'text',
            data: "malicious",
          },
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
        afterFilter: function(){
          console.log(hotDT.countRows()) // have this appear on page as filter results
        },
        licenseKey: 'non-commercial-and-evaluation',
      });
    })

})


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
      
      
            //loop through fetch response and insert into table data
            for (const [key, device] of Object.entries(data.devices)) {
              let deviceObj = {};
              deviceObj.id = `${device.id}`;
              deviceObj.name = `${device.name}`;
              deviceObj.OUI = `${device.OUI}`;
              deviceObj.comp_id = `${device.comp_id}`;
              deviceObj.group = `${device.group}`;
              deviceObj.mal = `${device.mal}`;
              deviceObj.btn = "<button id='" + `${device.id}` + "_packet_view'" + "type='button' class='btn btn-primary btn-sm' onclick='packetView("+ `${device.id}` +")'>" + "View Packets"  +"</button>";
              devices.push(deviceObj);
            }
            //console.log(devices);
        
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
            title: 'Name',
            type: 'text',
            data: 'name',
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
            title: 'Group Name',
            type: 'text',
            data: 'group',
          },
          {
            title: 'Malicous Device?',
            type: 'text',
            data: 'mal',
          },
          {
            title: 'View Packets',
            type:'text',
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
        afterFilter: function(){
          console.log(hotDT.countRows()) // have this appear on page as filter results
        },
        licenseKey: 'non-commercial-and-evaluation',
      });

    })
})

function packetView(foo){
  alert(foo);
}
