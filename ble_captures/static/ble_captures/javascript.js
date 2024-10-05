function staticTest(){
    alert("the static folder is set up properly");
}

document.getElementById("test").addEventListener("click",staticTest);

document.addEventListener('DOMContentLoaded', function(){

    const endpoint = '/api/fetch-data';

    fetch(endpoint)
        .then(response => {
            if(!response.ok){
                throw new Error('Netowrk response was not ok');
            }
            return response.json();
        })
        .then(data => {
            console.log(data);
        })
        .catch(error => {
            console.error('issue with fetch operation',error);
        });
});

/*
const labels = ['Monaday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'];
    const data = {
        labels: labels,
        datasets: [{
            label: 'Attacks by day',
            data: [1, 2, 3,4,5,6,7],
            fill: false,
            borderColor: 'rgb(75,192,192)',
            tension: 0.1
        }]
    }
     

    var ctx = document.getElementById('attacksByDate').getContext('2d');
    var myChart = new Chart(ctx,{
        type: 'line',
        data: data,
    });
 */   